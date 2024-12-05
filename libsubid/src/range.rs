pub type Id = u64;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IdRange(core::ops::Range<Id>);

impl From<core::ops::Range<Id>> for IdRange {
    fn from(value: core::ops::Range<Id>) -> Self {
        Self(value)
    }
}

impl IdRange {
    pub fn contains(&self, id: &Id) -> bool {
        self.0.contains(id)
    }

    pub fn contains_range(&self, other: &Self) -> bool {
        self.contains(&other.0.start) && self.contains(&other.0.end)
    }
}
