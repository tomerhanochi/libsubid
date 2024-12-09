use crate::{Id, IdRange, Result, SubidExtractor};
use alloc::boxed::Box;

pub const CONTAINER_ID_RANGE: IdRange = IdRange::new(524_288, 1_878_982_656);
pub const SUBID_ALLOCATION_SIZE: Id = 65536;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DynamicSubidExtractor {
    owner_id_range: IdRange,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NewDynamicSubidExtractorError {
    TooManyOwnerIds,
}

impl core::fmt::Display for NewDynamicSubidExtractorError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::TooManyOwnerIds => write!(f, ""),
        }
    }
}

impl core::error::Error for NewDynamicSubidExtractorError {}

impl DynamicSubidExtractor {
    pub fn try_new(
        owner_id_range: IdRange,
    ) -> core::result::Result<Self, NewDynamicSubidExtractorError> {
        if owner_id_range.len() > (CONTAINER_ID_RANGE.len() / (SUBID_ALLOCATION_SIZE as usize)) {
            return Err(NewDynamicSubidExtractorError::TooManyOwnerIds);
        }
        Ok(Self { owner_id_range })
    }
}

impl SubidExtractor for DynamicSubidExtractor {
    fn find_subid_owners(&self, subid: &Id) -> Result<Box<[Id]>> {
        Ok(vec![
            (*subid - CONTAINER_ID_RANGE.start) / SUBID_ALLOCATION_SIZE + self.owner_id_range.start,
        ]
        .into_boxed_slice())
    }

    fn list_owner_ranges(&self, id: &Id) -> Result<Box<[IdRange]>> {
        if !self.owner_id_range.contains(id) {
            return Ok(vec![].into_boxed_slice());
        }
        Ok(vec![IdRange::from_count(
            (id - self.owner_id_range.start) * SUBID_ALLOCATION_SIZE + CONTAINER_ID_RANGE.start,
            SUBID_ALLOCATION_SIZE,
        )]
        .into_boxed_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn extractor() -> DynamicSubidExtractor {
        DynamicSubidExtractor::try_new((1000..1001).into()).unwrap()
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
                    subid_range: IdRange::from_count(
                        CONTAINER_ID_RANGE.start,
                        SUBID_ALLOCATION_SIZE,
                    ),
                },
                output: Ok(true),
            },
            TestCase {
                input: TestCaseInput {
                    owner: 1000,
                    subid_range: IdRange::from_count(
                        CONTAINER_ID_RANGE.start + 1,
                        SUBID_ALLOCATION_SIZE - 1,
                    ),
                },
                output: Ok(true),
            },
            TestCase {
                input: TestCaseInput {
                    owner: 1000,
                    subid_range: IdRange::from_count(
                        CONTAINER_ID_RANGE.start + 1,
                        SUBID_ALLOCATION_SIZE,
                    ),
                },
                output: Ok(false),
            },
            TestCase {
                input: TestCaseInput {
                    owner: 1000,
                    subid_range: IdRange::from_count(
                        CONTAINER_ID_RANGE.start - 1,
                        SUBID_ALLOCATION_SIZE,
                    ),
                },
                output: Ok(false),
            },
            TestCase {
                input: TestCaseInput {
                    owner: 999,
                    subid_range: IdRange::from_count(
                        CONTAINER_ID_RANGE.start,
                        SUBID_ALLOCATION_SIZE,
                    ),
                },
                output: Ok(false),
            },
            TestCase {
                input: TestCaseInput {
                    owner: 1001,
                    subid_range: IdRange::from_count(
                        CONTAINER_ID_RANGE.start,
                        SUBID_ALLOCATION_SIZE,
                    ),
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
