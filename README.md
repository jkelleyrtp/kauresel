# Karusel: TotalSpaces/esque space management for macOS

This project is not done! It doesn't even work!

Its goal is to be a more maintainable flavor of alt-tab that provides switching between virtual desktops instead of apps. I tend to organize my work spatially, and macOS lack of spatial switching (in preference to app-based switching) is very confusing.

To improve maintainability and ensure that anyone can contribute and build the project, it is written in Rust, using the core-foundation and core-graphics crates directly.

## General design

- Rust
- CoreFoundation/CoreGraphics
- Private CGS APIs
- Dioxus for UI
- Tauri for bundling and updating

We've got all the components - I just need to figure out how to stitch them all together properly.

## Research

accessibility/accessibility-sys/src at master · eiz/accessibility
https://github.com/eiz/accessibility/tree/master/accessibility-sys/src

eiz/accessibility: Rust bindings for macOS accessibility API.
https://github.com/eiz/accessibility

objective c - Move other windows on Mac OS X using Accessibility API - Stack Overflow
https://stackoverflow.com/questions/21069066/move-other-windows-on-mac-os-x-using-accessibility-api

CGWindowListCopyWindowInfo - Google Search
https://www.google.com/search?client=safari&rls=en&q=CGWindowListCopyWindowInfo&ie=UTF-8&oe=UTF-8

CGWindowListOption | Apple Developer Documentation
https://developer.apple.com/documentation/coregraphics/cgwindowlistoption

Apple Developer Documentation
https://developer.apple.com/documentation/coregraphics/kcgwindowownername

macos - How can I programmatically add a space to mission control? - Stack Overflow
https://stackoverflow.com/questions/9606221/how-can-i-programmatically-add-a-space-to-mission-control

alt-tab-macos/Contributing.md at master · lwouis/alt-tab-macos
https://github.com/lwouis/alt-tab-macos/blob/master/docs/Contributing.md

enigo-rs/enigo: Cross platform input simulation in Rust
https://github.com/enigo-rs/enigo

macos - Getting window owner names via CGWindowListCopyWindowInfo in Rust - Stack Overflow
https://stackoverflow.com/questions/60117318/getting-window-owner-names-via-cgwindowlistcopywindowinfo-in-rust

tao/global_shortcut.rs at dev · tauri-apps/tao
https://github.com/tauri-apps/tao/blob/dev/examples/global_shortcut.rs

uBar - The Dock replacement for the Mac
https://brawersoftware.com/products/ubar

lwouis/alt-tab-macos: Windows alt-tab on macOS
https://github.com/lwouis/alt-tab-macos

Display windows from other Spaces · Issue #14 · lwouis/alt-tab-macos
https://github.com/lwouis/alt-tab-macos/issues/14

window:focus() focuses different window of app on same screen · Issue #370 · Hammerspoon/hammerspoon
https://github.com/Hammerspoon/hammerspoon/issues/370

phoenix/PHSpace.m at master · kasper/phoenix
https://github.com/kasper/phoenix/blob/master/Phoenix/PHSpace.m

A Native Art Gallery for Your Mac (Take 2) — Archagon Was Here
http://archagon.net/blog/2018/05/02/a-native-art-gallery-for-your-mac/

kasper/phoenix: A lightweight macOS window and app manager scriptable with JavaScript
https://github.com/kasper/phoenix

Windows on a space (using private CGS)
https://gist.github.com/sdsykes/5c2c0c2a41396aead3b7

user interface - AppleScript: Get list of windows on all desktops - Stack Overflow
https://stackoverflow.com/questions/20554602/applescript-get-list-of-windows-on-all-desktops

alt-tab-macos/PrivateApis.swift at master · lwouis/alt-tab-macos
https://github.com/lwouis/alt-tab-macos/blob/master/src/api-wrappers/PrivateApis.swift

CGSInternal/CGSWindow.h at master · NUIKit/CGSInternal
https://github.com/NUIKit/CGSInternal/blob/master/CGSWindow.h

"Move to window to space" feature leaves a copy of the window on the old space · Issue #1174 · ianyh/Amethyst
https://github.com/ianyh/Amethyst/issues/1174

Advances in macOS Security - WWDC19 - Videos - Apple Developer
https://developer.apple.com/videos/play/wwdc2019/701/

window name not available in macOS… | Apple Developer Forums
https://developer.apple.com/forums/thread/126860

Identifying Spaces in Mac OS X | ianyh
https://ianyh.com/blog/identifying-spaces-in-mac-os-x/

C++ (Cpp) CGWindowListCopyWindowInfo Examples - HotExamples
https://cpp.hotexamples.com/examples/-/-/CGWindowListCopyWindowInfo/cpp-cgwindowlistcopywindowinfo-function-examples.html

ios - How to enumerate CFProperyList / CFDictionary keys - Stack Overflow
https://stackoverflow.com/questions/2283466/how-to-enumerate-cfproperylist-cfdictionary-keys

alfwin/windows.rs at 2e9262c266159b32364681c7ef65061b7af8b1ff · kurtbuilds/alfwin
https://github.com/kurtbuilds/alfwin/blob/2e9262c266159b32364681c7ef65061b7af8b1ff/src/windows.rs

objective c - CGWindowListCopyWindowInfo returns info with kCGWindowSharingState set to 0 - Stack Overflow
https://stackoverflow.com/questions/62167614/cgwindowlistcopywindowinfo-returns-info-with-kcgwindowsharingstate-set-to-0

objective c - Front most window using CGWindowListCopyWindowInfo - Stack Overflow
https://stackoverflow.com/questions/5286274/front-most-window-using-cgwindowlistcopywindowinfo

swift - Activate a window using its Window ID - Stack Overflow
https://stackoverflow.com/questions/47152551/activate-a-window-using-its-window-id

