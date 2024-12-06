#[macro_use]
extern crate alloc;

mod extractor;
mod id_range;
mod id_type;
mod status_code;

use alloc::{boxed::Box, vec::Vec};
use extractor::subid_extractor;
use id_range::IdRange;
use id_type::IdType;
use status_code::StatusCode;

unsafe fn uid_from_name(owner: *const ::libc::c_char) -> Option<::libc::uid_t> {
    let owner_passwd = libc::getpwnam(owner);
    if owner_passwd.is_null() {
        return None;
    }
    Some((*owner_passwd).pw_uid)
}

/**
 * # Safety
 */
#[no_mangle]
pub unsafe extern "C" fn shadow_subid_has_range(
    owner_ptr: *const ::libc::c_char,
    start: ::libc::c_ulong,
    count: ::libc::c_ulong,
    subid_type: IdType,
    has_range_ptr: *mut bool,
) -> StatusCode {
    let owner_uid = match uid_from_name(owner_ptr) {
        Some(val) => val,
        None => return StatusCode::UnknownUser,
    };
    let start = match start.try_into() {
        Ok(val) => val,
        Err(_) => return StatusCode::Error,
    };
    let count = match count.try_into() {
        Ok(val) => val,
        Err(_) => return StatusCode::Error,
    };
    let id_range = ::libsubid::IdRange::new(start, count);

    match subid_extractor(subid_type).has_range(&owner_uid, &id_range) {
        Ok(has_range) => {
            *has_range_ptr = has_range;
            StatusCode::Success
        }
        Err(err) => err.into(),
    }
}

/**
 * # Safety
 */
#[no_mangle]
pub unsafe extern "C" fn shadow_subid_find_subid_owners(
    subid: ::libc::c_ulong,
    subid_type: IdType,
    owner_uids_ptr: *mut *const ::libc::uid_t,
    length_ptr: *mut ::libc::c_int,
) -> StatusCode {
    let subid = match subid.try_into() {
        Ok(val) => val,
        Err(_) => return StatusCode::Error,
    };
    match subid_extractor(subid_type).find_subid_owners(&subid) {
        Ok(subid_owners) => {
            let length = subid_owners.len();
            let mut owner_uids: Vec<::libc::uid_t> = Vec::with_capacity(length);
            for subid_owner in subid_owners {
                owner_uids.push(subid_owner);
            }
            *owner_uids_ptr = Box::leak(owner_uids.into_boxed_slice()).as_ptr();
            *length_ptr = length as ::libc::c_int;
            StatusCode::Success
        }
        Err(err) => err.into(),
    }
}

/**
 * # Safety
 */
#[no_mangle]
pub unsafe extern "C" fn shadow_subid_list_owner_ranges(
    owner_ptr: *const ::libc::c_char,
    id_type: IdType,
    owner_subid_ranges_ptr: *mut *const IdRange,
    length_ptr: *mut ::libc::c_int,
) -> StatusCode {
    let owner_uid = match uid_from_name(owner_ptr) {
        Some(val) => val,
        None => return StatusCode::UnknownUser,
    };

    match subid_extractor(id_type).list_owner_ranges(&owner_uid) {
        Ok(owner_ranges) => {
            let length = owner_ranges.len();
            let mut owner_subid_ranges: Vec<IdRange> = Vec::with_capacity(length);
            for owner_range in owner_ranges {
                owner_subid_ranges.push(owner_range.into());
            }
            *owner_subid_ranges_ptr = Box::leak(owner_subid_ranges.into_boxed_slice()).as_ptr();
            *length_ptr = length as ::libc::c_int;
            StatusCode::Success
        }
        Err(err) => err.into(),
    }
}

/**
 * # Safety
 */
#[no_mangle]
pub unsafe extern "C" fn shadow_subid_free(ptr: *mut ::libc::c_void) {
    libc::free(ptr);
}
