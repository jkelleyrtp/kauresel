use std::ffi::{c_void, CStr};
use std::os::raw::c_uint;

use accessibility_sys::{
    kAXRaiseAction, kAXTitleAttribute, kAXWindowsAttribute, AXError, AXUIElementCopyAttributeValue,
    AXUIElementCopyAttributeValues, AXUIElementCreateApplication, AXUIElementCreateSystemWide,
    AXUIElementPerformAction, AXUIElementRef,
};
use core_foundation::array::{
    CFArray, CFArrayCreate, CFArrayGetCount, CFArrayGetValueAtIndex, CFArrayRef,
};
use core_foundation::base::*;
use core_foundation::dictionary::{CFDictionary, CFDictionaryApplyFunction, CFDictionaryRef};
use core_foundation::error::{CFError, CFErrorCopyDescription, CFErrorRef};
use core_foundation::number::{
    kCFNumberIntType, kCFNumberSInt32Type, kCFNumberSInt64Type, CFNumber, CFNumberGetType,
    CFNumberGetTypeID, CFNumberGetValue, CFNumberRef,
};
use core_foundation::string::{
    kCFStringEncodingUTF8, CFString, CFStringGetCStringPtr, CFStringRef,
};
use core_graphics::window::{CGWindowID, CGWindowListCreateDescriptionFromArray};

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

    let mask = 7;

    let n0 = CFNumber::from(57147_i32);
    let n1 = CFNumber::from(56487_i32);

    let array = CFArray::<CFType>::from_CFTypes(&[n0.as_CFType(), n1.as_CFType()]);
    let re = array.as_concrete_TypeRef();

    let windows = unsafe { CGSCopySpacesForWindows(connection, mask, re) };

    let count = unsafe { CFArrayGetCount(windows) };

    for i in 0..count {
        let value = unsafe { CFArrayGetValueAtIndex(windows, i as isize) as CFNumberRef };

        let mut result = 0;
        let result_ref: *mut i64 = &mut result;
        unsafe { CFNumberGetValue(value, kCFNumberIntType, result_ref.cast()) };

        println!("{}", result);
    }
}

type UInt32 = c_uint;

fn make_key_window() {

    // uint8_t bytes1[0xf8] = {
    //     [0x04] = 0xF8,
    //     [0x08] = 0x01,
    //     [0x3a] = 0x10
    // };

    // uint8_t bytes2[0xf8] = {
    //     [0x04] = 0xF8,
    //     [0x08] = 0x02,
    //     [0x3a] = 0x10
    // };

    // memcpy(bytes1 + 0x3c, &window_id, sizeof(uint32_t));
    // memset(bytes1 + 0x20, 0xFF, 0x10);
    // memcpy(bytes2 + 0x3c, &window_id, sizeof(uint32_t));
    // memset(bytes2 + 0x20, 0xFF, 0x10);
    // SLPSPostEventRecordTo(window_psn, bytes1);
    // SLPSPostEventRecordTo(window_psn, bytes2);
}

/// Type for unique process identifier.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ProcessSerialNumber {
    padding: u32,
    val: u32,
}

// pub type ProcessSerialNumberPtr = *mut ProcessSerialNumber;

#[test]
pub fn focus_window() {
    // 23468:9:463

    // let wid: u32 = 55440;
    // let pid = 7020;

    let wid: u32 = 52305;
    // let wid: u32 = 23468;
    let pid = 463;

    let mut psn = ProcessSerialNumber { padding: 0, val: 0 };
    unsafe { GetProcessForPID(pid, &mut psn) };

    let mut bytes1 = [0; 0xf8];
    bytes1[0x04] = 0xF8;
    bytes1[0x08] = 0x01;
    bytes1[0x3a] = 0x10;

    let mut bytes2 = [0; 0xf8];
    bytes2[0x04] = 0xF8;
    bytes2[0x08] = 0x02;
    bytes2[0x3a] = 0x10;

    bytes1[0x3c..(0x3c + 4)].copy_from_slice(&wid.to_le_bytes());
    bytes1[0x20..(0x20 + 0x10)].fill(0xFF);

    bytes2[0x3c..0x3c + 4].copy_from_slice(&wid.to_le_bytes());
    bytes2[0x20..(0x20 + 0x10)].fill(0xFF);

    unsafe {
        _SLPSSetFrontProcessWithOptions(&mut psn, wid, 0x400);
        let e1 = SLPSPostEventRecordTo(&mut psn, &bytes1);
        let e2 = SLPSPostEventRecordTo(&mut psn, &bytes2);
    }

    let app_ref = unsafe { AXUIElementCreateApplication(pid as i32) };
    let mut window_list_ref = std::ptr::null();
    unsafe {
        AXUIElementCopyAttributeValues(
            app_ref,
            CFString::new(kAXWindowsAttribute).as_concrete_TypeRef(),
            0,
            9999999,
            &mut window_list_ref,
        )
    };

    if !window_list_ref.is_null() {
        let window_count = unsafe { CFArrayGetCount(window_list_ref) };
        dbg!(window_count);

        for i in 0..window_count {
            let mut window_id: u32 = 0;

            let window_ref =
                unsafe { CFArrayGetValueAtIndex(window_list_ref, i as isize) as AXUIElementRef };

            unsafe { _AXUIElementGetWindow(window_ref, &mut window_id) };

            if window_id == wid {
                println!("window found, now move the space");
                std::thread::sleep(std::time::Duration::from_millis(5000));
                unsafe {
                    AXUIElementPerformAction(
                        window_ref,
                        CFString::new("kAXRaiseAction").as_concrete_TypeRef(),
                    )
                };

                println!("Found matching window");
            }

            println!("Window: {}", window_id);
        }
    } else {
        println!("Window list is null");
    }

    // TODO: need to get the AXUIElementRef for the window we want to focus

    // let e3 = AXUIElementPerformAction(
    //     ,
    //     CFString::new("kAXRaiseAction").as_concrete_TypeRef(),
    // );
    // }
}

#[test]
fn create_space() {
    let connection = unsafe { CGSMainConnectionID() };

    let null_ptr: *mut c_void = std::ptr::null() as *const c_void as *mut _;

    let opts = CFDictionary::<CFString, CFString>::from_CFType_pairs(&[]);
    // CFString::new("CGSSystemProcessType").as_concrete_TypeRef(),
    // CFNumber::from(0).as_CFType(),
    // CFString::new("CGSSystemProcessType").as_concrete_TypeRef(),
    // CFNumber::from(1).as_CFType(),
    let err = unsafe { CGSSpaceCreate(connection, null_ptr, opts.as_concrete_TypeRef()) };

    println!("{}", err);
}

#[test]
fn get_psn() {
    let pid = 7020;
    let mut psn = ProcessSerialNumber { padding: 0, val: 0 };
    unsafe { GetProcessForPID(pid, &mut psn) };
    println!("{:?}", psn);
}

#[test]
fn focus_space() {
    //

    let connection = unsafe { CGSMainConnectionID() };

    let s = CFString::new("951C832C-A39B-40D9-86B9-F37B79180E2F");
    let sid = 27;

    unsafe { CGSManagedDisplaySetCurrentSpace(connection, s.as_concrete_TypeRef(), sid) };
}

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    pub fn _AXUIElementGetWindow(el: AXUIElementRef, id: &mut CGWindowID) -> AXError;
}

#[link(name = "SkyLight", kind = "framework")]
extern "C" {
    fn _SLPSSetFrontProcessWithOptions(psn: *mut ProcessSerialNumber, wid: u32, mode: u32);

    fn SLSGetConnectionPSN(
        connection: u32,
        psn: *mut ProcessSerialNumber,
    ) -> core_graphics::base::CGError;

    fn SLPSPostEventRecordTo(
        psn: *mut ProcessSerialNumber,
        event: &[u8],
    ) -> core_graphics::base::CGError;

    fn GetProcessForPID(pid: i32, psn: *mut ProcessSerialNumber);
}

// extern CGError SLPSPostEventRecordTo(ProcessSerialNumber *psn, uint8_t *bytes);

extern "C" {
    /*
     * CFDictionary.h
     */

    // CG_EXTERN void CGSManagedDisplaySetCurrentSpace(CGSConnectionID cid, CFStringRef display, CGSSpaceID space);

    pub fn CGSSpaceCreate(connect: u32, null_ptr: *mut c_void, options: CFDictionaryRef) -> u32;

    pub fn CGSMainConnectionID() -> u32;

    pub fn CGSGetActiveSpace(connect: u32) -> CFIndex;

    pub fn CGSCopyManagedDisplaySpaces(connect: u32) -> CFArrayRef;

    pub fn CGSCopySpacesForWindows(cid: u32, mask: CFOptionFlags, wids: CFArrayRef) -> CFArrayRef;

    pub fn CGSManagedDisplaySetCurrentSpace(
        cid: u32,
        // cid: CGSConnectionID,
        display: CFStringRef,
        // display: CFString,
        sid: u32,
    );
    // pub fn CGSManagedDisplaySetCurrentSpace(
    //     cid: CGSConnectionID,
    //     display: CFString,
    //     sid: CGSSpaceID,
    // );

    // -> OSStatus;
    // fn _SLPSSetFrontProcessWithOptions(psn: inout ProcessSerialNumber, _ wid: CGWindowID, _ mode: SLPSMode) -> CGError

    // enum SLPSMode: UInt32 {
    //     case allWindows = 0x100
    //     case userGenerated = 0x200
    //     case noWindows = 0x400
    // }

    // func _SLPSSetFrontProcessWithOptions(_ psn: inout ProcessSerialNumber, _ wid: CGWindowID, _ mode: SLPSMode) -> CGError

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
