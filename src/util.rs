#![allow(non_snake_case)]
use core_foundation::dictionary::CFDictionaryApplyFunction;
use core_foundation::string::*;
use core_graphics::display::*;

use core_foundation::base::*;
use core_foundation::number::{
    kCFNumberIntType, kCFNumberSInt32Type, kCFNumberSInt64Type, CFNumber, CFNumberGetType,
    CFNumberGetTypeID, CFNumberGetValue, CFNumberRef,
};
use core_graphics::window::{
    kCGWindowLayer, kCGWindowName, kCGWindowNumber, kCGWindowOwnerName, kCGWindowOwnerPID,
};
use objc_foundation::{INSString, NSString};
use objc_id::Id;
use std::ffi::{c_void, CStr};
use std::ops::Deref;

use crate::private::{CGSCopySpacesForWindows, CGSMainConnectionID};

fn get_dict_string(dic_ref: CFDictionaryRef, key: CFStringRef) -> Option<String> {
    let mut value: *const c_void = std::ptr::null();
    if unsafe { CFDictionaryGetValueIfPresent(dic_ref, key.to_void(), &mut value) } != 0 {
        let type_id: CFTypeID = unsafe { CFGetTypeID(value) };
        if type_id != unsafe { CFStringGetTypeID() } {
            return None;
        }
        let c_ptr = unsafe { CFStringGetCStringPtr(value.cast(), kCFStringEncodingUTF8) };
        if c_ptr.is_null() {
            // Failed to read CFString. Try to read NSString.
            let nss: Id<NSString> = unsafe { Id::from_ptr(value as *mut NSString) };
            std::str::from_utf8(nss.deref().as_str().as_bytes())
                .map(|s| s.to_string())
                .ok()
        } else {
            let c_result = unsafe { CStr::from_ptr(c_ptr) };
            c_result.to_str().map(String::from).ok()
        }
    } else {
        None
    }
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

pub struct WindowName {
    pub app_name: String,
    pub win_name: String,
    pub window_id: i64,
    pub pid: i64,
    pub space_id: i64,
}

pub fn get_window_names() -> Vec<WindowName> {
    const OPTIONS: CGWindowListOption =
        kCGWindowListOptionAll | kCGWindowListExcludeDesktopElements;

    let window_list_info = unsafe { CGWindowListCopyWindowInfo(OPTIONS, kCGNullWindowID) };
    let count = unsafe { CFArrayGetCount(window_list_info) };

    let mut result = Vec::new();
    for i in 0..count {
        let dic_ref =
            unsafe { CFArrayGetValueAtIndex(window_list_info, i as isize) as CFDictionaryRef };
        let mut _owner_name: *const c_void = std::ptr::null();
        let mut _window_name: *const c_void = std::ptr::null();

        let app_name = match get_dict_string(dic_ref, unsafe { kCGWindowOwnerName }) {
            None => continue,
            Some(s) => s,
        };
        let win_name = match get_dict_string(dic_ref, unsafe { kCGWindowName }) {
            None => continue,
            Some(s) => s,
        };
        let layer = match get_dict_number(dic_ref, unsafe { kCGWindowLayer }) {
            None => continue,
            Some(s) => s,
        };
        let window_id = match get_dict_number(dic_ref, unsafe { kCGWindowNumber }) {
            None => continue,
            Some(s) => s,
        };
        let window_psn = match get_dict_number(dic_ref, unsafe { kCGWindowOwnerPID }) {
            None => continue,
            Some(s) => s,
        };
        if layer != 0 {
            continue;
        }
        result.push(WindowName {
            app_name,
            win_name,
            window_id,
            pid: window_psn,
            space_id: 0,
        });
    }

    let connection = unsafe { CGSMainConnectionID() };
    let mask = 7;

    for window in result.iter_mut() {
        window.space_id = {
            let window_id = CFNumber::from(window.window_id as i32);
            let window_id_ref = window_id.as_CFType();

            let array = CFArray::<CFType>::from_CFTypes(&[window_id_ref]);
            let re = array.as_concrete_TypeRef();

            let windows = unsafe { CGSCopySpacesForWindows(connection, mask, re) };

            let count = unsafe { CFArrayGetCount(windows) };

            let mut out = 0;
            for i in 0..count {
                let value = unsafe { CFArrayGetValueAtIndex(windows, i as isize) as CFNumberRef };

                let mut result = 0;
                let result_ref: *mut i64 = &mut result;
                unsafe { CFNumberGetValue(value, kCFNumberIntType, result_ref.cast()) };

                out = result;
            }
            out
        };
    }

    unsafe { CFRelease(window_list_info as CFTypeRef) }
    result
}

#[test]
fn get_window_name_from_id() {
    const OPTIONS: CGWindowListOption = kCGWindowListOptionAll
        | kCGWindowListExcludeDesktopElements
        | kCGWindowListOptionOnScreenOnly;

    let window_list_info = unsafe { CGWindowListCopyWindowInfo(OPTIONS, kCGNullWindowID) };
    let count = unsafe { CFArrayGetCount(window_list_info) };

    for i in 0..count {
        let dic_ref =
            unsafe { CFArrayGetValueAtIndex(window_list_info, i as isize) as CFDictionaryRef };

        // let key = CFString::new("kCGWindowNumber");
        // let mut value: *const c_void = std::ptr::null();

        // if unsafe { CFDictionaryGetValueIfPresent(dic_ref, key.to_void(), &mut value) != 0 } {
        //     let cf_ref = value as CFNumberRef;

        //     let mut number: *const c_void = std::ptr::null();
        //     let c_ptr = unsafe { CFNumberGetValue(cf_ref, kCFNumberLongType, number as *mut _) };

        //     // let c_ptr = unsafe { CFNumberGetValue(cf_ref, kCFStringEncodingUTF8) };
        //     if !number.is_null() {
        //         let c_result = unsafe { CFNumber::from_mut_void(number as *mut _) };

        //         // println!("window owner name: {}", result)
        //     }
        // }

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

        println!("\n");

        let key = CFString::new("kCGWindowOwnerName");
        let mut value: *const c_void = std::ptr::null();

        if unsafe { CFDictionaryGetValueIfPresent(dic_ref, key.to_void(), &mut value) != 0 } {
            let cf_ref = value as CFStringRef;
            let c_ptr = unsafe { CFStringGetCStringPtr(cf_ref, kCFStringEncodingUTF8) };
            if !c_ptr.is_null() {
                let c_result = unsafe { CStr::from_ptr(c_ptr) };
                let result = String::from(c_result.to_str().unwrap());
                println!("window owner name: {}", result)
            }
        }
    }

    unsafe { CFRelease(window_list_info as CFTypeRef) }
}

#[test]
fn print_names() {
    let windows = get_window_names();
    for window in windows {
        if window.space_id != 0 && !window.win_name.is_empty() {
            println!(
                "{}[{}:{}:{}]: {}",
                window.app_name, window.window_id, window.space_id, window.pid, window.win_name
            );
        }

        if window.app_name == "Spotify" {}
    }
}
