# Plugin template

A starting point for VST3 + CLAP plugins in Rust for macOS and Windows,
built with [nice-plug](https://codeberg.org/RustAudio/nice-plug), with the
build and release setup from [Patina](https://github.com/starzonmyarmz/patina).
The plugin itself is a gain knob with a webview editor, there to prove the setup works.

It pins the same nice-plug fork as Patina
([starzonmyarmz/nice-plug](https://github.com/starzonmyarmz/nice-plug/tree/patina)),
which adds `ActivateContext::gui_context()` so MIDI controllers can set
parameters before the editor opens.

## Plugins made with it

<img src="https://raw.githubusercontent.com/starzonmyarmz/halfwit/main/docs/halfwit.jpg" height="200" alt="Halfwit, a mustard-yellow stompbox with knobs and a footswitch"> <img src="https://raw.githubusercontent.com/starzonmyarmz/ghosted/main/docs/ghosted.jpg" height="200" alt="Ghosted, a vintage walnut and bakelite radio with an amber dial full of ghosts"> <img src="https://raw.githubusercontent.com/starzonmyarmz/altitude/main/docs/altitude.jpg" height="200" alt="Altitude, a powder-blue stompbox printed in navy, with five slide faders and a footswitch"> <img src="https://raw.githubusercontent.com/starzonmyarmz/patina/main/docs/screenshot-1.0.0.jpg" height="200" alt="Patina’s synth editor">

- [Halfwit](https://github.com/starzonmyarmz/halfwit): an octave-divider
  bass layer for guitar, as a stompbox.
- [Ghosted](https://github.com/starzonmyarmz/ghosted): drifting
  double-tracking, from ADT to slapback, as a vintage radio.
- [Altitude](https://github.com/starzonmyarmz/altitude): stacked octaves,
  two down to two up, with presets, as a screen-printed stompbox.
- [Patina](https://github.com/starzonmyarmz/patina): a polysynth and Mac
  app. It came first, and the template's build and release setup comes
  from it.

## Design

Every plugin gets its own look, but they share an approach.

- **Skeuomorphic.** The editor is a physical object (Patina an
  OP-1-style synth, Halfwit a stompbox, Ghosted a vintage radio), with
  real materials like powder coat, bakelite, walnut, brass and grille
  cloth, built in CSS gradients and shadows rather than images.
- **One theme.** Pick the object from the plugin's vibe and name, and
  let it run through everything: the wordmark, the art and the control
  labels. Names should be playful and capture the vibe.
- **Restrained.** One knob style and colour family per plugin, and few
  controls. Fold many settings into a handful of knobs with evocative
  names rather than exposing every one.
- **Colour and art mean something.** If two waves or knobs differ in
  colour, it's because they differ in function; otherwise make them the
  same. Art can follow parameters (Ghosted's ghosts fade with Haunt, the
  dial needle follows Distance), but don't fake audio metering.
- **Plain labels.** Words a guitarist understands, not DSP terms or
  fieldset jargon. Numbers go in a tooltip readout.
- **Type with character.** Embed a display face for the wordmark
  (Futura-like Jost, pillowy Modak), never a system or "commercial"
  font, and centre it optically on its badge.
- **Drawn, not vector-perfect.** Vary repeated elements (size, tilt,
  expression), and give outlines weight that swells on the shadow side,
  like inked comic art.
- **A footswitch on effects.** Pedals and other effects get an on/off
  toggle (`stomp`, on by default). Off fades the effect out and passes
  the dry signal; a lamp or LED shows the state, and the art reacts
  (Ghosted's dial dims and its ghosts leave).
- **The version on the panel.** Print it small and quiet as part of the
  object, like a model number on a pedal's base plate or a serial plate on
  a radio's back (`v1.2.0`), so anyone can tell which build they're
  running. It comes from `version` in the root `Cargo.toml`.
- **Physical details behave physically.** Shadows stay put while a knob
  turns, pressed keys sink, lit things glow.

## Starting a plugin

Click **Use this template** on GitHub (or `gh repo create --template`), then rename `Example`. `CLAUDE.md` has the rest of the routine:

| What | Where |
|---|---|
| Crate name (`example`) | `plugin/Cargo.toml`, `bundler.toml`, `CRATE` in both workflows, `crate` in `scripts/install.sh`, `use example::` in `plugin/tests/engine.rs` |
| Display name (`Example`) | `bundler.toml`, `NAME` and `PAGE.title` in `plugin/src/lib.rs`, `PLUGIN` in both workflows, `INSTALL.txt`, `<title>` in `ui/index.html`, the usage line in `scripts/clap-validator.sh` |
| Dev URL env var (`EXAMPLE_UI_URL`) | `PAGE.dev_url_var` in `plugin/src/lib.rs` |
| Plugin IDs | `CLAP_ID` and `VST3_CLASS_ID` (exactly 16 bytes) in `plugin/src/lib.rs`. These must be unique, and must never change after a release. |
| Description, features, subcategories | `ClapPlugin` and `Vst3Plugin` in `plugin/src/lib.rs` |

## Building

```sh
cargo xtask bundle example --release       # target/bundled/Example.{vst3,clap}
scripts/install.sh                         # bundle, sign ad hoc and install on macOS
scripts/clap-validator.sh target/bundled/Example.clap
```

## Tests

The DSP lives in `plugin/src/engine.rs`, apart from the host plumbing in
`lib.rs`, so `cargo test` can run it offline. The plugin passes parameter
values to the engine once per block.

`plugin/tests/support` has helpers for checking sound, not just numbers:

- `read_wav` loads test input, such as a guitar DI checked into
  `plugin/tests/di`.
- `check_golden` compares a render with `plugin/tests/golden/<name>.wav`.
  After an intended change to the sound, rerun with `BLESS=1` to rewrite
  the golden files, then listen to them before committing.
- `check_golden_stereo` does the same for a stereo render, stored as a
  stereo WAV, for effects whose width matters.
- `tone` makes a test note with a few harmonics, for checks that don't
  need a recording.

`plugin/tests/engine.rs` shows the pattern: render through the engine
in host-sized blocks, then compare with a golden file or assert on the
output directly.

## CI and releases

- `check.yml` runs on pushes to main and on PRs: the tests and clap-validator
  on macOS, and clap-validator on the Windows build.
- `release.yml` runs on `vX.Y.Z` tags. The tag must match `version` in the
  root `Cargo.toml`. It builds a universal macOS bundle and a Windows bundle,
  validates both, and publishes `Example-X.Y.Z.zip` with `INSTALL.txt`. A
  version with a suffix (`1.0.0-beta.1`) becomes a prerelease.

To release: bump `version` in `Cargo.toml`, commit, then
`git tag vX.Y.Z && git push origin main vX.Y.Z`.

## Upgrading nice-plug

Rebase the fork's `patina` branch onto newer nice-plug, push it, and update
the `rev` in `plugin/Cargo.toml` and `xtask/Cargo.toml`.

## The editor

The editor is a [wry](https://github.com/tauri-apps/wry) webview in a
[baseview](https://github.com/RustAudio/baseview) window, showing
`ui/index.html`, which is compiled into the plugin.

- `plugin/src/editor.rs` hosts the page. `PAGE` in `lib.rs` is its only
  config: title, size, HTML, the background shown until the page
  paints, and an env var that loads the page from a dev server instead.
- `plugin/src/bridge.rs` connects the page to any `#[derive(Params)]`
  struct through `param_map()`. The page sends `ready`, then `begin`,
  `set` and `end` with normalized values; the plugin answers with
  `init` (the plugin's version, and each parameter's id, name, step
  count, options and default) and `values` (normalized value and the
  plugin's formatted text).
- Host keys go to the page only while something on it has focus, so
  shortcuts like play keep working.

The starter page builds a knob for every parameter, or a switch for
two-step ones, so a new plugin has a working UI right away. Replace it
with a hand-laid page once the controls settle;
[Halfwit](https://github.com/starzonmyarmz/halfwit)'s pedal is an
example.

Without the plugin, the page runs against a mock bridge in
`ui/index.html`. Keep the mock's parameters and version in step with
the plugin's, then work on the page in a browser:

```sh
cd ui && python3 -m http.server 8765    # open http://127.0.0.1:8765
```

Set `EXAMPLE_UI_URL` to that URL before starting the host to load it
live in the plugin.

Fonts can't load from the network in the plugin: embed them as base64
`@font-face` data URLs, and keep the font file and its license in
`ui/fonts`.

## Adding a standalone app

Copy it from Patina (`app/`). It isn't here yet because it's built
around Patina's parameters, and the reusable shape should come from the
second plugin that needs one. Guitar plugins will also need audio input
selection, which Patina's app doesn't have.
