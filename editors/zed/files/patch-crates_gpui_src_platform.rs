--- crates/gpui/src/platform.rs.orig	2025-12-18 17:41:26 UTC
+++ crates/gpui/src/platform.rs
@@ -81,7 +81,7 @@ pub(crate) use windows::*;
 #[cfg(target_os = "windows")]
 pub(crate) use windows::*;
 
-#[cfg(all(target_os = "linux", feature = "wayland"))]
+#[cfg(all(any(target_os = "linux", target_os = "freebsd"), feature = "wayland"))]
 pub use linux::layer_shell;
 
 #[cfg(any(test, feature = "test-support"))]
@@ -1195,6 +1195,9 @@ pub struct WindowOptions {
     /// Note that this may be ignored.
     pub window_decorations: Option<WindowDecorations>,
 
+    /// Icon image (X11 only)
+    pub icon: Option<Arc<image::RgbaImage>>,
+
     /// Tab group name, allows opening the window as a native tab on macOS 10.12+. Windows with the same tabbing identifier will be grouped together.
     pub tabbing_identifier: Option<String>,
 }
@@ -1240,7 +1243,11 @@ pub(crate) struct WindowParams {
     #[cfg_attr(any(target_os = "linux", target_os = "freebsd"), allow(dead_code))]
     pub show: bool,
 
+    /// An image to set as the window icon (x11 only)
     #[cfg_attr(feature = "wayland", allow(dead_code))]
+    pub icon: Option<Arc<image::RgbaImage>>,
+
+    #[cfg_attr(feature = "wayland", allow(dead_code))]
     pub display_id: Option<DisplayId>,
 
     pub window_min_size: Option<Size<Pixels>>,
@@ -1300,6 +1307,7 @@ impl Default for WindowOptions {
             is_minimizable: true,
             display_id: None,
             window_background: WindowBackgroundAppearance::default(),
+            icon: None,
             app_id: None,
             window_min_size: None,
             window_decorations: None,
@@ -1337,7 +1345,7 @@ pub enum WindowKind {
 
     /// A Wayland LayerShell window, used to draw overlays or backgrounds for applications such as
     /// docks, notifications or wallpapers.
-    #[cfg(all(target_os = "linux", feature = "wayland"))]
+    #[cfg(all(any(target_os = "linux", target_os = "freebsd"), feature = "wayland"))]
     LayerShell(layer_shell::LayerShellOptions),
 }
 
