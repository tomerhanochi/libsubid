use crate::Id;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IdRange {
    pub start: Id,
    pub count: Id,
}

impl IdRange {
    pub(crate) fn contains_id(&self, id: &Id) -> bool {
        (self.start..self.start + self.count).contains(id)
    }

    pub(crate) fn contains_id_range(&self, other: &Self) -> bool {
        self.contains_id(&other.start) && self.contains_id(&(other.start + other.count))
    }
}
