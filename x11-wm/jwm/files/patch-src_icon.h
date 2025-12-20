--- src/icon.h.orig	2025-12-19 19:01:13 UTC
+++ src/icon.h
@@ -15,8 +15,12 @@ typedef struct ScaledIconNode {
 /** Structure to hold a scaled icon. */
 typedef struct ScaledIconNode {
 
-   int width;   /**< The scaled width of the icon. */
-   int height;  /**< The scaled height of the icon. */
+   int width;           /**< The scaled width of the icon. */
+   int height;          /**< The scaled height of the icon. */
+#ifdef USE_XRENDER
+   int renderWidth;     /**< The width of the icon for xrender */
+   int renderHeight;    /**< The height of the icon for xrender */
+#endif
    long fg;     /**< Foreground color for bitmaps. */
 
    XID image;
