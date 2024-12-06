use crate::{Id, IdRange, Result, SubidExtractor};
use alloc::{boxed::Box, collections::BTreeMap, vec::Vec};
use core::ops::RangeBounds;

pub struct MockSubidExtractor(BTreeMap<Id, Box<[IdRange]>>);

impl MockSubidExtractor {
    pub fn new(subids: BTreeMap<Id, Box<[IdRange]>>) -> Self {
        Self(subids)
    }
}

impl Default for MockSubidExtractor {
    fn default() -> Self {
        Self::new(BTreeMap::new())
    }
}

impl SubidExtractor for MockSubidExtractor {
    fn find_subid_owners(&self, subid: &Id) -> Result<Box<[Id]>> {
        let mut owner_ids = Vec::new();
        for (owner_id, owner_subid_ranges) in self.0.iter() {
            for owner_subid_range in owner_subid_ranges.iter() {
                if owner_subid_range.contains(subid) {
                    owner_ids.push(*owner_id);
                    break;
                }
            }
        }
        Ok(owner_ids.into_boxed_slice())
    }

    fn list_owner_ranges(&self, id: &Id) -> Result<Box<[IdRange]>> {
        Ok(match self.0.get(id) {
            None => Vec::new().into_boxed_slice(),
            Some(owner_subid_ranges) => owner_subid_ranges.clone(),
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn extractor() -> MockSubidExtractor {
        let mut map = BTreeMap::new();
        map.insert(1000, vec![IdRange::new(524288, 65536)].into_boxed_slice());
        MockSubidExtractor::new(map)
    }

    #[test]
    fn test_mock_subid_extractor_has_range() {
        struct TestCaseInput {
            owner: Id,
            subid_range: IdRange,
        }
        struct TestCase {
            input: TestCaseInput,
            output: Result<bool>,
        }
        let test_cases = [
            TestCase {
                input: TestCaseInput {
                    owner: 1000,
                    subid_range: IdRange::new(524288, 65536),
                },
                output: Ok(true),
            },
            TestCase {
                input: TestCaseInput {
                    owner: 1000,
                    subid_range: IdRange::new(524288 + 1, 65536 - 1),
                },
                output: Ok(true),
            },
            TestCase {
                input: TestCaseInput {
                    owner: 1000,
                    subid_range: IdRange::new(524288 + 1, 65536),
                },
                output: Ok(false),
            },
            TestCase {
                input: TestCaseInput {
                    owner: 1000,
                    subid_range: IdRange::new(524288 - 1, 65536),
                },
                output: Ok(false),
            },
            TestCase {
                input: TestCaseInput {
                    owner: 999,
                    subid_range: IdRange::new(524288 - 1, 65536),
                },
                output: Ok(false),
            },
        ];
        let extractor = extractor();
        for tc in test_cases {
            assert_eq!(
                extractor.has_range(&tc.input.owner, &tc.input.subid_range),
                tc.output
            );
        }
    }
}
