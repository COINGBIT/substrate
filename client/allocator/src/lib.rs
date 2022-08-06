// This file is part of Substrate.

// Copyright (C) 2020-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Collection of allocator implementations.
//!
//! This crate provides the following allocator implementations:
//! - A freeing-bump allocator: [`FreeingBumpHeapAllocator`](freeing_bump::FreeingBumpHeapAllocator)

#![warn(missing_docs)]

mod error;
mod freeing_bump;

pub use error::Error;
pub use freeing_bump::{AllocationStats, FreeingBumpHeapAllocator};

pub trait Memory {
	fn with_access_mut<R>(&mut self, run: impl FnOnce(&mut [u8]) -> R) -> R;
	fn with_access<R>(&self, run: impl FnOnce(&[u8]) -> R) -> R;
	fn grow(&mut self, additional: u32) -> Result<(), ()>;
	fn pages(&self) -> u32;
}
