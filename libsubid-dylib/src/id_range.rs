use core::ops::{Bound, RangeBounds};

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct IdRange {
    pub start: ::libc::c_ulong,
    pub count: ::libc::c_ulong,
}

impl From<::libsubid::IdRange> for IdRange {
    fn from(value: ::libsubid::IdRange) -> Self {
        let start = match value.start_bound() {
            Bound::Included(start) => *start,
            Bound::Excluded(start) => *start + 1,
            Bound::Unbounded => unreachable!(),
        };
        let end = match value.end_bound() {
            Bound::Included(end) => *end,
            Bound::Excluded(end) => *end + 1,
            Bound::Unbounded => unreachable!(),
        };
        Self {
            start: start as ::libc::c_ulong,
            count: (end - start) as ::libc::c_ulong,
        }
    }
}
