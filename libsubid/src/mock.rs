use crate::{
    error::{Error, Result},
    kind::Kind,
    range::{Id, IdRange},
    SubidExtractor,
};
use std::collections::HashMap;

pub struct MockSubidExtractor {
    user_ids: HashMap<&'static str, Id>,
    subuid_map: HashMap<Id, Box<[IdRange]>>,
    subgid_map: HashMap<Id, Box<[IdRange]>>,
}

impl MockSubidExtractor {
    pub fn new(
        user_ids: HashMap<&'static str, Id>,
        subuid_map: HashMap<Id, Box<[IdRange]>>,
        subgid_map: HashMap<Id, Box<[IdRange]>>,
    ) -> Self {
        Self {
            user_ids,
            subuid_map,
            subgid_map,
        }
    }

    fn get_id(&self, user: &str) -> Option<&Id> {
        self.user_ids.get(user)
    }
}

impl SubidExtractor for MockSubidExtractor {
    fn has_range(&self, kind: &Kind, owner: &str, id_range: &IdRange) -> Result<bool> {
        match self.get_id(owner) {
            None => Err(Error::UnknownUser),
            Some(owner_id) => {
                let map = match kind {
                    Kind::Uid => &self.subuid_map,
                    Kind::Gid => &self.subgid_map,
                };
                match map.get(owner_id) {
                    None => Ok(false),
                    Some(owner_id_ranges) => Ok(owner_id_ranges
                        .iter()
                        .any(|owner_id_range| owner_id_range.contains_range(id_range))),
                }
            }
        }
    }

    fn find_subid_owners(&self, kind: &Kind, subid: &Id) -> Result<Box<[Id]>> {
        let map = match kind {
            Kind::Uid => &self.subuid_map,
            Kind::Gid => &self.subgid_map,
        };
        let mut owner_uids = Vec::new();
        for (id, id_ranges) in map.iter() {
            if id_ranges.iter().any(|id_range| id_range.contains(subid)) {
                owner_uids.push(*id);
            }
        }
        Ok(owner_uids.into_boxed_slice())
    }

    fn list_owner_ranges(&self, kind: &Kind, owner: &str) -> Result<Box<[IdRange]>> {
        let map = match kind {
            Kind::Uid => &self.subuid_map,
            Kind::Gid => &self.subgid_map,
        };
        match self.user_ids.get(owner) {
            None => Err(Error::UnknownUser),
            Some(owner_id) => Ok(match map.get(owner_id) {
                None => Vec::new().into_boxed_slice(),
                Some(owner_id_ranges) => owner_id_ranges.clone(),
            }),
        }
    }
}
