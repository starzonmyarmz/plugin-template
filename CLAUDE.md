# Plugin template

Read `README.md` first for the layout, the rename checklist, building,
tests and the editor. This file has what plugins made from the template
learned the hard way. History and reasoning live in the Keep Up idea
"VST/CLAP Plugin development idea and log" (`keepup idea show 54`). Add a
section there when a plugin is started or ships, and a "Gotchas from
<plugin>" list for anything new.

Plugins made from it so far: Halfwit (octave-divider bass splitter) and
Ghosted (drifting ADT/slapback). When something here is unclear, look at
how they did it.

## Starting a plugin

- Create the repo on GitHub from the template, so it's linked:
  `gh repo create starzonmyarmz/<name> --public --template starzonmyarmz/plugin-template`,
  then clone it. A repo can only get that link when it's created. If the
  command is blocked, ask the user to approve or run it rather than
  copying the template locally.
- Do the README's rename checklist as its own commit ("Rename the
  template to <Name>"), then grep for `example` and `Example` to catch
  anything it missed.
- Give the plugin its own `CLAUDE.md`, based on Halfwit's or Ghosted's:
  layout, where a new parameter has to go, golden renders, window size,
  screenshot and releases.

## Porting a JSFX

JSFX sources are in REAPER's `Effects` folder, or `~/Code/jsfx`.

1. Port the engine in f64, keeping the JSFX's sliders one for one in a
   `Settings` struct. If the front panel simplifies them, map knobs to
   `Settings` in a separate step, so the engine can still be compared
   with the JSFX.
2. Cross-check against a line-by-line Python transliteration of the
   JSFX, run over a real DI (`plugin/tests/di`), to within about 1e-8. If
   the JSFX calls `rand()`, use the same seeded RNG in both (Ghosted uses
   xorshift64*) and draw in the same order.
3. Lock it in with golden renders. Never commit rewritten golden files
   until the user has listened to them, and say in the commit what
   changed in the sound.
4. Then add what the JSFX lacks: glides on parameters that click, slew
   limits on modulated delay times (an unlimited glide can read a delay
   line backwards), and subnormal flushing.

## Things clap-validator catches

Run `scripts/clap-validator.sh` after any DSP change, not just in CI.

- Subnormal output. Recursive filters, envelopes and smoothers decay into
  subnormals in silence. Flush state and output below about 1e-30 to zero.
- nice-plug's `v2s_f32_hz_then_khz` already appends Hz/kHz, so adding
  `with_unit(" Hz")` too doubles it. `v2s_f32_gain_to_db` and
  `v2s_f32_percentage` don't append their unit, so `with_unit` is right
  with those.
- NaN from filters whose corner frequency is above Nyquist at low sample
  rates. Clamp corners below Nyquist.

## Design

Every plugin gets its own look, but they share an approach.

- Skeuomorphic: the editor is a physical object (Patina an OP-1-style
  synth, Halfwit a stompbox, Ghosted a vintage radio), with real
  materials, like powder coat, bakelite, walnut, brass and grille cloth,
  built in CSS gradients and shadows rather than images.
- Pick the object from the plugin's vibe and name, and let the theme
  run through everything: the wordmark, the art and the control labels.
  Names should be playful and capture the vibe.
- Restrained: one knob style and colour family per plugin, and few
  controls. Fold JSFX sliders into a handful of knobs with evocative
  names rather than exposing every setting.
- Colour and art must mean something. If two waves or knobs differ in
  colour, it's because they differ in function; otherwise make them
  the same. Art can follow parameters (Ghosted's ghosts fade with Haunt,
  the dial needle follows Distance), but don't fake audio metering.
- Labels are plain words a guitarist understands, not DSP terms or
  fieldset jargon. Use a tooltip value readout for numbers.
- Typography has character: embed a display face for the wordmark
  (Futura-like Jost, pillowy Modak), never a system or "commercial"
  font, and centre it optically on its badge.
- Illustration should look drawn, not vector-perfect: vary repeated
  elements (size, tilt, expression), and give outlines weight that
  swells on the shadow side, like inked comic art.
- Physical details behave physically: shadows stay put while a knob
  turns, pressed keys sink, lit things glow.
- Before building, ask about palette, the central object and what the
  art should react to, with previews. Show font candidates side by side
  in place before choosing one.

## Editor

- The window size is set in `PAGE.size` in `lib.rs` and by the page's
  outer element in `ui/index.html`. Change them together.
- Work on the page in a browser against the mock (see the README). The
  preview server gets cached: add `?v=N` after each edit. Keep the mock's
  `specs` in step with the plugin's parameters, including the skew of any
  skewed range.
- SVG reused through `<use>` doesn't get page CSS rules. To style its
  parts, stamp it from a `<template>` instead, or put attributes on the
  elements themselves.
- Fonts can't load from the network: embed them as base64 `@font-face`
  data URLs, and keep the file and its license in `ui/fonts`. For a
  wordmark, Google Fonts' css2 API with `&text=<glyphs>` returns a subset
  of a few KB.
- Before relying on a newer CSS feature, check it's in Safari's WebKit
  for the macOS versions you support. The macOS webview is the system
  WKWebView; Windows uses WebView2 (current Chromium).
- After a visible UI change, retake `docs/<name>.jpg`: capture the full
  window at device scale 2 with reduced motion, save it as a JPEG 852 px
  tall, and set the README `<img>` width to half the JPEG's width.

## Testing in REAPER

- `scripts/install.sh` installs into `~/Library/Audio/Plug-Ins`. REAPER
  keeps a plugin's dylib loaded, so restart it after reinstalling.
- REAPER only monitors input on armed tracks.
- Multi-output plugins: declare a second layout without the aux port so
  hosts without multi-out still load them, and keep a scratch buffer for
  the aux signal in that case (see Halfwit).
