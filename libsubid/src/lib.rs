#![forbid(unsafe_code)]

#[macro_use]
extern crate alloc;

mod error;
mod extractor;
mod id;
mod id_range;

pub use error::{Error, Result};
#[cfg(feature = "dynamic")]
pub use extractor::dynamic::DynamicSubidExtractor;
#[cfg(feature = "mock")]
pub use extractor::mock::MockSubidExtractor;
#[cfg(feature = "noop")]
pub use extractor::noop::NoopSubidExtractor;
pub use extractor::SubidExtractor;
pub use id::Id;
pub use id_range::IdRange;
