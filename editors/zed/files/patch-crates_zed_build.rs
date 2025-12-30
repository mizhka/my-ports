--- crates/zed/build.rs.orig	2025-12-30 18:25:52 UTC
+++ crates/zed/build.rs
@@ -102,4 +102,49 @@ fn main() {
             std::process::exit(1);
         }
     }
+
+    #[cfg(any(target_os = "linux",target_os = "freebsd"))]
+    {
+        use image::{DynamicImage, ImageReader, ImageResult, imageops};
+        use std::env;
+        use std::path::Path;
+
+        let out_dir = env::var("OUT_DIR").unwrap();
+
+        let release_channel = option_env!("RELEASE_CHANNEL").unwrap_or("dev");
+        let icon = match release_channel {
+            "stable" => "resources/app-icon.png",
+            "preview" => "resources/app-icon-preview.png",
+            "nightly" => "resources/app-icon-nightly.png",
+            "dev" | _ => "resources/app-icon-dev.png",
+        };
+
+        let icon_src = Path::new(icon);
+
+        let resized_image = match || -> ImageResult<DynamicImage> {
+            Ok(ImageReader::open(icon_src)?.decode()?)
+        }() {
+            Err(msg) => {
+                eprintln!("failed to read or decode {}: {msg}", icon_src.display());
+                std::process::exit(1);
+            }
+            Ok(image) => imageops::resize(&image, 256, 256, imageops::FilterType::Lanczos3),
+        };
+
+        // name should match include_bytes! call in src/zed.rs
+        let icon_out_path = Path::new(&out_dir).join("app_icon.png");
+        resized_image.save(&icon_out_path).expect("saving app icon");
+
+        // verify icon can be read and decoded
+        if let Err(msg) = ImageReader::open(&icon_out_path).unwrap().decode() {
+            eprintln!(
+                "error verifying {}: {msg} (resized from {icon})",
+                icon_out_path.display(),
+            );
+            std::process::exit(1);
+        }
+
+        println!("cargo:rerun-if-env-changed=RELEASE_CHANNEL");
+        println!("cargo:rerun-if-changed={}", icon_src.to_string_lossy());
+    }
 }
