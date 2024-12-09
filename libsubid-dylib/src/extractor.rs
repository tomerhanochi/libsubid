use crate::id_type::IdType;
use alloc::boxed::Box;

#[cfg(not(any(
    feature = "subuid-mock",
    feature = "subuid-dynamic",
    feature = "subuid-dynamic-systemd"
)))]
compile_error!("At least one subuid extractor feature must be enabled.");

#[cfg(not(any(
    feature = "subgid-mock",
    feature = "subgid-dynamic",
    feature = "subgid-dynamic-systemd"
)))]
compile_error!("At least one subgid extractor feature must be enabled.");

#[cfg(any(feature = "subuid-mock", feature = "subgid-mock"))]
fn mock_subid_extractor() -> ::libsubid::MockSubidExtractor {
    ::libsubid::MockSubidExtractor::new({
        let mut map = ::alloc::collections::BTreeMap::new();
        map.insert(
            1000,
            vec![::libsubid::IdRange::from_count(524288, 65536)].into_boxed_slice(),
        );
        map
    })
}

#[cfg(any(feature = "subuid-dynamic", feature = "subgid-dynamic"))]
fn dynamic_subid_extractor() -> ::libsubid::DynamicSubidExtractor {
    ::libsubid::DynamicSubidExtractor::try_new((1000..25000).into()).unwrap()
}

#[cfg(any(feature = "subuid-dynamic-systemd", feature = "subgid-dynamic-systemd"))]
fn systemd_dynamic_subid_extractor() -> ::libsubid::DynamicSubidExtractor {
    ::libsubid::DynamicSubidExtractor::try_new((61184..65519).into()).unwrap()
}

fn noop_subid_extractor() -> ::libsubid::NoopSubidExtractor {
    ::libsubid::NoopSubidExtractor::new()
}

pub(crate) fn subid_extractor(subid_type: IdType) -> Box<dyn::libsubid::SubidExtractor> {
    #[allow(unreachable_code)]
    match subid_type {
        IdType::Uid => {
            #[cfg(feature = "subuid-dynamic-systemd")]
            return Box::new(systemd_dynamic_subid_extractor());
            #[cfg(feature = "subuid-dynamic")]
            return Box::new(dynamic_subid_extractor());
            #[cfg(feature = "subuid-mock")]
            return Box::new(mock_subid_extractor());

            Box::new(noop_subid_extractor())
        }
        IdType::Gid => {
            #[cfg(feature = "subgid-dynamic-systemd")]
            return Box::new(systemd_dynamic_subid_extractor());
            #[cfg(feature = "subgid-dynamic")]
            return Box::new(dynamic_subid_extractor());
            #[cfg(feature = "subgid-mock")]
            return Box::new(mock_subid_extractor());

            Box::new(noop_subid_extractor())
        }
    }
}
