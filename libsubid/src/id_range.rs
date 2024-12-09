use crate::Id;
use core::ops::{Bound, Deref, Range, RangeBounds};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IdRange(Range<Id>);

impl IdRange {
    pub const fn new(start: Id, end: Id) -> Self {
        Self(start..end)
    }

    pub const fn from_count(start: Id, count: Id) -> Self {
        Self::new(start, start + count)
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

impl Deref for IdRange {
    type Target = Range<Id>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for IdRange
where
    T: RangeBounds<Id>,
{
    fn from(value: T) -> Self {
        let start = match value.start_bound() {
            core::ops::Bound::Included(start) => *start,
            core::ops::Bound::Excluded(start) => start + 1,
            core::ops::Bound::Unbounded => 0,
        };
        let end = match value.end_bound() {
            core::ops::Bound::Included(end) => *end,
            core::ops::Bound::Excluded(end) => end - 1,
            core::ops::Bound::Unbounded => Id::MAX,
        };
        Self::new(start, end + 1)
    }
}
