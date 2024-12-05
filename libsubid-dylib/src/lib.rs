use std::{collections::HashMap, sync::LazyLock};

use libsubid::{self, kind::Kind, mock::MockSubidExtractor, range::IdRange, Id, SubidExtractor};

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
            vec![IdRange {
                start: 524288,
                count: 65536,
            }]
            .into_boxed_slice(),
        );
        map
    };
    let subgid_map = subuid_map.clone();
    libsubid::mock::MockSubidExtractor::new(user_ids, subuid_map, subgid_map)
});

pub type SubidType = libc::c_uint;
pub const SUBID_TYPE_UID: SubidType = 1;
pub const SUBID_TYPE_GID: SubidType = 2;

pub type SubidStatus = libc::c_uint;
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
    owner: *const libc::c_char,
    start: libc::c_ulong,
    count: libc::c_ulong,
    id_type: SubidType,
    result: *mut libc::c_int,
) -> SubidStatus {
    let owner = unsafe { core::ffi::CStr::from_ptr(owner) };
    let Ok(owner) = owner.to_str() else {
        unsafe {
            *result = 0;
        }
        return SUBID_STATUS_ERROR;
    };
    let kind = match id_type {
        SUBID_TYPE_UID => Kind::Uid,
        SUBID_TYPE_GID => Kind::Gid,
        _ => {
            unsafe {
                *result = 0;
            }
            return SUBID_STATUS_ERROR;
        }
    };
    match MOCK_SUBID_EXTRACTOR.has_range(
        &kind,
        owner,
        &IdRange {
            start: start as Id,
            count: count as Id,
        },
    ) {
        Ok(res) => {
            unsafe {
                *result = match res {
                    true => 1,
                    false => 0,
                };
            }
            SUBID_STATUS_SUCCESS
        }
        Err(err) => {
            unsafe {
                *result = 0;
            }
            match err {
                libsubid::error::Error::General => SUBID_STATUS_ERROR,
                libsubid::error::Error::Connection => SUBID_STATUS_ERROR_CONN,
                libsubid::error::Error::UnknownUser => SUBID_STATUS_UNKNOWN_USER,
            }
        }
    }
}

#[no_mangle]
/**
 * # Safety
 */
pub unsafe extern "C" fn shadow_subid_find_subid_owners(
    id: libc::c_ulong,
    id_type: SubidType,
    uids: *mut *const libc::uid_t,
    count: *mut libc::c_int,
) -> SubidStatus {
    let kind = match id_type {
        SUBID_TYPE_UID => Kind::Uid,
        SUBID_TYPE_GID => Kind::Gid,
        _ => {
            return SUBID_STATUS_ERROR;
        }
    };
    match MOCK_SUBID_EXTRACTOR.find_subid_owners(&kind, &(id as Id)) {
        Ok(owner_ids) => {
            let owner_ids = owner_ids
                .iter()
                .map(|owner_id| (*owner_id as libc::uid_t))
                .collect::<Vec<_>>();
            unsafe {
                *uids = owner_ids.as_ptr();
                *count = owner_ids.len() as libc::c_int;
            }
            SUBID_STATUS_SUCCESS
        }
        Err(err) => match err {
            libsubid::error::Error::General => SUBID_STATUS_ERROR,
            libsubid::error::Error::Connection => SUBID_STATUS_ERROR_CONN,
            libsubid::error::Error::UnknownUser => SUBID_STATUS_UNKNOWN_USER,
        },
    }
}

#[repr(C)]
pub struct SubidRange {
    start: libc::c_ulong,
    count: libc::c_ulong,
}

#[no_mangle]
/**
 * # Safety
 */
pub unsafe extern "C" fn shadow_subid_list_owner_ranges(
    owner: *const libc::c_char,
    id_type: SubidType,
    in_ranges: *mut *const SubidRange,
    count: *mut libc::c_int,
) -> SubidStatus {
    unsafe {
        *count = 0;
    }
    let owner = unsafe { core::ffi::CStr::from_ptr(owner) };
    let Ok(owner) = owner.to_str() else {
        return SUBID_STATUS_ERROR;
    };
    let kind = match id_type {
        SUBID_TYPE_UID => Kind::Uid,
        SUBID_TYPE_GID => Kind::Gid,
        _ => {
            return SUBID_STATUS_ERROR;
        }
    };
    match MOCK_SUBID_EXTRACTOR.list_owner_ranges(&kind, owner) {
        Ok(id_ranges) => {
            let id_ranges = id_ranges
                .iter()
                .map(|id_range| SubidRange {
                    start: id_range.start as libc::c_ulong,
                    count: id_range.count as libc::c_ulong,
                })
                .collect::<Vec<_>>();
            unsafe {
                *in_ranges = id_ranges.as_ptr();
                *count = id_ranges.len() as i32;
            }
            SUBID_STATUS_SUCCESS
        }
        Err(err) => match err {
            libsubid::error::Error::General => SUBID_STATUS_ERROR,
            libsubid::error::Error::Connection => SUBID_STATUS_ERROR_CONN,
            libsubid::error::Error::UnknownUser => SUBID_STATUS_UNKNOWN_USER,
        },
    }
}
