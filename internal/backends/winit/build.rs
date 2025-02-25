// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-1.1 OR LicenseRef-Slint-commercial

use cfg_aliases::cfg_aliases;

fn main() {
    // Setup cfg aliases
    cfg_aliases! {
       enable_skia_renderer: { any(feature = "renderer-winit-skia", feature = "renderer-winit-skia-opengl", feature = "renderer-winit-skia-vulkan")},
       enable_accesskit: { all(feature = "accessibility", not(target_arch = "wasm32")) },
    }
}
