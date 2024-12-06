use crate::Id;
use core::ops::{Bound, RangeBounds};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IdRange(core::ops::Range<Id>);

impl IdRange {
    pub fn new(start: Id, count: Id) -> Self {
        Self(start..start + count)
    }

    pub(crate) fn contains_id_range(&self, other: &Self) -> bool {
        (match other.start_bound() {
            Bound::Included(start) => self.contains(start),
            Bound::Excluded(start) => self.contains(&(start + 1)),
            Bound::Unbounded => unreachable!(),
        } && match other.end_bound() {
            Bound::Included(end) => self.contains(end),
            Bound::Excluded(end) => self.contains(&(end - 1)),
            Bound::Unbounded => unreachable!(),
        })
    }
}

impl RangeBounds<Id> for IdRange {
    fn start_bound(&self) -> Bound<&Id> {
        self.0.start_bound()
    }
    fn end_bound(&self) -> Bound<&Id> {
        self.0.end_bound()
    }
}
