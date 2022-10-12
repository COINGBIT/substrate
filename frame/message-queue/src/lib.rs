// Copyright 2020-2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Pallet to handle XCM message queuing.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;
mod weights;

use codec::{Codec, Decode, Encode, FullCodec, MaxEncodedLen};
use frame_support::{pallet_prelude::*, BoundedSlice};
use frame_system::pallet_prelude::*;
pub use pallet::*;
use scale_info::TypeInfo;
use sp_arithmetic::traits::{BaseArithmetic, Unsigned};
use sp_runtime::{
	traits::{Hash, One, Zero},
	SaturatedConversion, Saturating,
};
use sp_std::{prelude::*, vec};
pub use weights::WeightInfo;

/// Type for identifying an overweight message.
type OverweightIndex = u64;
/// Type for identifying a page.
type PageIndex = u32;

/// Data encoded and prefixed to the encoded `MessageItem`.
#[derive(Encode, Decode, MaxEncodedLen)]
pub struct ItemHeader<Size> {
	/// The length of this item, not including the size of this header. The next item of the page
	/// follows immediately after the payload of this item.
	payload_len: Size,
	/// Whether this item has been processed.
	is_processed: bool,
}

/// A page of messages. Pages always contain at least one item.
#[derive(Clone, Encode, Decode, RuntimeDebug, Default, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(HeapSize))]
#[codec(mel_bound(Size: MaxEncodedLen))]
pub struct Page<Size: Into<u32>, HeapSize: Get<Size>> {
	/// Messages remaining to be processed; this includes overweight messages which have been
	/// skipped.
	remaining: Size,
	/// The heap-offset of the header of the first message item in this page which is ready for
	/// processing.
	first: Size,
	/// The heap-offset of the header of the last message item in this page.
	last: Size,
	/// The heap. If `self.offset == self.heap.len()` then the page is empty and should be deleted.
	heap: BoundedVec<u8, IntoU32<HeapSize, Size>>,
}

impl<
		Size: BaseArithmetic + Unsigned + Copy + Into<u32> + Codec + MaxEncodedLen,
		HeapSize: Get<Size>,
	> Page<Size, HeapSize>
{
	fn from_message(message: &[u8], origin: &[u8]) -> Self {
		let len = ItemHeader::<Size>::max_encoded_len() + origin.len() + message.len();
		let mut heap = Vec::with_capacity(len);
		let payload_len: Size = (origin.len() + message.len()).saturated_into(); // TODO: bounded inputs for safety
		let h = ItemHeader { payload_len, is_processed: false };
		h.using_encoded(|d| heap.extend_from_slice(d));
		heap.extend_from_slice(origin);
		heap.extend_from_slice(message);
		Page {
			remaining: One::one(),
			first: Zero::zero(),
			last: Zero::zero(),
			heap: BoundedVec::truncate_from(heap),
		}
	}

	fn try_append_message(&mut self, message: &[u8], origin: &[u8]) -> Result<(), ()> {
		let pos = self.heap.len();
		let len = (ItemHeader::<Size>::max_encoded_len() + origin.len() + message.len()) as u32;
		let payload_len: Size = (origin.len() + message.len()).saturated_into(); // TODO: bounded inputs for safety
		let h = ItemHeader { payload_len, is_processed: false };
		let heap_size: u32 = HeapSize::get().into();
		if heap_size.saturating_sub(self.heap.len() as u32) < len {
			// Can't fit.
			return Err(())
		}

		let mut heap = sp_std::mem::replace(&mut self.heap, Default::default()).into_inner();
		h.using_encoded(|d| heap.extend_from_slice(d));
		heap.extend_from_slice(origin);
		heap.extend_from_slice(message);
		debug_assert!(heap.len() as u32 <= HeapSize::get().into(), "already checked size; qed");
		self.heap = BoundedVec::truncate_from(heap);
		self.last = pos.saturated_into();
		self.remaining.saturating_inc();
		Ok(())
	}

	fn peek_first(&self) -> Option<BoundedSlice<u8, IntoU32<HeapSize, Size>>> {
		if self.first > self.last {
			return None
		}
		let mut item_slice = &self.heap[(self.first.into() as usize)..];
		if let Ok(h) = ItemHeader::<Size>::decode(&mut item_slice) {
			let payload_len = h.payload_len.into();
			if payload_len <= item_slice.len() as u32 {
				// impossible to truncate since is sliced up from `self.heap: BoundedVec<u8,
				// HeapSize>`
				return Some(BoundedSlice::truncate_from(&item_slice[..(payload_len as usize)]))
			}
		}
		debug_assert!(false, "message-queue: heap corruption");
		None
	}

	/// Point `first` at the next message, marking the first as processed if `is_processed` is true.
	fn skip_first(&mut self, is_processed: bool) {
		let f = self.first.into() as usize;
		if let Ok(mut h) = ItemHeader::decode(&mut &self.heap[f..]) {
			if is_processed && !h.is_processed {
				h.is_processed = true;
				h.using_encoded(|d| self.heap[f..f + d.len()].copy_from_slice(d));
				self.remaining.saturating_dec();
			}
			self.first
				.saturating_accrue(ItemHeader::<Size>::max_encoded_len().saturated_into());
			self.first.saturating_accrue(h.payload_len);
		}
	}

	fn is_complete(&self) -> bool {
		self.remaining.is_zero()
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Encode, Decode, TypeInfo, Debug)]
pub enum ProcessMessageError {
	/// The message data format is unknown (e.g. unrecognised header)
	BadFormat,
	/// The message data is bad (e.g. decoding returns an error).
	Corrupt,
	/// The message format is unsupported (e.g. old XCM version).
	Unsupported,
	/// Message processing was not attempted because it was not certain that the weight limit
	/// would be respected. The parameter gives the maximum weight which the message could take
	/// to process.
	Overweight(Weight),
}

pub trait ProcessMessage {
	/// The transport from where a message originates.
	type Origin: FullCodec
		+ MaxEncodedLen
		+ Clone
		+ Eq
		+ PartialEq
		+ TypeInfo
		+ sp_std::fmt::Debug
		+ Arity;

	/// Process the given message, using no more than `weight_limit` in weight to do so.
	fn process_message(
		message: &[u8],
		origin: Self::Origin,
		weight_limit: Weight,
	) -> Result<(bool, Weight), ProcessMessageError>;
}

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo, Default)]
pub struct BookState {
	/// The first page with some items to be processed in it. If this is `>= end`, then there are
	/// no pages with items to be processing in them.
	begin: PageIndex,
	/// One more than the last page with some items to be processed in it.
	end: PageIndex,
	/// The number of pages stored at present.
	count: PageIndex,
	/// The earliest page still in storage. If this is `>= end`, then there are
	/// no pages in storage. Pages up to `head` are reapable if they have a `remaining`
	/// field of zero or if `begin - page_number` is sufficiently large compared to
	/// `count - (end - begin)`.
	/// NOTE: This currently doesn't really work and will become "holey" when pages are removed
	/// out of order. This can be maintained "automatically" with a doubly-linked-list or
	/// "manually" by having a transaction to bump it. However, it probably doesn't actually matter
	/// anyway since reaping can happen perfectly well without it.
	earliest: PageIndex,
}

/// Type which has a finite and small total possible number of values.
pub trait Arity {
	/// Get the total number of distinct values `Self` can take.
	fn arity() -> usize;
}
impl Arity for sp_std::convert::Infallible {
	fn arity() -> usize {
		0
	}
}
impl Arity for () {
	fn arity() -> usize {
		1
	}
}
impl Arity for bool {
	fn arity() -> usize {
		2
	}
}
impl Arity for u8 {
	fn arity() -> usize {
		256
	}
}
impl Arity for u16 {
	fn arity() -> usize {
		65536
	}
}
impl Arity for i8 {
	fn arity() -> usize {
		256
	}
}
impl Arity for i16 {
	fn arity() -> usize {
		65536
	}
}

pub struct GetArity<T>(sp_std::marker::PhantomData<T>);
impl<T: Arity> Get<u32> for GetArity<T> {
	fn get() -> u32 {
		T::arity() as u32
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	/// The module configuration trait.
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		/// Processor for a message.
		type MessageProcessor: ProcessMessage;

		/// Page/heap size type.
		type Size: BaseArithmetic
			+ Unsigned
			+ Copy
			+ Into<u32>
			+ Member
			+ Encode
			+ Decode
			+ MaxEncodedLen
			+ TypeInfo;

		/// The size of the page; this implies the maximum message size which can be sent.
		#[pallet::constant]
		type HeapSize: Get<Self::Size>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Message discarded due to an inability to decode the item. Usually caused by state
		/// corruption.
		Discarded { hash: T::Hash },
		/// Message discarded due to an error in the `MessageProcessor` (usually a format error).
		ProcessingFailed { hash: T::Hash, origin: MessageOriginOf<T>, error: ProcessMessageError },
		/// Message is processed.
		Processed { hash: T::Hash, origin: MessageOriginOf<T>, weight_used: Weight, success: bool },
		/// Message placed in overweight queue.
		Overweight { hash: T::Hash, origin: MessageOriginOf<T>, index: OverweightIndex },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Dummy.
		Dummy,
	}

	/// The index of the first and last (non-empty) pages.
	#[pallet::storage]
	pub(super) type BookStateOf<T: Config> =
		StorageMap<_, Twox64Concat, MessageOriginOf<T>, BookState, ValueQuery>;

	#[pallet::storage]
	pub(super) type ReadyBooks<T: Config> =
		StorageValue<_, BoundedVec<MessageOriginOf<T>, GetArity<MessageOriginOf<T>>>, ValueQuery>;

	/// The map of page indices to pages.
	#[pallet::storage]
	pub(super) type Pages<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		MessageOriginOf<T>,
		Twox64Concat,
		PageIndex,
		Page<T::Size, T::HeapSize>,
		OptionQuery,
	>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
			let weight_used = Weight::zero();
			weight_used
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn send(origin: OriginFor<T>) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	fn do_enqueue_message(
		origin: &MessageOriginOf<T>,
		message: BoundedSlice<u8, MaxMessageLenOf<T>>,
		origin_data: BoundedSlice<u8, MaxOriginLenOf<T>>,
	) {
		let mut book_state = BookStateOf::<T>::get(origin);
		let was_empty = if book_state.end > book_state.begin {
			// Already have a page in progress - attempt to append.
			let last = book_state.end - 1;
			let mut page = match Pages::<T>::get(origin, last) {
				Some(p) => p,
				None => {
					debug_assert!(false, "Corruption: referenced page doesn't exist.");
					return
				},
			};
			if let Ok(_) = page.try_append_message(&message[..], &origin_data[..]) {
				Pages::<T>::insert(origin, last, &page);
				return
			}
			false
		} else {
			true
		};
		// No room on the page or no page - link in a new page.
		book_state.end.saturating_inc();
		Pages::<T>::insert(
			origin,
			book_state.end - 1,
			Page::from_message(&message[..], &origin_data[..]),
		);
		BookStateOf::<T>::insert(origin, book_state);
		if was_empty {
			ReadyBooks::<T>::mutate(|b| {
				let is_ok = b.try_push(origin.clone()).is_ok();
				debug_assert!(
					is_ok,
					"ready books can never overflow because duplicates cannot be pushed"
				);
			});
		}
	}
}

pub struct MaxEncodedLenOf<T>(sp_std::marker::PhantomData<T>);
impl<T: MaxEncodedLen> Get<u32> for MaxEncodedLenOf<T> {
	fn get() -> u32 {
		T::max_encoded_len() as u32
	}
}

pub struct MaxMessageLen<Origin, Size, HeapSize>(
	sp_std::marker::PhantomData<(Origin, Size, HeapSize)>,
);
impl<Origin: MaxEncodedLen, Size: MaxEncodedLen + Into<u32>, HeapSize: Get<Size>> Get<u32>
	for MaxMessageLen<Origin, Size, HeapSize>
{
	fn get() -> u32 {
		(HeapSize::get().into())
			.saturating_sub(Origin::max_encoded_len() as u32)
			.saturating_sub(ItemHeader::<Size>::max_encoded_len() as u32)
	}
}

pub type MaxMessageLenOf<T> =
	MaxMessageLen<MessageOriginOf<T>, <T as Config>::Size, <T as Config>::HeapSize>;
pub type MaxOriginLenOf<T> = MaxEncodedLenOf<MessageOriginOf<T>>;
pub type MessageOriginOf<T> = <<T as Config>::MessageProcessor as ProcessMessage>::Origin;
pub type HeapSizeU32Of<T> = IntoU32<<T as Config>::HeapSize, <T as Config>::Size>;

pub struct IntoU32<T, O>(sp_std::marker::PhantomData<(T, O)>);
impl<T: Get<O>, O: Into<u32>> Get<u32> for IntoU32<T, O> {
	fn get() -> u32 {
		T::get().into()
	}
}

pub trait ServiceQueue<Origin> {
	/// Service the given message queue.
	///
	/// - `weight_limit`: The maximum amount of dynamic weight that this call can use.
	///
	/// Returns the dynamic weight used by this call; is never greater than `weight_limit`.
	fn service_queue(origin: Origin, weight_limit: Weight) -> Weight;
}

pub trait ServiceQueues {
	/// Service all message queues in some fair manner.
	///
	/// - `weight_limit`: The maximum amount of dynamic weight that this call can use.
	///
	/// Returns the dynamic weight used by this call; is never greater than `weight_limit`.
	fn service_queues(weight_limit: Weight) -> Weight;
}

impl<T: Config> ServiceQueue<MessageOriginOf<T>> for Pallet<T> {
	fn service_queue(origin: MessageOriginOf<T>, weight_limit: Weight) -> Weight {
		let mut processed = 0;
		let max_weight = weight_limit.saturating_mul(3).saturating_div(4);
		let mut weight_left = weight_limit;
		let mut book_state = BookStateOf::<T>::get(&origin);
		while book_state.end > book_state.begin {
			let page_index = book_state.begin;
			// TODO: Check `weight_left` and bail before doing this storage read.
			let mut page = match Pages::<T>::get(&origin, page_index) {
				Some(p) => p,
				None => {
					debug_assert!(false, "message-queue: referenced page not found");
					break
				},
			};

			let bail = loop {
				let mut message = &match page.peek_first() {
					Some(m) => m,
					None => break false,
				}[..];

				let is_processed = match MessageOriginOf::<T>::decode(&mut message) {
					Ok(origin) => {
						let hash = T::Hashing::hash(message);
						use ProcessMessageError::Overweight;
						match T::MessageProcessor::process_message(
							message,
							origin.clone(),
							weight_left,
						) {
							Err(Overweight(w)) if processed == 0 || w.any_gt(max_weight) => {
								// Permanently overweight.
								Self::deposit_event(Event::<T>::Overweight {
									hash,
									origin,
									index: 0,
								}); // TODO page + index
								false
							},
							Err(Overweight(_)) => {
								// Temporarily overweight - save progress and stop processing this
								// queue.
								break true
							},
							Err(error) => {
								// Permanent error - drop
								Self::deposit_event(Event::<T>::ProcessingFailed {
									hash,
									origin,
									error,
								});
								true
							},
							Ok((success, weight_used)) => {
								// Success
								processed.saturating_inc();
								weight_left = weight_left.saturating_sub(weight_used);
								let event =
									Event::<T>::Processed { hash, origin, weight_used, success };
								Self::deposit_event(event);
								true
							},
						}
					},
					Err(_) => {
						let hash = T::Hashing::hash(message);
						Self::deposit_event(Event::<T>::Discarded { hash });
						false
					},
				};
				page.skip_first(is_processed);
			};

			if page.is_complete() {
				debug_assert!(!bail, "we never bail if a page became complete");
				Pages::<T>::remove(&origin, page_index);
				if book_state.earliest == page_index {
					book_state.earliest.saturating_inc();
				}
			} else {
				Pages::<T>::insert(&origin, page_index, page);
			}

			if bail {
				break
			}
			book_state.begin.saturating_inc();
		}
		if book_state.begin >= book_state.end && processed > 0 {
			// empty now having processed at least one.
			ReadyBooks::<T>::mutate(|b| {
				b.retain(|b| b != &origin);
			});
		}
		BookStateOf::<T>::insert(&origin, book_state);
		weight_limit.saturating_sub(weight_left)
	}
}

impl<T: Config> ServiceQueues for Pallet<T> {
	fn service_queues(weight_limit: Weight) -> Weight {
		let queues = ReadyBooks::<T>::get();
		// TODO: take a random number and shuffle the queues rather than the next stuff.
		let mut weight_used = Weight::zero();
		for origin in queues.into_iter() {
			let left = weight_limit - weight_used;
			if left.any_lte(Weight::zero()) {
				break
			}
			let used = Self::service_queue(origin, left);
			weight_used.saturating_accrue(used);
		}
		weight_used
	}
}

pub trait EnqueueMessage<Origin: MaxEncodedLen> {
	type MaxMessageLen: Get<u32>;

	/// Enqueue a single `message` from a specific `origin`.
	///
	/// Infallible.
	fn enqueue_message(message: BoundedSlice<u8, Self::MaxMessageLen>, origin: Origin);

	/// Enqueue multiple `messages` from a specific `origin`.
	///
	/// If no `message.len()` is greater than `HEAP_SIZE - Origin::max_encoded_len()`, then this
	/// is guaranteed to succeed. In the case of `Err`, no messages are queued.
	fn enqueue_messages<'a>(
		messages: impl Iterator<Item = BoundedSlice<'a, u8, Self::MaxMessageLen>>,
		origin: Origin,
	);

	// TODO: consider: `fn enqueue_mqc_page(page: &[u8], origin: Origin);`
}

impl<T: Config> EnqueueMessage<MessageOriginOf<T>> for Pallet<T> {
	type MaxMessageLen =
		MaxMessageLen<<T::MessageProcessor as ProcessMessage>::Origin, T::Size, T::HeapSize>;

	fn enqueue_message(
		message: BoundedSlice<u8, Self::MaxMessageLen>,
		origin: <T::MessageProcessor as ProcessMessage>::Origin,
	) {
		// the `truncate_from` is just for safety - it will never fail since the bound is the
		// maximum encoded length of the type.
		origin.using_encoded(|data| {
			Self::do_enqueue_message(&origin, message, BoundedSlice::truncate_from(data))
		})
	}

	fn enqueue_messages<'a>(
		messages: impl Iterator<Item = BoundedSlice<'a, u8, Self::MaxMessageLen>>,
		origin: <T::MessageProcessor as ProcessMessage>::Origin,
	) {
		origin.using_encoded(|data| {
			// the `truncate_from` is just for safety - it will never fail since the bound is the
			// maximum encoded length of the type.
			let origin_data = BoundedSlice::truncate_from(data);
			for message in messages {
				Self::do_enqueue_message(&origin, message, origin_data);
			}
		})
	}
}
