// let mut space_ids = vec![];
// {
//     let connection = unsafe { CGSMainConnectionID() };

//     let mask = 7;

//     let window_ids = result
//         .iter()
//         .map(|x| CFNumber::from(x.window_id as i32))
//         .collect::<Vec<CFNumber>>();

//     let windows_as_ref = window_ids.iter().map(|x| x.as_CFType()).collect::<Vec<_>>();

//     let array = CFArray::<CFType>::from_CFTypes(&windows_as_ref);
//     let re = array.as_concrete_TypeRef();

//     let windows = unsafe { CGSCopySpacesForWindows(connection, mask, re) };

//     let count = unsafe { CFArrayGetCount(windows) };

//     for i in 0..count {
//         let value = unsafe { CFArrayGetValueAtIndex(windows, i as isize) as CFNumberRef };

//         let mut result = 0;
//         let result_ref: *mut i64 = &mut result;
//         unsafe { CFNumberGetValue(value, kCFNumberIntType, result_ref.cast()) };

//         space_ids.push(result);
//     }

//     for (item, window) in space_ids.into_iter().zip(result.iter_mut()) {
//         window.space_id = item;
//     }
// }
