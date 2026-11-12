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

## Skills

`.claude/skills` has the routines plugins repeat. Use them rather than
working from memory:

- `design-kickoff`: settle the look before building the editor.
- `screenshot`: retake `docs/<name>.jpg` after a visible UI change.
- `port-jsfx`: port a JSFX prototype, cross-checked and locked in.
- `bless-goldens`: rewrite golden renders, and have the user listen
  before committing.

If a plugin is ported from a JSFX, never mention the JSFX in its repo:
not in the README, CLAUDE.md, code, comments, test names or parameter
tables, and not in commit messages.

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

Follow the README's design principles, and use the `design-kickoff`
skill before building. Effects get the footswitch unless the user says
otherwise.

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
- After a visible UI change, retake `docs/<name>.jpg` with the
  `screenshot` skill.

## Testing in REAPER

- `scripts/install.sh` installs into `~/Library/Audio/Plug-Ins`. REAPER
  keeps a plugin's dylib loaded, so restart it after reinstalling.
- REAPER only monitors input on armed tracks.
- Multi-output plugins: declare a second layout without the aux port so
  hosts without multi-out still load them, and keep a scratch buffer for
  the aux signal in that case (see Halfwit).

## nice-plug fork

Plugins pin the `patina` branch of
[starzonmyarmz/nice-plug](https://github.com/starzonmyarmz/nice-plug/tree/patina):
upstream nice-plug plus one commit (`d1f60d2`) that adds
`ActivateContext::gui_context()`, so a hardware controller can set
parameters before the editor has opened. Patina relies on it for its
MiniLab knobs. Keep the fork to that one commit: anything else on the
branch has to be carried through every rebase.

- Upgrading: rebase `patina` onto upstream, push, and bump the `rev` in
  every repo that pins it (`plugin/` and `xtask/` `Cargo.toml`, plus
  `app/` in Patina). Then run the tests and clap-validator in each.
- Don't send the patch upstream as a PR. It was written with AI, and
  nice-plug's [AI policy](https://rust.audio/community/ai/) doesn't
  accept AI-written code, PR descriptions or comments. If the need
  should be raised, the user writes an issue by hand that describes only
  the use case.
