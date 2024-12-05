pub mod error;
pub mod kind;
pub mod range;

#[cfg(feature = "mock")]
pub mod mock;

use error::Result;
use kind::Kind;
use range::{Id, IdRange};

pub trait SubidExtractor {
    fn has_range(&self, kind: &Kind, owner: &str, id_range: &IdRange) -> Result<bool>;
    fn find_subid_owners(&self, kind: &Kind, id: &Id) -> Result<Box<[Id]>>;
    fn list_owner_ranges(&self, kind: &Kind, owner: &str) -> Result<Box<[IdRange]>>;
}
