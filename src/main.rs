use core_foundation::dictionary::CFDictionaryApplyFunction;
use core_foundation::{base::*, number::CFNumberGetValue};

use core_foundation::number::{kCFNumberLongType, CFNumber, CFNumberRef};
use core_foundation::string::*;
use core_graphics::display::*;
use std::ffi::{c_void, CStr};
mod private;
mod util;

#[derive(serde::Deserialize)]
struct Spaces {
    SpacesDisplayConfiguration: DisplayConfiguration,

    #[serde(rename = "spans-displays")]
    spans_displays: bool,
}

#[derive(serde::Deserialize, Debug)]
struct DisplayConfiguration {
    #[serde(rename = "Management Data")]
    management: plist::Dictionary,

    #[serde(rename = "Space Properties")]
    properties: Vec<DisplayData>,
}

#[derive(serde::Deserialize, Debug)]
struct DisplayData {
    name: String,
    windows: Vec<i32>,
}

fn main() {
    use plist::Value;

    let book: Spaces =
        plist::from_file("/Users/jonkelley/Library/Preferences/com.apple.spaces.plist")
            .expect("failed to read book.plist");

    for key in book.SpacesDisplayConfiguration.management.keys() {
        println!("{:?}", key);
    }

    for prop in &book.SpacesDisplayConfiguration.properties {
        println!("{:?}\n", prop);
    }

    println!(
        "there are {} spaces",
        book.SpacesDisplayConfiguration.properties.len()
    );
}

#[test]
fn get_window_name_from_id() {
    const OPTIONS: CGWindowListOption = kCGWindowListOptionAll
        | kCGWindowListExcludeDesktopElements
        | kCGWindowListOptionOnScreenOnly;

    let window_list_info = unsafe { CGWindowListCopyWindowInfo(OPTIONS, kCGNullWindowID) };
    let count = unsafe { CFArrayGetCount(window_list_info) };

    // for i in 0..1 {

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
