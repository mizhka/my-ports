--- src/icon.c.orig	2025-12-19 19:01:07 UTC
+++ src/icon.c
@@ -598,6 +598,10 @@ ScaledIconNode *GetScaledIcon(IconNode *icon, long fg,
          if(icon->render) {
             if(icon->images == NULL || icon->images->next == NULL) {
                return np;
+            } else {
+               if(np->renderWidth == nwidth && np->renderHeight == nheight) {
+                  return np;
+               }
             }
          }
 #endif
@@ -618,6 +622,8 @@ ScaledIconNode *GetScaledIcon(IconNode *icon, long fg,
    if(icon->render) {
       np = CreateScaledRenderIcon(imageNode, fg);
       np->next = icon->nodes;
+      np->renderWidth = nwidth;
+      np->renderHeight = nheight;
       icon->nodes = np;
 
       /* Don't keep the image data around after creating the icon. */
