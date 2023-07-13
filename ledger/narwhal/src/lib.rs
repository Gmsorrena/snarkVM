// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![forbid(unsafe_code)]
#![warn(clippy::cast_possible_truncation)]

#[cfg(feature = "transmission")]
mod helpers;
#[cfg(feature = "transmission")]
pub use helpers::*;

#[cfg(feature = "batch-certificate")]
pub mod batch_certificate;
#[cfg(feature = "batch-certificate")]
pub use batch_certificate::*;

#[cfg(feature = "batch-header")]
pub mod batch_header;
#[cfg(feature = "batch-header")]
pub use batch_header::*;

#[cfg(feature = "transmission")]
pub mod transmission;
#[cfg(feature = "transmission")]
pub use transmission::*;

#[cfg(feature = "transmission-id")]
pub mod transmission_id;
#[cfg(feature = "transmission-id")]
pub use transmission_id::*;
