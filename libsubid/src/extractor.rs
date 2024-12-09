#[cfg(feature = "dynamic")]
pub mod dynamic;
#[cfg(feature = "mock")]
pub mod mock;
#[cfg(feature = "noop")]
pub mod noop;

use crate::{Id, IdRange, Result};
use alloc::boxed::Box;

pub trait SubidExtractor {
    fn find_subid_owners(&self, subid: &Id) -> Result<Box<[Id]>>;

    fn list_owner_ranges(&self, id: &Id) -> Result<Box<[IdRange]>>;

    fn has_range(&self, id: &Id, subid_range: &IdRange) -> Result<bool> {
        for owner_id_range in self.list_owner_ranges(id)? {
            if owner_id_range.contains_id_range(subid_range) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
