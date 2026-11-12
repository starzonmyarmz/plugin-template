---
name: screenshot
description: Retake the plugin's README screenshot (docs/<name>.jpg) after a visible change to the editor in ui/index.html. Use after any UI change you'd see in the window, or when asked for a new screenshot.
---

# Retake the screenshot

The README shows `docs/<name>.jpg`, where `<name>` is the crate name. Retake
it whenever a change to `ui/index.html` would show in the window.

1. Serve the page against its mock bridge:
   `cd ui && python3 -m http.server 8765`. Check the mock's `specs` match the
   plugin's parameters and defaults first, so the shot shows the real
   controls at their defaults.
2. With Playwright, open a context at device scale factor 2 with reduced
   motion, and a viewport of `PAGE.size` from `plugin/src/lib.rs`. The MCP
   tools can't set those, so use `browser_run_code_unsafe`:

   ```js
   const ctx = await page.context().browser().newContext({
     viewport: { width: W, height: H },
     deviceScaleFactor: 2,
     reducedMotion: "reduce",
   });
   const p = await ctx.newPage();
   await p.goto("http://127.0.0.1:8765/?v=" + Date.now());
   await p.waitForLoadState("networkidle");
   await p.screenshot({ path: "<abs path>/docs/<name>.png" });
   await ctx.close();
   ```

   Capture the whole window, not one element, so nothing gets cropped.
3. Look at the PNG. If fonts haven't loaded, a knob is mid-glide or the
   mock's values are off, fix that and retake it.
4. Convert it to a JPEG 852 px tall and remove the PNG:

   ```sh
   sips -s format jpeg -s formatOptions 85 --resampleHeight 852 docs/<name>.png --out docs/<name>.jpg
   rm docs/<name>.png
   ```

5. Set the README `<img>` `width` to half the JPEG's pixel width
   (`sips -g pixelWidth docs/<name>.jpg`), and keep its `alt` describing
   what's on the panel, including any new control.
6. Commit the image with the UI change it shows, not on its own.
