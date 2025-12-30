--- crates/gpui/src/platform/linux/x11/window.rs.orig	2025-12-30 18:10:05 UTC
+++ crates/gpui/src/platform/linux/x11/window.rs
@@ -11,6 +11,7 @@ use blade_graphics as gpu;
 };
 
 use blade_graphics as gpu;
+use image::Pixel as _;
 use raw_window_handle as rwh;
 use util::{ResultExt, maybe};
 use x11rb::{
@@ -60,6 +61,7 @@ x11rb::atom_manager! {
         WM_TRANSIENT_FOR,
         _NET_WM_PID,
         _NET_WM_NAME,
+        _NET_WM_ICON,
         _NET_WM_STATE,
         _NET_WM_STATE_MAXIMIZED_VERT,
         _NET_WM_STATE_MAXIMIZED_HORZ,
@@ -502,6 +504,29 @@ impl X11WindowState {
                         )
                     },
                     size_hints.set_normal_hints(xcb, x_window),
+                )?;
+            }
+
+            if let Some(image) = params.icon {
+                // https://specifications.freedesktop.org/wm-spec/1.4/ar01s05.html#id-1.6.13
+                let property_size = 2 + (image.width() * image.height()) as usize;
+                let mut property_data: Vec<u32> = Vec::with_capacity(property_size);
+                property_data.push(image.width());
+                property_data.push(image.height());
+                property_data.extend(image.pixels().map(|px| {
+                    let [r, g, b, a]: [u8; 4] = px.to_rgba().0;
+                    u32::from_le_bytes([b, g, r, a])
+                }));
+
+                check_reply(
+                    || "X11 ChangeProperty32 for _NET_ICON_NAME failed.",
+                    xcb.change_property32(
+                        xproto::PropMode::REPLACE,
+                        x_window,
+                        atoms._NET_WM_ICON,
+                        xproto::AtomEnum::CARDINAL,
+                        &property_data,
+                    ),
                 )?;
             }
 
