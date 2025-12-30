--- crates/gpui/src/window.rs.orig	2025-12-30 18:12:45 UTC
+++ crates/gpui/src/window.rs
@@ -1020,6 +1020,8 @@ impl Window {
             app_id,
             window_min_size,
             window_decorations,
+            #[cfg_attr(not(any(target_os = "linux", target_os = "freebsd")), allow(unused_variables))]
+            icon,
             #[cfg_attr(not(target_os = "macos"), allow(unused_variables))]
             tabbing_identifier,
         } = options;
@@ -1040,6 +1042,8 @@ impl Window {
                 show,
                 display_id,
                 window_min_size,
+                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
+                icon,
                 #[cfg(target_os = "macos")]
                 tabbing_identifier,
             },
