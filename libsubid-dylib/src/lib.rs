use std::{collections::HashMap, sync::LazyLock};

use libsubid::{self, kind::Kind, mock::MockSubidExtractor, range::IdRange, SubidExtractor};

static MOCK_SUBID_EXTRACTOR: LazyLock<MockSubidExtractor> = LazyLock::new(|| {
    let user_ids = {
        let mut map = HashMap::new();
        map.insert("mock", 1000);
        map
    };
    let subuid_map = {
        let mut map = HashMap::new();
        map.insert(
            1000,
            vec![IdRange::from(524288..524288 + 65536)].into_boxed_slice(),
        );
        map
    };
    let subgid_map = subuid_map.clone();
    libsubid::mock::MockSubidExtractor::new(user_ids, subuid_map, subgid_map)
});

pub type SubidType = ::std::os::raw::c_uint;
pub const SUBID_TYPE_UID: SubidType = 1;
pub const SUBID_TYPE_GID: SubidType = 2;

pub type SubidStatus = core::ffi::c_uint;
pub const SUBID_STATUS_SUCCESS: SubidStatus = 0;
pub const SUBID_STATUS_UNKNOWN_USER: SubidStatus = 1;
pub const SUBID_STATUS_ERROR_CONN: SubidStatus = 2;
pub const SUBID_STATUS_ERROR: SubidStatus = 3;

#[no_mangle]
/**
 * # Safety
 * # TODO
 */
pub unsafe extern "C" fn shadow_subid_has_range(
    owner: *const core::ffi::c_char,
    start: core::ffi::c_ulong,
    count: core::ffi::c_ulong,
    subid_type: SubidType,
    result: *mut core::ffi::c_int,
) -> SubidStatus {
    let owner = unsafe { core::ffi::CStr::from_ptr(owner) };
    let Ok(owner) = owner.to_str() else {
        unsafe {
            *result = 1;
        }
        return SUBID_STATUS_ERROR;
    };
    let kind = match subid_type {
        SUBID_TYPE_UID => Kind::Uid,
        SUBID_TYPE_GID => Kind::Gid,
        _ => {
            unsafe {
                *result = 1;
            }
            return SUBID_STATUS_ERROR;
        }
    };
    match MOCK_SUBID_EXTRACTOR.has_range(&kind, owner, &IdRange::from(start..start + count)) {
        Ok(res) => match res {
            true => 1,
            false => 0,
        },
        Err(err) => {
            *result = 0;
            match err {
                libsubid::error::Error::General => SUBID_STATUS_ERROR,
                libsubid::error::Error::Connection => SUBID_STATUS_ERROR_CONN,
                libsubid::error::Error::UnknownUser => SUBID_STATUS_UNKNOWN_USER,
            }
        }
    }
}
