--- crates/zed/src/zed.rs.orig	2025-12-18 17:41:26 UTC
+++ crates/zed/src/zed.rs
@@ -316,6 +316,21 @@ pub fn build_window_options(display_uuid: Option<Uuid>
 
     let use_system_window_tabs = WorkspaceSettings::get_global(cx).use_system_window_tabs;
 
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
+    static APP_ICON: std::sync::LazyLock<Option<std::sync::Arc<image::RgbaImage>>> =
+        std::sync::LazyLock::new(|| {
+            // this shouldn't fail since decode is checked in build.rs
+            const BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/app_icon.png"));
+            util::maybe!({
+                let image = image::ImageReader::new(std::io::Cursor::new(BYTES))
+                    .with_guessed_format()?
+                    .decode()?
+                    .into();
+                anyhow::Ok(Arc::new(image))
+            })
+            .log_err()
+        });
+
     WindowOptions {
         titlebar: Some(TitlebarOptions {
             title: None,
@@ -330,6 +345,8 @@ pub fn build_window_options(display_uuid: Option<Uuid>
         display_id: display.map(|display| display.id()),
         window_background: cx.theme().window_background_appearance(),
         app_id: Some(app_id.to_owned()),
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
+        icon: APP_ICON.as_ref().cloned(),
         window_decorations: Some(window_decorations),
         window_min_size: Some(gpui::Size {
             width: px(360.0),
@@ -385,6 +402,7 @@ pub fn initialize_workspace(
         if let Some(specs) = window.gpu_specs() {
             log::info!("Using GPU: {:?}", specs);
             show_software_emulation_warning_if_needed(specs.clone(), window, cx);
+            #[cfg(not(target_os = "freebsd"))]
             if let Some((crash_server, message)) = crashes::CRASH_HANDLER
                 .get()
                 .zip(bincode::serialize(&specs).ok())
