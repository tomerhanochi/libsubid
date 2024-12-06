use crate::id_type::IdType;
use alloc::boxed::Box;

#[cfg(not(any(feature = "mock-subuid", feature = "dynamic-subuid",)))]
compile_error!("At least one subuid extractor feature must be enabled.");

#[cfg(not(any(feature = "mock-subgid", feature = "dynamic-subgid",)))]
compile_error!("At least one subgid extractor feature must be enabled.");

#[cfg(any(feature = "mock-subuid", feature = "mock-subgid"))]
fn mock_subid_extractor() -> ::libsubid::MockSubidExtractor {
    ::libsubid::MockSubidExtractor::new({
        let mut map = ::alloc::collections::BTreeMap::new();
        map.insert(
            1000,
            vec![::libsubid::IdRange::new(524288, 65536)].into_boxed_slice(),
        );
        map
    })
}

#[cfg(any(feature = "dynamic-subuid", feature = "dynamic-subgid"))]
fn dynamic_subid_extractor() -> ::libsubid::DynamicSubidExtractor {
    ::libsubid::DynamicSubidExtractor::new()
}

fn noop_subid_extractor() -> ::libsubid::NoopSubidExtractor {
    ::libsubid::NoopSubidExtractor::new()
}

pub(crate) fn subid_extractor(subid_type: IdType) -> Box<dyn::libsubid::SubidExtractor> {
    #[allow(unreachable_code)]
    match subid_type {
        IdType::Uid => {
            #[cfg(feature = "dynamic-subuid")]
            return Box::new(dynamic_subid_extractor());
            #[cfg(feature = "mock-subuid")]
            return Box::new(mock_subid_extractor());

            Box::new(noop_subid_extractor())
        }
        IdType::Gid => {
            #[cfg(feature = "dynamic-subgid")]
            return Box::new(dynamic_subid_extractor());
            #[cfg(feature = "mock-subgid")]
            return Box::new(mock_subid_extractor());
            Box::new(noop_subid_extractor())
        }
    }
}
