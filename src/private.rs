use std::ffi::{c_void, CStr};

use core_foundation::array::{CFArrayGetCount, CFArrayGetValueAtIndex, CFArrayRef};
use core_foundation::base::*;
use core_foundation::dictionary::{CFDictionaryApplyFunction, CFDictionaryRef};
use core_foundation::number::{
    kCFNumberIntType, kCFNumberSInt32Type, kCFNumberSInt64Type, CFNumberGetType, CFNumberGetTypeID,
    CFNumberGetValue, CFNumberRef,
};
use core_foundation::string::{kCFStringEncodingUTF8, CFStringGetCStringPtr, CFStringRef};

#[test]
fn blah() {
    let connection = unsafe { CGSMainConnectionID() };

    // let r = unsafe { CGSGetActiveSpace(connection) };

    let space_list_info = unsafe { CGSCopyManagedDisplaySpaces(connection) };
    let count = unsafe { CFArrayGetCount(space_list_info) };

    for i in 0..count {
        let dic_ref =
            unsafe { CFArrayGetValueAtIndex(space_list_info, i as isize) as CFDictionaryRef };

        extern "C" fn callback(
            key: *const std::ffi::c_void,
            value: *const std::ffi::c_void,
            cx: *mut std::ffi::c_void,
        ) -> () {
            let cf_ref = key as CFStringRef;
            let c_ptr = unsafe { CFStringGetCStringPtr(cf_ref, kCFStringEncodingUTF8) };

            if !c_ptr.is_null() {
                let c_result = unsafe { CStr::from_ptr(c_ptr) };
                let result = String::from(c_result.to_str().unwrap());
                println!("Key: {}", result);
            }
        }

        let cx: *mut c_void = std::ptr::null() as *const c_void as *mut _;

        let cb = callback as *const ();

        unsafe { CFDictionaryApplyFunction(dic_ref, callback, cx) };

        // println!("{}", id);
    }

    // dbg!(r);
}

extern "C" {
    /*
     * CFDictionary.h
     */

    pub fn CGSMainConnectionID() -> CFIndex;

    pub fn CGSGetActiveSpace(connect: CFIndex) -> CFIndex;

    pub fn CGSCopyManagedDisplaySpaces(connect: CFIndex) -> CFArrayRef;
    // // XXX: Undocumented private API to get the CGSSpaceID for the active space
    // CGSSpaceID CGSGetActiveSpace(CGSConnectionID connection);

    // pub static kCFTypeDictionaryKeyCallBacks: CFDictionaryKeyCallBacks;
    // pub static kCFTypeDictionaryValueCallBacks: CFDictionaryValueCallBacks;

    // pub fn CFDictionaryContainsKey(theDict: CFDictionaryRef, key: *const c_void) -> Boolean;
    // pub fn CFDictionaryCreate(
    //     allocator: CFAllocatorRef,
    //     keys: *const *const c_void,
    //     values: *const *const c_void,
    //     numValues: CFIndex,
    //     keyCallBacks: *const CFDictionaryKeyCallBacks,
    //     valueCallBacks: *const CFDictionaryValueCallBacks,
    // ) -> CFDictionaryRef;
    // pub fn CFDictionaryGetCount(theDict: CFDictionaryRef) -> CFIndex;
    // pub fn CFDictionaryGetTypeID() -> CFTypeID;
    // pub fn CFDictionaryGetValueIfPresent(
    //     theDict: CFDictionaryRef,
    //     key: *const c_void,
    //     value: *mut *const c_void,
    // ) -> Boolean;
    // pub fn CFDictionaryApplyFunction(
    //     theDict: CFDictionaryRef,
    //     applier: CFDictionaryApplierFunction,
    //     context: *mut c_void,
    // );
    // pub fn CFDictionaryGetKeysAndValues(
    //     theDict: CFDictionaryRef,
    //     keys: *mut *const c_void,
    //     values: *mut *const c_void,
    // );

    // pub fn CFDictionaryCreateMutable(
    //     allocator: CFAllocatorRef,
    //     capacity: CFIndex,
    //     keyCallbacks: *const CFDictionaryKeyCallBacks,
    //     valueCallbacks: *const CFDictionaryValueCallBacks,
    // ) -> CFMutableDictionaryRef;
    // pub fn CFDictionaryCreateMutableCopy(
    //     allocator: CFAllocatorRef,
    //     capacity: CFIndex,
    //     theDict: CFDictionaryRef,
    // ) -> CFMutableDictionaryRef;
    // pub fn CFDictionaryAddValue(
    //     theDict: CFMutableDictionaryRef,
    //     key: *const c_void,
    //     value: *const c_void,
    // );
    // pub fn CFDictionarySetValue(
    //     theDict: CFMutableDictionaryRef,
    //     key: *const c_void,
    //     value: *const c_void,
    // );
    // pub fn CFDictionaryReplaceValue(
    //     theDict: CFMutableDictionaryRef,
    //     key: *const c_void,
    //     value: *const c_void,
    // );
    // pub fn CFDictionaryRemoveValue(theDict: CFMutableDictionaryRef, key: *const c_void);
    // pub fn CFDictionaryRemoveAllValues(theDict: CFMutableDictionaryRef);
}
