use crate::{Result, SubidExtractor};
use alloc::boxed::Box;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NoopSubidExtractor();

impl NoopSubidExtractor {
    pub fn new() -> Self {
        NoopSubidExtractor()
    }
}

impl Default for NoopSubidExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl SubidExtractor for NoopSubidExtractor {
    fn find_subid_owners(&self, _: &crate::Id) -> Result<Box<[crate::Id]>> {
        Ok(Box::new([]))
    }

    fn list_owner_ranges(&self, _: &crate::Id) -> crate::error::Result<Box<[crate::IdRange]>> {
        Ok(Box::new([]))
    }
}
