# ClerdoVector app icon

A fountain-pen nib drawing a Bézier curve, with its anchor points and handles, in warm orange on a
deep navy tile with a slate rim.

**Palette:**

| Colour | Hex | Used for |
|---|---|---|
| Navy | `#101b2e` → `#0c1422` | the tile |
| Slate | `#3a4a6a` → `#2a3854` | the rim |
| Orange | `#efa13d` / `#d9862a` | the nib (light and shaded halves), curve, handles and anchors |

**Tile:** `viewBox="0 0 512 512"`, full bleed with rounded corners. The macOS files add Apple's
transparent margin (tile = 824/1024 of the canvas); Windows and Linux use the tile edge to edge.

**Provenance:** the ClerdoVector icon artwork supplied by Clerdo, redrawn as a vector so every size
stays sharp. `clerdovector.svg` is the canonical artwork (the file names are kept so packaging and
the build find it).

## Files

| File | What | Used by |
|---|---|---|
| `clerdovector.svg` | master vector | source for everything below |
| `clerdovector-small.svg` | same artwork | `hicolor/scalable` |
| `clerdovector-1024.png` | full tile, 1024 px | docs, store listings |
| `clerdovector-macos-512.png` | tile with Apple margin, 512 px | runtime Dock icon (`apps/clerdovector/src/main.rs`) |
| `clerdovector.icns` | macOS icon set, 16 to 1024 px | `cargo xtask bundle` (`CFBundleIconFile`) |
| `clerdovector.ico` | Windows icon, 16 to 256 px | `apps/clerdovector/build.rs` (embedded in the `.exe`) |
| `hicolor/<size>/apps/app.clerdo.clerdovector.png` | Linux theme icons, 16 to 512 px; the 256 one is also the runtime icon on Windows and Linux, the 128 one the brand mark in the app bar and About box (`crates/ui-egui/src/brand.rs`) | `packaging/linux/app.clerdo.clerdovector.desktop` |
| `hicolor/scalable/apps/app.clerdo.clerdovector.svg` | Linux scalable icon | as above |

## Regenerate

Edit `clerdovector.svg` (copy it to `clerdovector-small.svg`), then run `packaging/icons.sh`
(needs `resvg`; `iconutil` on macOS for the `.icns`).
