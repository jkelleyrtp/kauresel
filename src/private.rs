use std::ffi::{c_void, CStr};

use core_foundation::array::{
    CFArray, CFArrayCreate, CFArrayGetCount, CFArrayGetValueAtIndex, CFArrayRef,
};
use core_foundation::base::*;
use core_foundation::dictionary::{CFDictionaryApplyFunction, CFDictionaryRef};
use core_foundation::number::{
    kCFNumberIntType, kCFNumberSInt32Type, kCFNumberSInt64Type, CFNumber, CFNumberGetType,
    CFNumberGetTypeID, CFNumberGetValue, CFNumberRef,
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

#[test]
fn get_space_id_for_window() {
    let connection = unsafe { CGSMainConnectionID() };

    let window_id = 57147_i32;
    let mask = 7;

    let n0 = CFNumber::from(window_id);

    let array = CFArray::<CFType>::from_CFTypes(&[n0.as_CFType()]);

    let re = array.as_concrete_TypeRef();

    // let cf_arr = CFArrayCreate(allocator, values, numValues, callBacks)

    let windows = unsafe { CGSCopySpacesForWindows(connection, mask, re) };

    let count = unsafe { CFArrayGetCount(windows) };

    for i in 0..count {
        let value = unsafe { CFArrayGetValueAtIndex(windows, i as isize) as CFNumberRef };

        let mut result = 0;
        let result_ref: *mut i64 = &mut result;
        unsafe { CFNumberGetValue(value, kCFNumberSInt64Type, result_ref.cast()) };

        println!("{}", result);
    }
}

extern "C" {
    /*
     * CFDictionary.h
     */

    pub fn CGSMainConnectionID() -> CFIndex;

    pub fn CGSGetActiveSpace(connect: CFIndex) -> CFIndex;

    pub fn CGSCopyManagedDisplaySpaces(connect: CFIndex) -> CFArrayRef;

    pub fn CGSCopySpacesForWindows(
        cid: CFIndex,
        mask: CFOptionFlags,
        wids: CFArrayRef,
    ) -> CFArrayRef;

    // struct CGSWindowCaptureOptions: OptionSet {
    //     let rawValue: UInt32
    //     static let ignoreGlobalClipShape = CGSWindowCaptureOptions(rawValue: 1 << 11)
    //     // on a retina display, 1px is spread on 4px, so nominalResolution is 1/4 of bestResolution
    //     static let nominalResolution = CGSWindowCaptureOptions(rawValue: 1 << 9)
    //     static let bestResolution = CGSWindowCaptureOptions(rawValue: 1 << 8)
    // }

    // enum CGSSpaceMask: Int {
    //     case current = 5
    //     case other = 6
    //     case all = 7
    // }

    // pub fn CGSCopySpacesForWindows( cid: CGSConnectionID,  mask: CGSSpaceMask.RawValue,  wids: CFArrayRef) -> CFArrayRef;

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

// fn convert_num(value: CFNumberRef) -> i32 {
//     let type_id: CFTypeID = unsafe { CFGetTypeID(value) };
//     if type_id != unsafe { CFNumberGetTypeID() } {
//         return None;
//     }
//     let value = value as CFNumberRef;
//     match unsafe { CFNumberGetType(value) } {
//         I64 if I64 == kCFNumberSInt64Type => {
//             let mut result: i64 = 0;
//             let result_ref: *mut i64 = &mut result;
//             if unsafe { CFNumberGetValue(value, I64, result_ref.cast()) } {
//                 Some(result)
//             } else {
//                 None
//             }
//         }
//         I32 if I32 == kCFNumberSInt32Type => {
//             let mut result: i32 = 0;
//             let result_ref: *mut i32 = &mut result;
//             if unsafe { CFNumberGetValue(value, I32, result_ref.cast()) } {
//                 Some(result as i64)
//             } else {
//                 None
//             }
//         }
//         n => {
//             eprintln!("Unsupported Number of typeId: {}", n);
//             None
//         }
//     }
// }
