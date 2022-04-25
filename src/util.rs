use core_foundation::string::*;
use core_graphics::display::*;

use core_foundation::base::*;
use core_foundation::number::{
    kCFNumberSInt32Type, kCFNumberSInt64Type, CFNumberGetType, CFNumberGetTypeID, CFNumberGetValue,
    CFNumberRef,
};
use core_graphics::window::{kCGWindowLayer, kCGWindowName, kCGWindowOwnerName};
use objc_foundation::{INSString, NSString};
use objc_id::Id;
use std::ffi::{c_void, CStr};
use std::ops::Deref;

fn get_dict_string(dic_ref: CFDictionaryRef) -> Option<String> {
    // let mut value: *const c_void = std::ptr::null();

    let key = CFString::new("kCGWindowNumber");
    let mut value: *const c_void = std::ptr::null();

    if unsafe { CFDictionaryGetValueIfPresent(dic_ref, key.to_void(), &mut value) != 0 } {
        let cf_ref = value as CFStringRef;
        let c_ptr = unsafe { CFStringGetCStringPtr(cf_ref, kCFStringEncodingUTF8) };
        if !c_ptr.is_null() {
            let c_result = unsafe { CStr::from_ptr(c_ptr) };
            let result = String::from(c_result.to_str().unwrap());
            return Some(result.clone());
        }
    }

    None
    // if unsafe { CFDictionaryGetValueIfPresent(dic_ref, key.to_void(), &mut value) } != 0 {
    //     let type_id: CFTypeID = unsafe { CFGetTypeID(value) };
    //     if type_id != unsafe { CFStringGetTypeID() } {
    //         return None;
    //     }
    //     let c_ptr = unsafe { CFStringGetCStringPtr(value.cast(), kCFStringEncodingUTF8) };
    //     if c_ptr.is_null() {
    //         // Failed to read CFString. Try to read NSString.
    //         let nss: Id<NSString> = unsafe { Id::from_ptr(value as *mut NSString) };
    //         std::str::from_utf8(nss.deref().as_str().as_bytes())
    //             .map(|s| s.to_string())
    //             .ok()
    //     } else {
    //         let c_result = unsafe { CStr::from_ptr(c_ptr) };
    //         c_result.to_str().map(String::from).ok()
    //     }
    // } else {
    //     None
    // }
}

fn get_dict_number(dic_ref: CFDictionaryRef, key: CFStringRef) -> Option<i64> {
    let mut value: *const c_void = std::ptr::null();
    if unsafe { CFDictionaryGetValueIfPresent(dic_ref, key.to_void(), &mut value) } != 0 {
        let type_id: CFTypeID = unsafe { CFGetTypeID(value) };
        if type_id != unsafe { CFNumberGetTypeID() } {
            return None;
        }
        let value = value as CFNumberRef;
        match unsafe { CFNumberGetType(value) } {
            I64 if I64 == kCFNumberSInt64Type => {
                let mut result: i64 = 0;
                let result_ref: *mut i64 = &mut result;
                if unsafe { CFNumberGetValue(value, I64, result_ref.cast()) } {
                    Some(result)
                } else {
                    None
                }
            }
            I32 if I32 == kCFNumberSInt32Type => {
                let mut result: i32 = 0;
                let result_ref: *mut i32 = &mut result;
                if unsafe { CFNumberGetValue(value, I32, result_ref.cast()) } {
                    Some(result as i64)
                } else {
                    None
                }
            }
            n => {
                eprintln!("Unsupported Number of typeId: {}", n);
                None
            }
        }
    } else {
        None
    }
}

#[derive(Debug)]
pub struct WindowName {
    pub app_name: String,
    pub win_name: String,
}

pub fn get_window_names() -> Vec<WindowName> {
    const OPTIONS: CGWindowListOption =
        kCGWindowListOptionAll | kCGWindowListExcludeDesktopElements;

    // const OPTIONS: CGWindowListOption = kCGWindowListOptionOnScreenOnly;

    let window_list_info = unsafe { CGWindowListCopyWindowInfo(OPTIONS, kCGNullWindowID) };
    let count = unsafe { CFArrayGetCount(window_list_info) };

    let mut result = Vec::new();

    for i in 0..count {
        let dic_ref =
            unsafe { CFArrayGetValueAtIndex(window_list_info, i as isize) as CFDictionaryRef };

        // let key = CFString::new("kCGWindowOwnerName");
        // let mut value: *const c_void = std::ptr::null();

        // if unsafe { CFDictionaryGetValueIfPresent(dic_ref, key.to_void(), &mut value) != 0 } {
        //     let cf_ref = value as CFStringRef;
        //     let c_ptr = unsafe { CFStringGetCStringPtr(cf_ref, kCFStringEncodingUTF8) };
        //     if !c_ptr.is_null() {
        //         let c_result = unsafe { CStr::from_ptr(c_ptr) };
        //         let result = String::from(c_result.to_str().unwrap());
        //         println!("window owner name: {}", result)
        //     }
        // }

        let key = CFString::new("kCGWindowName");
        let mut value: *const c_void = std::ptr::null();

        if unsafe { CFDictionaryGetValueIfPresent(dic_ref, key.to_void(), &mut value) != 0 } {
            let cf_ref = value as CFStringRef;
            let c_ptr = unsafe { CFStringGetCStringPtr(cf_ref, kCFStringEncodingUTF8) };
            if !c_ptr.is_null() {
                let c_result = unsafe { CStr::from_ptr(c_ptr) };
                let result = String::from(c_result.to_str().unwrap());
                println!("name: {}", result)
            }
        } else {
            println!("no we messed it up");
        }

        // let mut _owner_name: *const c_void = std::ptr::null();
        // let mut _window_name: *const c_void = std::ptr::null();

        // let app_name = match get_dict_string(dic_ref) {
        //     // let app_name = match get_dict_string(dic_ref, unsafe { kCGWindowOwnerName }) {
        //     None => {
        //         eprintln!("Failed to get app name");
        //         continue;
        //     }
        //     Some(s) => s,
        // };

        // println!("{}", app_name);

        // let win_name = match get_dict_string(dic_ref, unsafe { kCGWindowName }) {
        //     None => {
        //         eprintln!("Failed to get window name");
        //         continue;
        //     }
        //     Some(s) => s,
        // };
        // let layer = match get_dict_number(dic_ref, unsafe { kCGWindowLayer }) {
        //     None => {
        //         eprintln!("Failed to get window layer");
        //         continue;
        //     }
        //     Some(s) => s,
        // };
        // if layer != 0 {
        //     continue;
        // }
        // result.push(WindowName { app_name, win_name });
    }

    unsafe { CFRelease(window_list_info as CFTypeRef) }

    result
}

#[test]
fn get_names() {
    let names = get_window_names();

    dbg!(names);
}
