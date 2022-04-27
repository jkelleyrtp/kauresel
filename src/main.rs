mod plist_parsing;
mod private;
mod util;

mod tao_helper;

use std::collections::HashMap;

use cocoa::appkit::{
    NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
    NSApplicationActivationPolicyRegular, NSBackingStoreBuffered, NSMenu, NSMenuItem,
    NSRunningApplication, NSWindow, NSWindowStyleMask,
};
use cocoa::base::{nil, selector, NO};
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSProcessInfo, NSRect, NSSize, NSString};

fn main() {
    use std::{str::FromStr, sync::mpsc};
    use tao::{
        accelerator::{Accelerator, AcceleratorId, RawMods, SysMods},
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        global_shortcut::ShortcutManager,
        keyboard::KeyCode,
        window::WindowBuilder,
    };

    let event_loop = EventLoop::new();

    // create new shortcut manager instance
    let mut hotkey_manager = ShortcutManager::new(&event_loop);

    // create our accelerators
    let shortcut_1 = Accelerator::new(SysMods::Shift, KeyCode::ArrowUp);
    let shortcut_2 = Accelerator::new(RawMods::AltCtrlMeta, KeyCode::KeyB);
    // use string parser to generate accelerator (require `std::str::FromStr`)
    let shortcut_3 = Accelerator::from_str("COMMAND+`").unwrap();
    let shortcut_4 = Accelerator::from_str("COMMAND+SHIFT+2").unwrap();

    // save a reference to unregister it later
    let global_shortcut_1 = hotkey_manager.register(shortcut_1.clone()).unwrap();
    // register other accelerator's
    hotkey_manager.register(shortcut_2.clone()).unwrap();
    hotkey_manager.register(shortcut_3).unwrap();
    hotkey_manager.register(shortcut_4.clone()).unwrap();

    let mut windows = Vec::new();
    let mut scroll_id = 0;

    event_loop.run(move |event, target, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
                ..
            } => {
                //
                // if window_id == window.id() => *control_flow = ControlFlow::Exit
            }
            Event::MainEventsCleared => {
                // window.request_redraw();
            }
            Event::GlobalShortcutEvent(hotkey_id) if hotkey_id == shortcut_1.clone().id() => {
                println!("Pressed `shortcut_1` -- unregister for future use");
                // unregister key
                hotkey_manager
                    .unregister(global_shortcut_1.clone())
                    .unwrap();
            }
            Event::GlobalShortcutEvent(hotkey_id) if hotkey_id == shortcut_2.clone().id() => {
                println!("Pressed on `shortcut_2`");
            }
            // you can match hotkey_id with accelerator_string only if you used `from_str`
            // by example `shortcut_1` will NOT match AcceleratorId::new("SHIFT+UP") as it's
            // been created with a struct and the ID is generated automatically
            Event::GlobalShortcutEvent(hotkey_id)
                if hotkey_id == AcceleratorId::new("COMMANDORCONTROL+SHIFT+3") =>
            {
                println!("Pressed on `shortcut_3`");
            }
            Event::GlobalShortcutEvent(hotkey_id) if hotkey_id == shortcut_4.clone().id() => {
                let window = WindowBuilder::new()
                    .with_title("A fantastic window!")
                    .build(target)
                    .unwrap();

                let id = window.id();
                windows.push((id, window));
                //
            }

            Event::GlobalShortcutEvent(hotkey_id) => {
                scroll_id += 1;
                if scroll_id >= windows.len() {
                    scroll_id = 0;
                }

                let (id, window) = &mut windows[scroll_id];

                println!("hotkey_id {:?}", hotkey_id);
                use tao::platform::macos::WindowExtMacOS;

                let id = window.ns_window();
                let raw_window = id as *mut objc::runtime::Object;

                unsafe {
                    raw_window.makeKeyAndOrderFront_(nil);
                }

                let app = unsafe { NSApp() };
                unsafe { app.activateIgnoringOtherApps_(true) };
            }
            _ => (),
        }
    });
}
