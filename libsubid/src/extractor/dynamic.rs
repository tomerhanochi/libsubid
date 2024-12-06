use crate::{Id, IdRange, Result, SubidExtractor};
use alloc::boxed::Box;

const ID_RANGE_SIZE: u32 = 65536;
const SUBID_START: u32 = u32::MAX / ID_RANGE_SIZE;

pub struct DynamicSubidExtractor();

impl DynamicSubidExtractor {
    pub fn new() -> Self {
        Self()
    }
}

impl Default for DynamicSubidExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl SubidExtractor for DynamicSubidExtractor {
    fn find_subid_owners(&self, subid: &Id) -> Result<alloc::boxed::Box<[Id]>> {
        Ok(vec![subid / ID_RANGE_SIZE].into_boxed_slice())
    }

    fn list_owner_ranges(&self, id: &Id) -> Result<alloc::boxed::Box<[IdRange]>> {
        if (SUBID_START..).contains(id) {
            return Ok(Box::new([]));
        }
        Ok(vec![IdRange::new(id * ID_RANGE_SIZE, ID_RANGE_SIZE)].into_boxed_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn extractor() -> DynamicSubidExtractor {
        DynamicSubidExtractor::new()
    }

    #[test]
    fn test_dynamic_subid_extractor_has_range() {
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
                    subid_range: IdRange::new(1000 * ID_RANGE_SIZE, ID_RANGE_SIZE),
                },
                output: Ok(true),
            },
            TestCase {
                input: TestCaseInput {
                    owner: 1000,
                    subid_range: IdRange::new(1000 * ID_RANGE_SIZE + 1, 65536 - 1),
                },
                output: Ok(true),
            },
            TestCase {
                input: TestCaseInput {
                    owner: 1000,
                    subid_range: IdRange::new(1000 * ID_RANGE_SIZE + 1, 65536),
                },
                output: Ok(false),
            },
            TestCase {
                input: TestCaseInput {
                    owner: 1000,
                    subid_range: IdRange::new(1000 * ID_RANGE_SIZE - 1, 65536),
                },
                output: Ok(false),
            },
            TestCase {
                input: TestCaseInput {
                    owner: 999,
                    subid_range: IdRange::new(1000 * ID_RANGE_SIZE - 1, 65536),
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
