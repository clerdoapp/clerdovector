# Gaps

> **Last reviewed:** 2026-10-10 · **Last updated:** 2026-11-01 · **Change:** minor (G5: dynamic symbols, #4938; estimate 15–26 → 14–24 h) · **Target:** Adobe Illustrator 2026 (30.x)

## Ranked

| Rank | Gap | Hours | Beta | Doc |
|---:|---|---:|:---:|---|
| 1 | [G1 Interaction fidelity](#g1-interaction-fidelity) | 60–90 | yes | [ui-parity.md](ui-parity.md) |
| 2 | [G9 Illustrator `.ai` and EPS files from real users](#g9-illustrator-ai-and-eps-files-from-real-users) | 10–20 | yes | [file-format-parity.md](file-format-parity.md) |
| 3 | [G10 Launch and stability on real machines](#g10-launch-and-stability-on-real-machines) | 20–35 | yes | — |
| 4 | [G8 Hardening at scale](#g8-hardening-at-scale) | 30–50 | yes (part) | — |
| 5 | [G11 Correctness bugs in core tools](#g11-correctness-bugs-in-core-tools) | 8–15 | yes | [target-app-parity.md](target-app-parity.md) |
| 6 | [G4 Advanced type](#g4-advanced-type) | 34–47 | core part | [type-parity.md](type-parity.md) |
| 7 | [G12 Preferences and panels that don't act yet](#g12-preferences-and-panels-that-dont-act-yet) | 13–20 | yes | [ui-parity.md](ui-parity.md) |
| 8 | [G2 Photoshop-style raster effects and the Effect Gallery](#g2-photoshop-style-raster-effects-and-the-effect-gallery) | 1–2 | no | [effects-parity.md](effects-parity.md) |
| 9 | [G13 Illustrator 2026 (30.x) additions](#g13-illustrator-2026-30x-additions) | 13–23 | no | [target-app-parity.md](target-app-parity.md) |
| 10 | [G14 Pen hardware beyond Windows pressure](#g14-pen-hardware-beyond-windows-pressure) | 4–10 | Wayland pressure | [hardware-parity.md](hardware-parity.md) |
| 11 | [G5 Brushes, symbols and libraries](#g5-brushes-symbols-and-libraries) | 15–25 | no | [target-app-parity.md](target-app-parity.md) |
| 12 | [G3 3D and Materials](#g3-3d-and-materials) | 25–45 | no | [effects-parity.md](effects-parity.md) |
| 13 | [G7 Views and windows](#g7-views-and-windows) | 15–25 | no | [ui-parity.md](ui-parity.md) |
| 14 | [G6 Automation and scripting](#g6-automation-and-scripting) | 4–8 | no | [target-app-parity.md](target-app-parity.md) |
| 15 | [G15 Performance budgets and a GPU renderer](#g15-performance-budgets-and-a-gpu-renderer) | 15–25 | budgets only | [hardware-parity.md](hardware-parity.md) |
| 16 | [G16 Localization: the three missing key languages](#g16-localization-the-three-missing-key-languages) | 50–85 | no | [localization-parity.md](localization-parity.md) |
| 17 | [G17 Ecosystem: third-party plug-ins](#g17-ecosystem-third-party-plug-ins) | 20–45 | no | — |
| 18 | [G18 Generative AI features](#g18-generative-ai-features) | 30–60 | no | — |
| 19 | [G19 Accessibility](#g19-accessibility) | 15–25 | no | [ui-parity.md](ui-parity.md) |

**Alpha blockers:** none. Every core workflow passes the [alpha gate](roadmap.md#alpha-gate); G9 is the only
partial one, and it blocks beta, not alpha.

**To beta:** G1, G9, G10, G11, G12, the corpus and QA part of G8, the type-core part of G4, macOS pen pressure
from G14 and the budget run of G15: **~170–270 h** one agent, ~50–80 h wall clock with 4–6 agents.

## G1 Interaction fidelity

- **Done:** Distance Guides ([public page](https://helpx.adobe.com/illustrator/desktop/measure-and-align/grids-and-guides/use-distance-guides-for-accurate-placement.html)): with Smart Guides on and something selected, the Selection tool with Alt/Option held over another object shows the horizontal and vertical distances between them, and over an empty part of an artboard the selection's distances from its four edges, labelled in the General unit; Preferences › Smart Guides › Invoke Distance Guides turns them off. Hover only, as documented: not while moving or nudging. No entry closed; estimates unchanged.

- **Done (part of #4028):** Recolor Artwork › Assign: double-clicking a row's new color opens the Color Picker, as the [public page](https://helpx.adobe.com/illustrator/desktop/manage-colors/modify-colors/assign-new-colors-to-selected-artwork.html) describes ("double-click it and select a color in the Color Picker"). OK sets the row's new color in the model it was picked in, so a CMYK mix is no longer a hex value converted back; Cancel leaves the row. A Lab new color is kept, but can't be typed yet: the picker's Lab fields work in RGB, as for fill and stroke. **Missing:** Lab entry in the picker, swatch libraries in the picker and the row's right-click menu (Pick from Swatches, Limit to Library).

- **Done (#3416):** Recolor Artwork Edit chips scroll to the last color while the heading, color wheel and actions stay visible. Screen resize and UI scaling, Preview, Confirm, Cancel, Undo and #1733 wheel drags are covered in `tests/recolor.rs`. No gap entry closed; estimates unchanged.

- **Polygon/Star count persistence:** Exact-size placement saves the chosen count for later drags and reopening the existing canvas dialog. Regression coverage includes both tools, repeated drags, switching tools and Cancel. The remaining G1 scope and estimates are unchanged.

- **Artboard Options shares existing presets and orientation controls and can fit artwork without moving it; artwork fitting includes all drawn document art across artboards and off-board; artwork bounds are cached by active document and revision, and selection uses the shared bounds cache across artboards:** Shared existing preset definitions are reused; no external preset assets are added. Presets preserve each selected artboard's orientation, and orientation choices apply even when the active board already matches. Fit operations draft bounds until OK, affect only the active artboard and leave artwork in place. Regression coverage checks draft/Cancel, selected-set application, active-only fitting and Undo. Broader scope and estimates unchanged.

- **Implemented, regression-tested and contributor-confirmed:** status messages and readouts use stable responsive allocations sized for localized labels and ordinary coordinates. Original-file acceptance confirmed 2026-10-19; broader G1 scope and estimates unchanged. Sample widths are cached by language, document unit and pixels per point rather than remeasured every frame.

- **Canvas review follow-up:** Cached canvas dimensions match the renderer limit before cache-key creation, view construction and worker submission. Controlled-worker regression checks oversized width and height, high DPI and the render transform without allocating oversized images. G1 scope and estimates are unchanged.
- **Done (#3106):** while the window is resized, rotated or moved to a screen of another DPI, the cached canvas frame is redrawn at its own scale until the next render, at most the renderer's largest raster size. Broader G1 scope and estimates unchanged.
- **Done:** the selection tools take a clipping path (a clip group's or a layer's clipping mask) on its outline, within the selection tolerance, before the art it clips, so Direct Selection reshapes a mask where the art covers its edge; Select Behind lists the same, and the eyedropper and the tools that act on what is drawn are unchanged. Broader G1 estimates unchanged.
- **Done:** one selected clip group's Properties panel has Edit Clipping Path and Edit Contents at its top, which select its clipping path or its clipped art for the selection tools (public doc cited in target-app-parity.md). Broader G1 estimates unchanged.
- **Done:** the Properties panel has the additional contextual controls of the public Properties panel overview: Blend Options in Quick Actions for one blend with the Blend tool, Merge Live Paint for a Live Paint group selected with more paths to merge (Live Paint Make and Merge refuse type, images and other objects that aren't paths, which used to vanish among the group's sources), the Width Profile list in Appearance with the Width tool or a variable-width stroke, an applied effect's Delete icon beside fx (with several effects it opens the Appearance panel), and Corner and Corner Radius for corners selected with Direct Selection (public doc cited in target-app-parity.md). Broader G1 estimates unchanged.
- **Done (#2691):** a selected Grid Repeat ([public page](https://helpx.adobe.com/illustrator/desktop/paint-and-fill/create-and-edit-patterns/set-grid-repeat-options.html)) shows a handle below the grid, one right of it and one past its bottom-right corner: dragging them adds or removes rows, columns or both (1 to 500). A slider between the first two columns and one between the first two rows set the horizontal and vertical spacing, staying halfway across the gap under the pointer. Each drag is one undo step through `object.repeat.options` and Escape cancels it; the controls hide with the edges or the bounding box, and bounding-box handles keep priority where they meet. Radial Repeat handles remain. Estimates unchanged.

- **Done (#2651):** Object › Repeat › Options… for a Grid Repeat has a Preview checkbox ([public page](https://helpx.adobe.com/illustrator/desktop/paint-and-fill/create-and-edit-patterns/set-grid-repeat-options.html)): while it is on the repeat on the canvas follows the fields, Cancel puts the original back without a history step and OK keeps the previewed values as one undo step. Radial and Mirror Repeat Options have none, as their public pages list none. Estimate unchanged.
- **Done (#2877):** a selected Mirror Repeat shows its axis: drag the line or the center handle to move it, an end handle to rotate it (Shift: 45°); each drag is one undo step and Escape cancels it. Bounding-box handles take priority over the axis line; the center and end handles keep their actions. Broader G1 estimates unchanged.

- **Existing-guide Escape cancellation:** Selection and Direct/Group Selection clear pending presses and cancel begun guide previews. App-input regressions cover preselected-guide document and interaction-start selection restoration, Undo/Redo and saved state, and later on-canvas motion/release. All eight app-input checks and the Direct Selection Escape control passed in current full CI; [Validation evidence](target-app-parity.md#evidence-what-ships-today-by-area). The broader G1 scope and estimate remain unchanged.

- **Done:** Artboards menu, trash and Properties delete selected artboards together; selecting all retains artboard 1. Pointer-event regressions cover ordinary artwork, the first-board survivor, Undo/Redo and the single-board guard; broader artboard reports remain open.

- **Done:** Artboard Delete/Backspace delete selected boards, retaining the original first when all are selected. The last board and active drags stay protected; headless shortcut regressions cover artwork, saved state and Undo/Redo.
- **Done:** status-bar artboard navigation synchronizes the Artboard tool selection before keyboard or Properties deletion, preserving ordinary artwork selection. Actual Next-button regressions cover both deletion keys, Properties Delete, Undo/Redo and other tools in `chrome.rs`. **Tab-switch follow-up:** changing the active document through its tab aligns the Artboard tool with the displayed board; same-tab multiselection and Move/Scale Artwork options are preserved. Three tab regressions and historical full CI on `a0671777` passed; [Validation evidence](target-app-parity.md#evidence-what-ships-today-by-area). The broader G1 scope and estimate remain unchanged. With the Artboard tool active, closing a clean active tab restores the surviving document's displayed board before Properties Delete.
- **Implemented and tested:** Undo removes invalid Artboard selection indices without resetting valid multiselection or Move/Scale Artwork options. Headless history tests cover actual panel New/Duplicate, deletion, artwork, object selection, saved state and Undo/Redo; full workspace CI passes.
- **Added:** Alt swaps the orientation of a new guide dragged from either ruler. Preview and final placement use the current axis; cancelling preserves the document and Redo, and a completed drag is one undo step. Tool and egui event regressions cover these interactions; the broader G1 pass and estimate remain unchanged.
- **Width-point follow-up:** Adjoining adjustments avoid intermediate overflow; non-finite normalized or resulting factors are rejected before committing. Engine regressions cover atomic rejection, Undo/Redo and native save/reopen. Current full CI passed; [Validation evidence](target-app-parity.md#evidence-what-ships-today-by-area). The broader G1 pass and estimate remain unchanged.
- **Done:** double-click a stroked segment to create a width point through Width Point Edit. Opening, Cancel and deleting the prospective point preserve the document and history; OK creates the point and optionally adjusts adjoining widths in one undo step. Regression and visual evidence: [Width-point creation](target-app-parity.md#evidence-what-ships-today-by-area).
- **Done:** Shift-double-click opens Width Point Edit with adjoining adjustment enabled. Plain double-click retains the default; dispatch, neighboring widths, Undo/Redo and Cancel have regression coverage.
- **Done (#1227):** object clicks with the Artboard tool create a fitted artboard, preserving art and selection; handles, modified clicks and drags retain their meaning. Covered by tool-event regression tests and undo/cancellation checks.
- **Done (#3705, in part):** pastes follow the drawing mode ([drawing modes](https://helpx.adobe.com/illustrator/desktop/draw-shapes-and-paths/learn-drawing-basics/drawing-modes-overview.html)): Draw Inside takes Paste, Paste in Place and Paste on All Artboards inside the object drawn into, Draw Behind puts Paste in Place and Paste on All Artboards behind the selection, and Shift+D no longer stops at Draw Behind when one group or other non-path object is selected.
- **Done:** Artboard Options' Show Center Mark and Show Cross Hairs ([public source](https://helpx.adobe.com/illustrator/desktop/create-manage-artboards/organize-manage-artboards/modify-display-settings-of-artboards.html)): per-artboard checkboxes (`artboard.setProps`/`artboard.new` `showCenterMark`, `showCrossHairs`) that the canvas draws as a center cross and lines from the middle of each side through the center; exports and print leave them out.
- **Done (#2849):** Artboard Options' video display options ([public source](https://helpx.adobe.com/illustrator/desktop/create-manage-artboards/organize-manage-artboards/set-video-display-options-for-artboards.html)): Show Video Safe Areas (`showVideoSafeAreas`) outlines the action-safe area 3.5% and the title-safe area 5% in from each edge, our choice from [EBU R 95](https://tech.ebu.ch/docs/r/r095.pdf) (the help page gives no numbers), on the canvas only; Video Ruler Pixel Aspect Ratio (`videoRulerPixelAspectRatio`, 0.1–10, default 1) is the width of one video pixel over its height. Both per artboard, one undo step, in the native format only when set. View › Rulers › Show Video Rulers is in G7.
- **Done (#3705):** Draw Inside in isolation mode ([isolation mode](https://helpx.adobe.com/illustrator/desktop/manage-objects/select-objects/isolate-objects.html)): with the object drawn into isolated by a double-click, art drawn or pasted inside it takes isolation mode into its clip group, so it can be clicked and isn't dimmed; it used to stay locked and dimmed beside the isolated path, and double-clicks never reached the clip group.
- **Done:** Artboard Options' Show Center Mark and Show Cross Hairs ([public source](https://helpx.adobe.com/illustrator/desktop/create-manage-artboards/organize-manage-artboards/modify-display-settings-of-artboards.html)): per-artboard checkboxes (`artboard.setProps`/`artboard.new` `showCenterMark`, `showCrossHairs`) that the canvas draws as a center cross and lines from the middle of each side through the center; exports and print leave them out. Video safe areas, video rulers and the pixel aspect ratio remain missing.
- **Done:** Document Setup → Edit Artboards applies valid settings, closes the dialog and activates the Artboard tool. Pointer-click tests cover successful handoff, invalid settings, OK, Cancel and Undo in `dialogs/document_setup.rs`.
- **Done (#2691):** a double-click on the Line Segment, Arc, Rectangular Grid or Polar Grid tool opens its Tool Options, as the [Rectangular Grid tool](https://helpx.adobe.com/illustrator/using/tool-techniques/rectangular-grid-tool.html) and [Arc tool](https://helpx.adobe.com/illustrator/using/tool-techniques/arc-tool.html) pages (step 3) and the [Tools](https://helpx.adobe.com/illustrator/using/tools.html) page describe: the tool's click dialog without a place to draw (Line Segment adds Fill Line; the grids keep their Default Size and Reference Point). OK draws nothing and sets what the next drag draws with and what a click's dialog opens at; Cancel changes nothing. Editing a grid's rows and columns after it is drawn (the rest of #2691) remains. Broader G1 estimate unchanged.
- **Done:** Simplify of part of a path, as the [manual-simplify page](https://helpx.adobe.com/illustrator/desktop/draw-shapes-and-paths/modify-paths/manually-simplify-paths.html) describes for part of a path picked with Direct Selection: with anchors or segments of a path direct-selected, Object › Path › Simplify (the slider, Auto-Simplify and the dialog) refits only each run of selected segments between its end anchors, which stay where they are with their outer handles; the other anchors and segments stay exactly as they were, a closed path's run may cross its start point, separate runs on one path simplify independently in one undo step, and the runs stay selected (`object.path.simplify` `wholePaths: true` simplifies such paths whole). Tests: `crates/engine/src/tests/simplify_part.rs`, `simplify_slider_and_dialog_simplify_only_the_direct_selected_part_of_a_path`. Broader G1 estimate unchanged.

- **Improved:** Curvature uses circular point markers without direction handles or duplicate markers
  (#1236, in part); real egui frames cover creation, existing paths, Anchor Size and Hide Edges.
  The request to change interpolation of existing paths remains open.
- **Improved:** the Curvature tool's curve: each stretch of three points follows the circle through
  them (handles along its tangent, `chord / (3 cos²(α/2))` long) and the ends of an open path mirror
  their neighbour's tangent, so a J or a U curves evenly, a turn tighter than 30° comes to a point,
  and uneven spacing doesn't loop (#1236, in part). The rubber band, dragged, inserted and converted
  points use the same rule. Re-curving a dragged point's neighbours on an existing path remains open.
- **Improved:** Eyedropper has an original pipette cursor at its sampling tip; Use Precise
  Cursors keeps the crosshair. Bitmap/DPI and preference checks cover the distinction (#1269).
  OS cursor display and Caps Lock remain unverified.
- **Done (#1251):** Eyedropper source reads are independent of locks and isolation, including Shift/Shift+Alt and gradient stops. Source and selection preservation, unchanged Alt reverse/selection restrictions, hidden art and Undo/Redo have regression coverage. This addresses the reported behavior, with no new full-parity measurement.
- **Done (#1253):** adding paint to an unpainted zero-width stroke restores the existing 1 pt default, including new-art defaults and Swap Fill/Stroke. The real Eyedropper inheritance sequence, targeted appearance items, preserved widths and Undo/Redo have regression coverage.
- **Done:** a busy cursor while a file opens off the UI thread or a labelled background save or export runs (`Cursor::Busy` in `crates/tools/src/cursors.toml`). Heavy synchronous effect previews remain open.
- **Done:** Average and Join in one step, with a corner or smooth choice for the joined anchor: Object › Path › Average & Join… (`path.averageJoin {to: "corner"|"smooth"}`, Cmd+Alt+Shift+J, Ctrl+Alt+Shift+J on Windows) moves the two direct-selected open path ends, or the nearest ends of two selected open paths (both ends of one, closing it), to their midpoint and joins them there in one undo step. The public [Default keyboard shortcuts](https://helpx.adobe.com/illustrator/using/default-keyboard-shortcuts.html) page lists the shortcut as "Create corner or smooth join" ("Select the paths" on Windows, "Select the anchor point" on macOS). The dialog (Points: Corner / Smooth) opens on Corner and then on the choice confirmed last in the session: ClerdoVector's reading, as the page doesn't say. Inside the estimate below.
- **Implemented (#1760):** Effect › Warp has a shared style selector with original previews, orientation buttons and percent sliders. Existing effects retain their stack slot and target. Focused checks passed and original captures were reviewed; optional Properties integration and remembered defaults remain outside this change.
- **Done (#2120):** Smart Guides line extension: dragging the end of a straight segment with Direct Selection or Curvature, the point snaps to the segment's own direction at any angle and the guide runs along it from the other end. Inside the estimate below.
- **Done (#1692):** Tab in the Color Picker moves through one color model's fields before the next. Completed before the layout change (#2008).
- **Improved (#2008):** Color Picker layout: stacked top-right actions, aligned inputs, less empty space, a distinct header and compact color controls. Narrow windows use stacked sections and vertical scrolling. Pointer/key, warning, layout and mode-switch regressions cover the change; the broader fidelity pass and estimates remain unchanged.
- **Done (#834):** bezier drag previews no longer freeze: a Pen, Anchor Point or Direct Selection drag gets every
  position the pointer is seen at, including the one where it comes back into the window after leaving it and a
  move in the frame it lets go, so the release commits where the pointer let go; a click's jitter stays a click.
- **Missing (noted for reporters, 2026-10-14):** aligning anchor points selected with
  Direct Selection to a key anchor (from #1636: they align to their bounds; the current public Align page does
  not say how the key anchor is chosen, so this waits for the owner). Inside the estimate below.
- **Missing:** a tool-by-tool, panel-by-panel pass over modifiers, cursors, the Properties panel per context,
  isolation mode and small behaviours, against the public tool and shortcut documentation. Itemized in
  [ui-parity.md](ui-parity.md). The Shaper's construction-mode widget and gesture refinements remain.
- **Evidence:** no side-by-side session has ever happened (the clean-room rule forbids running Illustrator, so
  the pass works from the documentation); users keep reporting small differences: #991, #975, #973, #955, #908.
- **Impact:** a power user notices this in the first minutes and stops trusting the tool.
- **Estimate:** 60–90 h; splits well by tool across agents.

## G9 Illustrator `.ai` and EPS files from real users

- **Done (2026-10-30):** from an audit of a user's `.ai`/EPS files: All Caps and Small Caps; placed files at their box's corner; pages cropped to the art; gradient mesh corners with handle weights of 0 and the transparency written after a mesh, a masked mesh and a blend; a layer's state block with the stroke style; type on a path spaced as the file lays it out; EPS10 legacy-format type; a PDF of some artboards compared on the artboard its page shows. Comparison limits remain unchanged. This narrows G9; scores and estimates unchanged.

- **Save-back update (2026-10-26):** ordinary EPS Save retains page margins and the whole editable document; source-loss confirmation remains. Import fidelity gaps and estimates below are unchanged.

- **Done (2026-10-16):** EPS legacy mesh corners and vector opacity masks retained; winding-declared compound member groups keep shared paint. Raster print-page minification no longer causes a false editing-data fallback; comparison limits remain unchanged. Synthetic regressions cover damage and missing artwork. This narrows G9; scores and estimates unchanged.

- **Missing:** faithful layer and group structure from the editing data in real files (#951: 109 layers instead
  of 14), guides and non-printing construction
  lines (#779), open speed on large files (#758), live effects, brushes and symbols as live objects (#637; plain art styles read Drop Shadow, Outer Glow and Feather as live effects; pattern fills and global colours are read since #1025, placed (linked) files since #1264). Writing Illustrator's own editing data is out of scope
  (undocumented): a PDF-compatible `.ai` opens in Illustrator 2018 without its layers and groups (#868). Save As
  `.ai` › Version: Legacy (version 7) writes the PostScript form from the published version 7 specification
  instead, keeping layers (names, order, visibility, lock), groups, compound paths, clipping groups, gradients and
  images, and type as editable text objects (or outlines), one artboard per file (an artboard picker chooses
  which); transparency, live effects and object names don't survive it (transparent images are flattened or
  omitted, hidden objects go to a hidden layer, are omitted or made visible), and type is set again in the
  reader's fonts. Legacy type in `.ai` files in PostScript form is read on open. It waits for testing by users
  who hand files back.
- **Evidence:** the issues above; 28 of the 350 issues are about `.ai`/EPS (about 22 people, the most common single
  format theme), 20 fixed, and the 8 open ones are about structure (layers, guides, speed, live objects), not missing
  art. No real-file `.ai` corpus (users' files may be used locally, never committed).
- **Impact:** blocks anyone who hands files back to Illustrator users. Opening and reworking their own archive
  works, with layer cleanup in some files. This is the main-format gap that keeps the stage at alpha.
- **Estimate:** 10–20 h with users' files to test against.

## G10 Launch and stability on real machines

- **Unconfirmed (2026-10-14):** Tool cursors are painted into the window on Wayland (#1652; not run on Linux, #1645 stays open until confirmed). On Windows the custom title bar's window stays truly borderless, so the pointer lines up with what is drawn on systems where the hidden caption offset it (#1791, #1758; awaiting the reporter's confirmation).
- **Done:** RPM required dependencies use 64-bit library capabilities and declare glibc 2.35, libm and libgcc_s (#1275); generated metadata and isolated synthetic-provider checks cover x86_64/aarch64. Real-distribution install/start-up QA is still missing.
- **Done (2026-10-29):** the Linux Flatpak on GNOME no longer reconnects to the compositor and logs a warning every 250 ms when arboard has neither data-control nor X11 (267k warnings in 18 h filled the 16 MB log in 3 h): a clipboard that can't be opened is retried with a wait growing to a minute and logged once. Copy and Paste of text with other apps work there through the core Wayland protocol.
- **Missing:** pictures (`image/png`) and SVG as `image/svg+xml` on the clipboard of the Flatpak on GNOME, where only text travels (smithay-clipboard 0.7 carries text only); the app not starting or staying in the background on some Windows 11 machines (#858, #964 fixed
  2026-10-10, #620, #818), slow pointer on a Linux AppImage (#575), macOS paste from other apps (#954), a
  web-free crash/hang reporter, a field record.
- **Evidence:** open issues; the never-crash lints and `guard` stop panics, not driver, windowing or packaging
  failures.
- **Impact:** an app that doesn't start on someone's machine is 0% for them.
- **Estimate:** 20–35 h, needs reporters' help on hardware we don't have.

## G8 Hardening at scale

- **Done:** PngSuite fetching verifies 175 original PNGs and redistribution notices against a pinned SHA-256/size manifest before publishing; file and total-size budgets, cache verification and no archive extraction (#1729). The real-world corpus and manual QA gaps below remain.

- **Done:** bounded synthetic control-input expansion (#1709): click/drag counts and pending-event capacity are checked before mutation, and control-channel bursts yield between frames. The corpus and platform QA below remain pending.
- **Done (2026-10-14):** WMF/EMF import refuses an oversized embedded PNG or JPEG from its header, before any pixels are decoded (#1776). PDF open no longer takes exponential time on a crafted name tree with shared nodes when it looks for the editing data (#1781). The size limits on a native file's embedded data also apply to files read from memory, not only streamed ones (#1783). Saving over a private file on Unix no longer leaves a readable temporary copy beside it while the file is written (#1778). An Export for Screens suffix can no longer place files outside the export folder (#1777). Package refuses a folder where it would overwrite the open document or one of its linked files (#1786). ruzstd 0.9.1 fixes a decompression bomb (RUSTSEC-2026-0344) reachable from EPS/`.ai` and Affinity import (#1807). Scatter graph data near the largest number is refused with a message instead of hanging the app (#1780). One very long Liquify stroke segment can no longer create an unbounded number of dabs (#1779). MCP limits: 16 MiB lines, 64-message batches, 256-step `command_batch` with 8 MiB of kept results and 256 MiB control replies (#1764, fixes #1727). MCP requires the JSON-RPC 2.0 envelope and matches control replies by id, and an app refusal without an id reports the app's reason (#1768, fixes #1728). The corpus and platform QA below remain pending.
- **Done:** control sockets have bounded idle, absolute partial-line and reply-write waits (#1726); real TCP tests cover idle, trickled and blocked peers, buffered partial lines, persistent requests and connection-slot recovery. The corpus and platform QA below remain pending.
- **Done (later 2026-10-14):** nesting capped at 32 objects across SVG, PDF, EPS and native files, self-using symbols
  and patterns (#1852); deep or self-referencing PDF and `.ai` data (#1871), decompression bombs and impossible
  predictors (#1873, #1700, #1701), self-drawing brushes (#1872), hostile EPS, Affinity and native data (#1863:
  #1715, #1714, #1704, #1699; save fallback #1695), ids at their largest value, failed `command.batch` rollback,
  timed-out control requests and oversized Eyedropper images (#1862: #1718, #1697, #1725, #1717), bounded shadows,
  glows and stacked Transform effects (#1864: #1706, #1705).
- **Done (2026-10-22):** SVG images in SVG images share the reader's depth cap: usvg reads an image while it reads the
  SVG showing it, on the same stack, so their depths add up, and `text/plain` and untyped data URLs (which usvg also
  reads as SVG) are measured as `image/svg+xml` ones are; an image past the cap is left out with a warning instead of
  overflowing the stack (#3562). SVGZ images are unpacked before usvg reads them, within one 512 MB cap for all of a
  file's images, where usvg unpacked a data URL's SVGZ without a cap (#3563).
- **Done (2026-10-22):** stacked geometry effects make at most 2 million anchors of a path (of all the paths of a
  type, symbol or image art together), or the path's own count when that is more. Roughen spaces its points further
  apart, Zig Zag makes fewer ridges a segment and Free Distort, Twist and the warps split segments into longer
  pieces where they would pass it, and an effect that would still pass it (Offset Path, Round Corners, a plug-in) is
  left out. The result is also kept to about 8 million lines for the renderer to draw at 1 px/pt (Wang's formula at
  vello's 0.25 px tolerance): a large Roughen or Zig Zag draws each point as up to hundreds of lines, so it is built
  again with fewer points. Three Roughens of a rectangle in a 25 KB file needed tens of gigabytes of geometry, and two
  million anchors of them 141 million lines (7 GB, past vello's 2²⁷ lines a path); the file now opens in 0.4 s and
  about 450 MB (#3578). The budget is per path, not per document: many objects that each reach it still add up.
- **Done (2026-10-23):** a brushed stroke makes at most 2 million anchors in total, the same figure as the
  stacked-effects budget above. Pattern tiling and scatter capped each factor on its own (20,000 tiles a run, 4,000
  subdivisions a segment, 20,000 copies a track), but those caps are per track and a path has as many tracks as it
  has subpaths, so they multiplied instead of bounding: one track at its cap measures 20,000 pieces and 320,000
  anchors, and sixteen subpaths measured exactly sixteen times that, 5.1 million anchors in 0.57 s. A piece is now
  taken whole or not at all, and the walk that counts an art tile's anchors charges the whole budget for a tree it
  cannot finish, so nesting buys no room (#3594). The budget is per brushed stroke, not per document: many strokes
  that each reach it still add up, and a brush inside a symbol is bounded per stroke rather than per instantiation.
- **Done (2026-10-31):** a gradient mesh is budgeted (#3592). One tessellation makes at most 1,048,576 quads, and a
  mesh with more patches than that (rows times columns) is malformed, like a point count that doesn't fit, so it draws
  nothing instead of exhausting memory: a 150 x 150 mesh from a 3 MB file exported at 20x asked for about 23 million
  quads (2 GB). Below the budget the subdivision per patch gives way a step at a time (a 60 x 60 mesh draws at 16 x 16
  quads a patch instead of 32 x 32); a small mesh is drawn as finely as before. The command's own cap of 50 rows and
  columns is unchanged.
- **Done (2026-10-29):** a blend makes at most 100,000 steps in all, counting the steps of the blends inside its keys
  and steps. Each step between keys that are blends is a copy of one, which makes its own steps when drawn, so step
  counts capped at 1,000 a pair multiplied by nesting: five levels of 15 steps were 1.3 million steps from a 23 KB
  file, 3.4 GB to export to PNG. Past the budget, every pair keeps the same share of its steps, and none when the keys
  alone fill it; a copy of a blend spaced by distance counts the most steps its spacing can make, since scaling it up
  adds steps. That file now exports in 0.5 s and 520 MB (debug build), and nested blends within the budget draw
  every step as before (#3579). The budget is per blend, not per document: blends whose keys each fill it still add
  up with the file's size, and blends inside a symbol's art don't count toward a blend whose keys show it.
- **Missing:** a corpus of real-world SVG/PDF/EPS files; more Affinity versions and platforms (51 pinned files
  now, including 28 original Affinity 3.2.3 feature probes; large interacting documents and complex typography
  missing); Windows, Linux and browser QA by hand; packaging polish (Homebrew cask #588, Flathub #554, update
  checks #615).
- **Impact:** unknown failures on users' files.
- **Estimate:** 30–50 h (the corpus and QA part blocks beta).

## G11 Correctness bugs in core tools

- **Live trace result bounds and selection highlights exclude the retained bitmap except in source-image views:** `live_trace_result_bounds_exclude_the_retained_bitmap_at_any_angle` checks every TraceView, geometric and visual bounds and rotated result bounds. `live_trace_selection_highlight_excludes_the_retained_source_rectangle` checks result highlights and the existing ordinary-group policy. Shared editing-child traversal also preserves Shaper handling. Broader scope and estimates unchanged.

- **Implemented, Windows native canvas and regression checks passed in PR #2643 (#2544):** Anchor Point edits selected blend spine points and handles with path-compatible selection/stacking, handle visibility and 45-degree constraints; #2425 perpendicular segment reshaping is preserved. Broader G11 scope and estimates unchanged.

- **Done (#1968):** native saves retain transparency grid visibility without adding an edit or undo step; older files default to hidden. Broader G11 scope and estimates unchanged.

- **Done:** Asset Export / raster Art Optimized edges supersample so shapes that meet edge to edge leave no ghost seam (#983, #1037; `assets.export` abutting-rects regression; Asset Export panel thumbs use the same supersample).
- **Done (2026-10-14):** Align and Distribute act on the anchor points selected with Direct Selection: the points of one path align to each other, not to the artboard, and Distribute spaces them (#1641, fixes #1636; aligning to a key anchor is still missing). Delete removes every selected anchor of a closed path (it used to remove the wrong ones), removes objects selected whole along with the selected anchors of others, and keeps the remainder selected so a second Delete removes it (#1674). In isolation mode Direct Selection and Group Selection pick only inside the isolated object (#1666). Escape cancels a Direct Selection drag (#1803, #1743). Escape while typing ends the edit, keeps the type selected and switches to the Selection tool (#1802, #1686, #1656). A filled Pencil path keeps a stroke of None (#1804, part of #1679). In the Pathfinder panel a group selected alone counts its nested shapes, so an expanded stroked object keeps the Shape Modes enabled (#1642, #1635). Object › Expand turns radial, grid and mirror repeats into groups of their instances (#1765, #1756). Select › Same matches type on its characters' fill and stroke (#1746, #1737). Roughen with Smooth keeps turn-backs smooth instead of making cusps that draw as spikes (#1646, #1643). Scribble draws one zig-zag per region, with no lines across holes and notches; convex shapes are unchanged (#1651, #1648).

- **Done:** Divide splits filled paths along open cutting lines and curves (#1307), preserving the frontmost filled appearance, implicit filled closure, holes, unrelated objects and one Undo/Redo; the existing bounded-map fallback remains.

- **Done:** Numeric fields keep a text drag's start when the pointer gains focus and movement exceeds the click threshold (#1326, in part). A completed focus click selects the whole value; keyboard focus, double-click and numeric stepping remain regression-tested. Short movements still classified as clicks can select all; that case remains open.

- **Done:** DXF reads control-point SPLINE closure (#1299 follow-up after #1320); open curves keep their open state. DXF export now recognizes circular cubic paths conservatively and writes native CIRCLE/ARC where stroke semantics permit; R12 weighted strokes remain POLYLINE, and a non-circular ellipse keeps the existing fallback. Alter Paths for Appearance outlines only the strokes a CAD line can't draw (aligned, profiled, offset dashes, wider than 2.11 mm, or in R13 and R14), so a 1 pt circle or arc stays a CIRCLE or ARC. CNC preset and VCarve QA remain open. Escape leaves Artboard editing for Selection; during a drag it first cancels the gesture. Headless egui key events cover idle Escape, active-drag cancellation and a subsequent Escape in `tests/distortkeys.rs`.
- **Done:** DXF reads control-point SPLINE closure (#1299 follow-up after #1320); open curves keep their open state. DXF export now recognizes circular cubic paths conservatively and writes native CIRCLE/ARC where stroke semantics permit; R12 weighted strokes remain POLYLINE. A closed four-cubic ellipse, or a circle scaled out of proportion, is written as a native ELLIPSE in R13 and later; R12 keeps the existing fallback. CNC preset and VCarve QA remain open. Escape leaves Artboard editing for Selection; during a drag it first cancels the gesture. Headless egui key events cover idle Escape, active-drag cancellation and a subsequent Escape in `tests/distortkeys.rs`.

- **Done:** Simplify and Add Anchor Points clear stale component selections on rewritten paths; Simplify previews and reports original/new point counts (#1272).
- **Done:** Object › Path › Simplify opens its slider at the Auto-Simplify level for the selection and previews that result, as the [manual-simplify page](https://helpx.adobe.com/illustrator/desktop/draw-shapes-and-paths/modify-paths/manually-simplify-paths.html) describes; simplifying only a Direct Selection part of a path is done too (G1).
- **Done:** no-op anchor Align preserves Undo, Redo, saved state and path geometry (#1180 follow-up),
  with the same 1e-9 pt tolerance as whole-object Align; actual alignment remains one undo step.
- **Done:** Direct Selection picks a segment independently of its anchors and Delete preserves neighboring edges (#1278);
  scaled text's size and horizontal-scale controls account for object/group transforms (#1034),
  including exact sizes across grouped labels and resetting horizontal stretch while keeping editable text.
  Single-result Pathfinder booleans keep source names and SVG IDs, including named groups/compound paths (#1293).
- **Done (later 2026-10-14):** the Transform panel's, Properties panel's and Control bar's X/Y/W/H edit the anchor points
  selected with Direct Selection (#1839, #1824); Snap to Grid wins over Snap to Pixel (#1840, #1816 unconfirmed); Redo
  during a drag keeps the drag (#1856, #1719); a template layer dims its images by Dim Images instead of fading the
  whole layer (#1864, #1683); a failed save or export is reported in a dialog that names the file and stays until
  closed, the document left modified (#1865, #1685).
- **Done:** the Shape Builder offers the empty areas closed shapes enclose as regions, such as the middle of four
  circles that touch, and merges them with the regions around them (#1691).
- **Done (#2523):** Shape Builder Gap Detection shares Live Paint's spatially indexed gap finder,
  including its overshoot/hook safeguards. The existing Shape Builder Tool Options offers Small (3 pt),
  Medium (6 pt), Large (12 pt) or Custom (up to 72 pt). Temporary bridges are used in detection and
  merging, but are never added to the art. Off by default; `shapeBuilder.merge` and
  `shapeBuilder.regions` take `gap` in points. Other existing options are retained. Clicking a stroke
  in merge mode and broader interaction QA remain.

- **Implemented:** deleting an interior anchor of an open path removes its connecting segments and keeps surviving
  runs separate, without bridging the gap. The remainder stays selected for a second Delete; explicit Remove
  Anchors still reconnects/refits. Other G11 items and the estimate below remain unchanged.
- **Done (#1910):** Transform Each's Reflect X flips left-right and Reflect Y flips top-bottom (aligned with
  Effect › Distort › Transform). Properties flip buttons keep the reference app's Properties names (Flip Along Horizontal / Vertical
  Axis).
- **Done (#885):** the Transform effect dialog's checkboxes are visible, and it has Transform Objects, Transform
  Patterns and Scale Strokes & Effects: off, Transform Objects leaves the geometry in place; the other two scale
  stroke weights, dashes and the fills' and strokes' effect distances and move pattern tiles with the object, copy
  `k` by the transformation `k` times. Canvas, export and Expand Appearance agree. Not yet: a Transform effect in
  a fill's or stroke's own effect list ignores the two options, and object effects after it keep their distances.
- **Done (#937, reproduced case):** Shape Builder regions on art of more than 4,096 segments (traced or type
  outlines): the planar map no longer gives up there, so lines cut such art instead of the tool offering whole
  shapes (the planar map's edge-source and face lookups were quadratic in the segments, now indexed, and the cap is 65,536; Divide with open paths shares it). The edge-source index files a long segment under the grid cells along it, so a diagonal hatch (every line's bounds hold most points) no longer costs a scan of the hatch per edge. The grid holds a budget of references (64 per item plus four rows' or columns' worth), the items past it scanned, so coincident segments can't make it quadratic in memory.
- **Done (2026-10-24):** Shape Builder line and edge pieces on dense hatches: where the sweep cut a line's way there
  and its way back a rounding error apart and joined the two cuts with a stub a little over its tolerance, the stub
  came back as a speck of a piece (lines at exact 0.25 or 0.5 pt steps across a 45° line). The planar map now contracts
  such stubs into one vertex (within 4× the sweep's tolerance up and down its frame and 1/16 of the 0.01 pt precision
  along it, bounded by each cluster's extent so a run of tiny level segments keeps its length); regression tests
  in `crates/pathops/src/planar.rs`. Estimate unchanged.
- **Missing:** SVG units reverting to points (#864); a Transform effect in a fill's or stroke's own effect list ignores
  Transform Patterns and Scale Strokes & Effects (#885, above). Shape Builder regions (#937, closed), PDF text placement
  (#722, above), the bezier drag preview (#834, #3065, #3729) and deleting an interior anchor of an open path (above)
  are done.
- **Done (#722):** PDF text opens where the file draws it. Each line used to come in as point type laid out by the
  font's metrics (its kerning, one even tracking, every glyph on one baseline, a vertical glyph's column an em wide),
  so glyphs slid wherever the file spaced them otherwise. Now a line takes the font's kerning or none, whichever the
  file followed, and tracking that puts every glyph where the file drew it (word and character spacing and `TJ`
  adjustments per character); text rise (`Ts`) stays in the line as baseline shift; a vertical glyph is centred on
  its column by the font's own metrics (`/W2`, else half its `/W` or `/DW`, ISO 32000-1 §9.7.4.3); justified text
  boxes come back as justified area type. Fixtures written from ISO 32000-1 §9 (`tests_text_placement`). A font
  that is neither embedded nor installed still draws in a stand-in with the old fit.
- **Impact:** wrong output in everyday work.
- **Estimate:** 8–15 h.

## G4 Advanced type

- **Fixed:** fullwidth `！` / `？` stay upright in vertical point and area type when a font has no vertical alternate. Existing `vert` substitutions and the rotated fallback for brackets and long vowel marks are preserved; the broader CJK gap and estimate are unchanged.

- **Done:** character-style Auto-leading resets propagate to style users while retaining local leading overrides. Sparse definitions remain sparse, native serialization retains explicit Auto, and the Character Styles override marker stays consistent.

- **Improved:** mixed paragraph alignments leave the existing Paragraph/Properties alignment buttons unselected; choosing an alignment still uses the range-aware command. Headless regressions, full integration CI and visual review pass. Other mixed paragraph fields remain; estimate unchanged.
- **Improved:** Paragraph panel left/right/first-line indents and space before/after show blank when paragraphs in scope differ; uniform values still display and edits apply through `text.setFormat`. Headless regression in `panels/paragraph.rs`; estimate unchanged.
- **Improved (#1258):** full font names (name ID 4) resolve the face instead of falling back, and Missing Fonts searches recognize the same aliases. Synthetic regressions cover loaded/cataloged fonts, localized names, real-family precedence and variable default faces. Windows font-service activation still needs reporter QA; estimate unchanged.
- **Improved:** PDF import finds an installed font whose PDF name holds hyphens (`Noto-Sans-Bold`, `Noto-Sans-CJK-JP`) or a composite font's CMap (`SomeFont-Italic-Identity-H`; ISO 32000-1 §9.7.6.1), and names a missing one by the whole name unless a style part follows (`MS-PGothic` is MS PGothic). Synthetic regressions in `crates/pdf/src/tests/import_fidelity.rs`; estimate unchanged.
- **Done:** Paragraph panel menu › Roman Hanging Punctuation (#2756), per paragraph; estimate unchanged.
- **Improved:** hanging punctuation (Roman Hanging Punctuation and Optical Margin Alignment) keeps its published amounts when justification scales glyphs, and Standard burasagari counts a hung line start as room; paragraph-level tests with both composers; estimate unchanged.
- **Done:** Paragraph panel menu › Justification… (#2755): word and letter spacing, glyph scaling (Minimum, Desired,
  Maximum), Auto Leading and Single Word Justification per paragraph, used by both composers; estimate unchanged.
- **Done:** a paragraph's Auto Leading also sets the Leading the Character panel shows for Auto type, the leading the
  Alt+↑/↓ nudge steps from and the lines Fill with Placeholder Text counts; estimate unchanged.
- **Missing:** CJK vertical
  composition (ruby, mojikumi sets and dialog, kinsoku settings #633, manual tate-chu-yoko), Middle Eastern
- **Done:** Ruby (ふりがな) for live text (`type.ruby`): a reading over a byte range, above or below (right of the
  column in vertical type), aligned, sized, overhanging or squeezed; saved natively, laid out with line growth, drawn
  on the canvas and in SVG/PDF (outlines where ruby is present), EPS, EMF/WMF and legacy `.ai` (outlines where ruby is
  present), and kept by Create Outlines, Expand Appearance and flattening; estimate unchanged.
- **Done:** colour emoji fonts with bitmap glyphs (`sbix`, as Apple Color Emoji; `CBDT`/`CBLC`, as Noto Color Emoji):
  each glyph is its largest strike's PNG, placed and transformed with the type, its fill and stroke ignored; PDF writes
  it in the font's Type 3 glyphs (or as an image), SVG as an image when type is outlined (live text leaves emoji to the
  viewer, and their fonts aren't embedded), EPS and legacy `.ai` as images, and Create Outlines makes embedded images.
  Fonts with bitmaps but no outlines (Noto Color Emoji) are now found among the installed fonts. Limits: `sbix`
  duplicate, flipped, JPEG and TIFF glyphs and `CBDT` BGRA and mask data draw nothing, nor do emoji in ruby or in
  enveloped or warped type; estimate unchanged.
- **Done:** COLR colour fonts (v0 and v1): a glyph's layers are vector art (solid colours, linear and radial
  gradients, clips, blend modes, palette 0, the foreground colour from the run's fill), placed with the type, drawn on
  the canvas, written as paths in SVG, EPS, legacy `.ai` and PDF with outlined text, as text in PDF, and editable
  paths with Create Outlines. Limits: glyphs using sweep gradients, Porter-Duff compositing, radial gradients from a
  circle that isn't the centre, or repeating gradients are images of the glyph until documents have those paints;
  estimate unchanged.
- **Done:** OpenType SVG colour fonts: each glyph's SVG document is read by the SVG importer into vector art,
  in the run's colours where it uses `currentColor` (or the legacy `context-fill`/`context-stroke`), then drawn, exported and outlined as COLR
  glyphs are (PDF real text writes them as text that draws nothing, their art drawn where they sit). Images in glyph documents
  go with the glyph's art; text and scripts are left out, as the table asks. Limit: CPAL palette variables; estimate
  unchanged.
- **Done:** threading ports (#2572): every selected area type and type on a path shows an in port and an out port,
  empty, with an arrow where it is threaded or with the red "+" of text that doesn't fit; with the Selection tool a
  click on the out port of any of them loads the text to thread on, and a click on the in port threads a new or
  clicked frame before it, so the story starts there (`text.thread.add` `before`). A double-click on a port where the
  frame is threaded breaks the thread there, the story flowing into the frames before the break and the frames after it
  left empty (`text.thread.break`; [archived help](https://web.archive.org/web/20220127090642/https://helpx.adobe.com/illustrator/using/creating-text.html)); estimate unchanged.
- **Done:** variable fonts: the font menu's Font Technology › Variable lists only variable font families (a family
  with a variable file beside static ones included), together with the script, kind and favourites filters, as
  [Find, apply, and adjust variable fonts](https://helpx.adobe.com/illustrator/desktop/design-with-text/fonts-and-scripts/find-apply-and-adjust-variable-fonts.html)
  describes; with the named instances as styles and the Character panel's Variable Font axes, variable fonts are
  done; estimate unchanged.
- **Done:** COLR and SVG glyphs go with type that geometry effects reshape, into Expand Appearance and EMF/WMF
  export, where the type's outlines used to leave them out; bitmap glyphs (emoji) too, as images that hold their
  own pixels until a document takes them in; estimate unchanged.
- **Missing:** CJK vertical
  composition (mojikumi sets and dialog, kinsoku settings #633, manual tate-chu-yoko), Middle Eastern
  features beyond bidi and digit types (Character Direction, kashidas, composers), spell check with an open dictionary,
  Touch Type, Retype, Snap to Glyph; optional preservation of font sizes during group scaling (#1034 enhancement).
  Itemized in [type-parity.md](type-parity.md).
- **Impact:** professional typography and Japanese/Chinese publishing.
- **Estimate:** 34–47 h (type core 14–19 h blocks beta).

## G12 Preferences and panels that don't act yet

- **Done (2026-10-29, #4522):** the Pathfinder panel menu's Make, Release and Expand Compound Shape, which were dimmed placeholders, and the panel's Expand button act on live compound shapes ([Create compound shapes with Pathfinder](https://helpx.adobe.com/illustrator/desktop/manage-objects/reshape-transform-objects/create-compound-shapes-with-pathfinder.html)): Alt-clicking a Shape Mode makes one in that mode, a Shape Mode sets the mode of the members selected, and the compound shape draws, exports and saves as the area its members' modes make (`object.compoundShape.*`, `crates/engine/src/tests/compound_shape.rs`). Left: blends, envelopes and warps as members, question marks on the buttons for mixed member modes, clicks that hit the area rather than the members. Estimate unchanged.
- **Done (2026-10-25, #3426):** Pathfinder panel › Pathfinder Options… ([Pathfinder panel overview](https://helpx.adobe.com/illustrator/desktop/manage-objects/reshape-transform-objects/pathfinder-panel-overview.html)): Precision (0.001–10 pt; results are refit to within it and pieces thinner than it are dropped), Remove Redundant Points and Divide and Outline Will Remove Unpainted Artwork (pieces of objects with no fill), kept in the preferences (`object.pathfinder.options`, group `pathfinder`) and used by every Shape Mode and Pathfinder button and Repeat; the page gives no defaults, so ours (0.01 pt, both off) keep the earlier results byte for byte. Trap is left as it was. Estimate unchanged.
- **Done (2026-10-25):** Artboards panel › Delete Empty Artboards ([Delete artboards](https://helpx.adobe.com/illustrator/desktop/create-manage-artboards/add-edit-artboards/delete-artboards.html), [artboard context menu](https://helpx.adobe.com/illustrator/desktop/create-manage-artboards/organize-manage-artboards/use-artboard-context-menu.html)): `artboard.deleteEmpty` deletes every unlocked artboard with no artwork, with its guides, in one undo step, and keeps artboard 1 when every artboard is empty and none is locked; an artboard is empty when no object (hidden, locked and template art included; guides not) has visual bounds that meet or touch it, a group counting by its members and a clip group or clipping layer by its members inside the clip (our reading of "no artwork"; the pages give no finer rule). The canvas context menu doesn't list it yet. Estimate unchanged.
- **Done (2026-10-25):** the Transform panel menu's Transform Object Only, Transform Pattern Only and Transform Both
  (`transformPanelPatterns`) choose what the panel's fields, flips and menu flips and the Properties panel's Transform
  section transform: the object with its pattern tiles left as they are, the tiles alone, or both; Use Registration
  Point for Symbol (`useRegistrationPointForSymbol`) has them measure and transform one selected symbol instance from
  its symbol's registration point, where the instance's flips, rotations and shears put it, instead of the reference
  point; the Properties panel's rotation and flips now turn and mirror about the reference point, as the Transform
  panel's do. Limit: on a sheared instance a new W or H keeps the box's point nearest the registration
  point in place, not the registration point itself ([Transform panel overview](https://helpx.adobe.com/illustrator/desktop/manage-objects/reshape-transform-objects/transform-panel-overview.html)).
  Until a choice is made (`default`), the panels follow General › Transform Pattern Tiles as before, and the menu
  checks Transform Both while it is on and Transform Object Only while it is off; a choice overrides it.
  Estimate unchanged.
- **Done (2026-10-21, #394):** General › Constrain Angle turns the arrow keys' axes (guides stay on the page's), holds Shift drags of the Selection, Direct Selection, Free Transform, Slice Selection, Reshape and Measure tools and the Pen's handles to the turned axes, and new rectangles, rounded rectangles, ellipses, grids, point and area type, Shift-drawn polygons and stars and the shape size dialogs are drawn along it with their bounding box turned; 0° and whole turns are exactly as before, quarter turns exact (<https://helpx.adobe.com/illustrator/desktop/manage-objects/arrange-objects/rotate-objects-by-specific-angles.html>). Arcs, spirals, flares and the Pen's new paths are not turned yet. Estimate unchanged.
- **Done (2026-10-21, #394):** General › Constrain Angle turns the arrow keys' axes (guides stay on the page's), holds Shift drags of the Selection, Direct Selection, Free Transform, Slice Selection, Reshape and Measure tools and the Pen's handles to the turned axes, and new rectangles, rounded rectangles, ellipses, grids, point and area type, Shift-drawn polygons and stars and the shape size dialogs are drawn along it with their bounding box turned; 0° and whole turns are exactly as before, quarter turns exact (<https://helpx.adobe.com/illustrator/desktop/manage-objects/arrange-objects/rotate-objects-by-specific-angles.html>). Arcs, spirals, flares and the Pen's new paths are not turned yet. Estimate unchanged. **2026-10-24:** General › Corner Radius is the Rounded Rectangle tool's radius: the tool takes the preference when chosen and when it changes while the tool is active (or lent a selection tool for a drag), and the radius ↑/↓ set while drawing one, or a Tool Options value, becomes the preference, clamped to its 0–1296 pt range (<https://helpx.adobe.com/illustrator/using/drawing-simple-lines-shapes.html>). Estimate unchanged.
- **Done (2026-10-17):** Background Export shows its progress in a dialog with Cancel and Continue in Background (#2564), on the shared cancel token (#2546); a cancelled export writes no file. Save As, Export for Screens and Export Selection don't show it yet. Estimate unchanged.
- **Done:** quick Image Trace presets trace at once and leave the panel as it was; the Image Trace button (Control bar, Properties) opens the controls with Threshold and Advanced (Paths, Corners, Noise), as the public documentation describes (<https://helpx.adobe.com/illustrator/using/image-trace-results-optimization.html>). Other context-specific Properties gaps remain.
- **Audited (2026-10-30, #394):** every Preferences dialog option but the five below is read by shipped code, by
  its field or its key; Highlight Alternate Glyphs (the canvas's render options), Real-time Drawing and Editing
  (tool previews wait for the commit), Show Tool Tips, Anti-aliased Artwork, Use Precise Cursors and Devices ›
  Enable Touch Workspace and Touch Gestures included.
- **Missing:** 5 Preferences options stored but not acted on (#394), each waiting on a subsystem of its own:
  User Interface › Open Documents As Tabs (off would open documents in floating windows; one window per
  document is G7), Units › East Asian Type (nothing is measured in it yet, and the Q and Ha units aren't in the
  unit lists), Hyphenation › Default Language (hyphenation is English-only, with no per-language dictionaries),
  Plug-ins & Scratch Disks › Primary and Secondary (documents are kept in memory; nothing pages to disk), and
  Clipboard Handling › Legacy vector clipboard with its Preserve Paths / Preserve Appearance and Overprints choice
  (Illustrator's own clipboard flavour, which has no public specification). Also Properties panel per
  context, context menus beyond the canvas, Layers panel row menus and Collect for Export, SVG Interactivity.
- **Impact:** settings that silently do nothing erode trust.
- **Estimate:** 13–20 h.

## G2 Photoshop-style raster effects and the Effect Gallery

- **Done:** Effect › Texture › Texturizer › Load Texture… (stored grayscale `textureImage`, Loaded Texture in the Texture menu), matching Glass.
- **Missing:** the Effect Gallery's filter thumbnails and a preview area inside the dialog ([Filter Gallery overview](https://helpx.adobe.com/photoshop/desktop/effects-filters/get-started-with-filters/filter-gallery.html)); all 57 Photoshop-style filters are in the catalogue. Measured in
  [effects-parity.md](effects-parity.md).
- **Done:** the Effect Gallery (#2883): Effect › Effect Gallery… browses the Artistic, Brush Strokes, Distort, Sketch, Stylize and Texture filters, edits each filter's settings, builds an ordered stack (add, reorder, hide, remove) with a live canvas preview, and OK applies it as separate appearance effects in one undo step (`dialogs::effect_gallery::tests::stack_ok_is_one_undo_step`, `stack_preview_is_one_interaction`).
- **Done:** Effect › Sketch › Conté Crayon (`sketch.conteCrayon`: Foreground Level, Background Level, Texture, Scaling, Relief, Light Direction, Invert; original visual approximation, defaults unverified), the last missing filter.
- **Done:** Effect › Artistic › Colored Pencil (`artistic.coloredPencil`, pencil width, stroke pressure, paper brightness; original visual approximation).
- **Done:** the pixel-filter pipeline and all 57 Photoshop-style filters (Artistic, Blur, Brush Strokes, Distort, Pixelate,
  Sharpen, Sketch, Stylize, Texture, Video), with live preview and editable native settings. Smart Blur supports Edge Only and Overlay Edge
  modes. Per-filter detail: [effects-parity.md](effects-parity.md).
- **Estimate:** 1–2 h (Effect Gallery thumbnails and in-dialog preview).

## G13 Illustrator 2026 (30.x) additions

- **Done:** TIFF choices and format settings in Export for Screens and Asset Export (30.2), using the existing encoder; UI-to-file regressions cover selection, scaling, options and persistence.
- **Done:** artboard background colors on the canvas (30.0): Properties › Artboards › Fill and Artboard Options set
  each artboard's color, and the canvas draws its paper in it. Exports (raster, PDF, SVG, EPS, `.ai` PDF pages,
  Export for Screens with Include Artboard Fill) and print draw it behind the art (#4937). Left: the color isn't read
  from or written to the reference app's own `.ai` data.

- **Done:** Object › Intertwine (#1763): Make, Edit and Release; the Intertwine tool highlights the overlap area under
  the pointer, a click swaps which object is on top there, and a path (Shift: a rectangle) drawn around overlaps
  arranges each one inside it. Where three or more objects overlap, hovering highlights the object nearest to the
  pointer, and a right-click offers Bring to Front, Bring Forward, Send Backward and Send to Back for it in that area
  (`object.intertwine.arrange` with `order` front, forward, backward or back;
  <https://helpx.adobe.com/illustrator/desktop/manage-objects/reshape-transform-objects/create-intertwined-objects.html>).
  The group renders, saves and exports through its live expansion. Still missing: adding objects to the group by
  dragging them in the Layers panel is untested.
- **Done (from users' reports):** Object › Create Gradient Mesh on an embedded image turns it into a mesh over its
  rectangle, with the image's transform kept; each point takes the average colour and opacity of the picture around it
  (#1676). A linked image is refused, as the public documentation has it.
- **Partial:** blend easing (#1124, 30.7): Window › Blend has Type (Ease In, Ease Out, Ease In Out,
  Ease Out In), Ramp 0–100% and independent color Shift; easing is disabled without a spine.
  The original polynomial distributions approximate the public description. On-canvas easing handles
  and their synchronization remain missing. [Public source](https://helpx.adobe.com/illustrator/desktop/manage-colors/apply-transparency-and-blending/blend-panel-overview.html).

- **Done:** gradient Dither (30.0): the Gradient panel's Dither box (`paint.editGradient` `dither`, also in `paint.setFill`'s
  `gradient`, swatches and the native file) draws a linear or radial gradient fill as pixels with seeded noise of up to a
  colour level, which hides banding on the canvas and in raster exports; SVG and PDF exports write the fill as an image at
  the Document Raster Effects resolution, and PDF with Preserve Editing keeps the gradient editable. Fills only, as the
  [public documentation](https://helpx.adobe.com/illustrator/desktop/paint-and-fill/create-and-edit-gradients/control-dither-in-gradients.html) says. Freeform gradients ignore it.
- **Done:** gradient Method (30.0): the Gradient panel's Method menu, Classic or Perceptual (`paint.editGradient`
  `method`, also in `paint.setFill`'s `gradient`, swatches and the native file, written only when Perceptual). Perceptual
  mixes a linear or radial gradient's colours in OKLab, so blue to yellow passes through a light blue-grey rather than
  mid grey; renderers draw it through stops on that curve (within half a colour level), and SVG and PDF exports write it
  as an image at the Document Raster Effects resolution, as the
  [public documentation](https://helpx.adobe.com/illustrator/desktop/paint-and-fill/create-and-edit-gradients/control-perceptual-interpolation-in-gradients.html) says perceptual gradients are rasterised there.
  Classic draws exactly as before; freeform gradients mix their own way. With Dither, a Perceptual fill is the
  perceptual mix with dither's noise (an image with the noise in SVG and PDF); a stroke has no noise.
- **Missing (asked for by users, from earlier versions):** radial gradients in Image Trace's
  Gradients option, which traces linear ones only (draft #1849). Inside the estimate below.
- **Done (#2530):** tangent snapping to curves and perpendicular snapping to straight target
  segments, as the public [perpendicular](https://helpx.adobe.com/illustrator/desktop/measure-and-align/plot-and-measure/snap-to-perpendicular.html)
  and [tangent](https://helpx.adobe.com/illustrator/desktop/measure-and-align/plot-and-measure/snap-to-tangent.html)
  pages describe, for the Line Segment and Pen tools' new lines and for existing lines: dragging the end
  anchor of an open path's straight segment with Direct Selection lands it square to
  another straight path (the foot of the perpendicular from the segment's other end) or grazing a curve, with
  the "perpendicular" or "tangent" label (and the square-corner mark) Smart Guides show for drawn lines. As
  the pages say, only line endpoints snap: the middle anchor of a path, the end of a curved segment, a closed
  path's anchor, several anchors dragged together and Shift-constrained drags (which keep their angle step) do
  not, and a curve is never a perpendicular target. Controls are saved under Preferences › Smart Guides ›
  Geometric Guides and available from Control bar › Snapping; both default to on. Tests:
  `dragged_line_ends_snap_perpendicular_to_segments_and_tangent_to_curves` in `crates/tools/src/direct/tests.rs`.
- **Done (2026-10-30):** gradient presets (30.0): the Gradient panel's Presets menu applies one of the
  built-in gradient libraries' presets (by theme) or a saved one to the gradient behind the active proxy, keeping
  its placement, as one undo step; the panel menu saves the gradient as a preset (kept with the preferences) and
  deletes saved ones; `gradient.presets.*` commands.
- **Missing:** the consolidated
  snapping options (30.0); artboard colors in exports and print, and managing artboards from the canvas other
  than locking and unlocking them from the Artboard tool's right-click menu (30.0); the Blend panel with ease
  in/out and separate colour acceleration (30.7);
  the Dimension tool (28.x).
- **Evidence:** Adobe's 30.0 and 30.7 release announcements, see
  [target-app-parity.md](target-app-parity.md#how-this-was-measured-2026-10-10).
- **Estimate:** 13–23 h.
- **Estimate:** 14–24 h.

## G14 Pen hardware beyond Windows pressure

- **Done:** pen pressure on macOS (#852), read from AppKit's tablet events before the window gets them, each
  move of a stroke with its own pressure, and the pen's eraser end there selecting the Eraser tool, as on Linux
  under X11 and XWayland; not yet tried on a Mac with a tablet. On the web, a press of the pen's eraser end
  selects the Eraser tool and a press of the tip brings back the tool before; not yet tried with a pen.
  under X11 and XWayland; not yet tried on a Mac with a tablet. The pen's eraser end on Windows with Windows Ink,
  read from the pen's pointer messages; not yet tried with a pen.
- **Missing:** pen pressure on native Wayland (#491, #764) — winit 0.30 reports none there; tilt, bearing and
  barrel rotation for brushes (6D Art Pen, #372); the eraser end on Windows and native Wayland.
  barrel rotation for brushes (6D Art Pen, #372); the eraser end on native Wayland and the web.
- **Estimate:** 4–10 h, partly waiting on winit 0.31; needs pen hardware to verify.

## G5 Brushes, symbols and libraries

- **Done:** Symbol Options › Registration (9-point; stored and used by `symbol.place`); Symbol Options ›
  Export Type (Graphic or Movie Clip; default Movie Clip; `symbol.new` / `symbol.options` / Symbols panel;
  canvas placement unchanged; Animate / Flash handoff metadata); Symbol Options › Enable Guides for 9-Slice
  Scaling (four guides as fractions of the symbol size, `nineSlice` / `nineSliceGuides` on `symbol.new` /
  `symbol.options`, stored per symbol; handoff metadata, not drawn on the canvas, #2342); Brush Options › Key
  Color eyedropper for scatter, art and pattern brushes (Hue Shift colorization; samples canvas art); Select ›
  Start Global Edit (#3521) on paths, compound paths and symbol instances: the objects of the same shape at
  any place and size, narrowed by Appearance, Size and the artboards to search, take an edit of any one of
  them in proportion, as one undo step ([public source](https://helpx.adobe.com/illustrator/desktop/manage-objects/edit-objects/edit-similar-objects.html));
  on a group (#4939), the groups like it (the same objects at the same places, moved or scaled) take an edit
  made to an object inside it at the object in the same place ([public source](https://helpx.adobe.com/illustrator/desktop/manage-objects/edit-objects/edit-groups-with-similar-objects.html));
  dynamic symbols (#4938): Symbol Options › Symbol Type (Dynamic Symbol / Static Symbol; `type` on
  `symbol.new` / `symbol.options` / `symbol.list`), an instance's own fill, stroke, opacity and blend for an
  object of a dynamic symbol's art (`symbol.instance.setAppearance`, by child-index path), drawn and exported
  with the instance; redefining the symbol keeps each change whose object remains, Break Link bakes them,
  Reset (`symbol.instance.reset`, the Properties panel and the Symbols panel menu) drops them, and making the
  symbol static drops them; stored in the native format.
  Color eyedropper for scatter, art and pattern brushes (Hue Shift colorization; samples canvas art);
  symbol libraries ([public source](https://helpx.adobe.com/illustrator/desktop/manage-objects/traces-mockups-symbols/create-or-import-symbol-libraries.html)): Window › Symbol Libraries and the Symbols panel's Symbol
  Libraries Menu open each library in a library panel of its own (one original library generated in code,
  Basic Shapes; User Defined; Other Library… reads the symbols of any document ClerdoVector opens); a click
  adds a symbol to the document's Symbols with its Symbol Options and the resources it uses, as one undo
  step, and dragging one onto the canvas also places it; Save Symbols / Save Symbol Library writes the
  document's symbols as a `.clerdovector` library into the user library folder (listed under User Defined) or
  a file (`symbol.libraries`, `symbol.addFromLibrary`, `symbol.saveLibrary`, `symbol.loadLibrary`); brush
  libraries ([public source](https://helpx.adobe.com/illustrator/desktop/paint-and-fill/apply-and-edit-strokes/create-new-brush-libraries.html),
  [import](https://helpx.adobe.com/illustrator/desktop/paint-and-fill/apply-and-edit-strokes/import-brushes-from-brush-libraries-or-other-files.html)):
  Window › Brush Libraries and the Brushes panel's Brush Libraries Menu open each library in a library panel of
  its own (four original libraries generated in code: Art Ribbons, Calligraphic Nibs, Pattern Borders, Scatter
  Shapes; User Defined; Other Library… reads the brushes of any document ClerdoVector opens); a click adds a
  brush to the document's Brushes with the patterns, symbols and other resources its art uses, as one undo
  step (applied to the selected paths in the same step), dragging one onto a path adds and applies it and
  dragging one onto the Brushes panel adds it; Save Brushes / Save Brush Library writes the document's brushes
  as a `.clerdovector` library into the user library folder (listed under User Defined) or a file
  (`brush.libraries`, `brush.addFromLibrary`, `brush.saveLibrary`, `brush.loadLibrary`).
- **Missing:** the Art brush's Overlap; more built-in brush, symbol and graphic-style libraries (original,
  generated in code, never Adobe's); dynamic symbols' on-canvas editing (Direct Selection of an object inside a
  dynamic instance, with the Fill/Stroke controls and Appearance panel then setting its change) and their `.ai`
  read and write; Global Edit of groups holding type or
  images, rotated copies as matches, Shift-click to leave a match out and highlighting matches before the start;
  drawing the 9-slice guides on the canvas in symbol edit mode and scaling by them.
- **Estimate:** 14–24 h.

## G3 3D and Materials

- **Done:** an initial live Revolve in the `three-d` crate (#846, from the contributor who offered in #605), with automatic smooth vector shading, default-on circular caps and spike-free continuous rims;
  Extrude & Bevel, Inflate and Rotate with Position presets ([three-d.md](three-d.md), #1407). Colored Inflate has automatic smooth vector lighting from shared membrane normals. White-light Inflate keeps its existing appearance. All four take one to ten colored directional lights, with neutral ambient applied once, a light list and independent previewing controls. Standard, Diffuse, Top Left and Right select original rigs, shared with commands; manual settings show Custom.
- **Missing:** materials (base, Substance, graphics), shadows,
  ray-traced rendering, angular cut caps, exact visibility for intersecting and concave surfaces, 3D Classic,
  Turntable, 3D export (OBJ, USDA, glTF).
- **Impact:** specialist; Materials is the one menu stub left.
- **Estimate:** 25–45 h; plan in `plan/` before coding.

## G7 Views and windows

- **Done:** Selection artwork drag across visible document tabs (400 ms hover); lossless target copy, source/clipboard preservation and cancellation regressions. Cross-window dragging remains separate.

- **Done:** custom document ruler origin (#1212): corner drag with preview, double-click reset to the active artboard, relative X/Y inputs and native save/reopen.
- **Done:** View › Rulers › Change to Global / Artboard Rulers (`view.globalRulers`, Cmd+Alt+R): artboard rulers, a new document's, count from the active artboard, each artboard with its own zero point; rulers, X/Y fields and guide readouts follow the mode, which is saved with the document. Estimate unchanged.
- **Done (#2849):** View › Rulers › Show Video Rulers (`view.videoRulers`): the rulers show and count the active artboard in video pixels from its top-left corner, a pixel as many points wide as the artboard's Video Ruler Pixel Aspect Ratio and one point tall ([public source](https://helpx.adobe.com/illustrator/desktop/create-manage-artboards/organize-manage-artboards/set-video-display-options-for-artboards.html)); Hide Video Rulers returns them to the General unit and the ruler origin. Shift-snapping a new guide to ruler ticks still uses the General unit's ticks. Estimate unchanged.
- **Missing (noted for reporters, 2026-10-14):** custom toolbars (Window › Toolbars › New Toolbar, #1759);
  panel widths kept with a workspace (#1753). Inside the estimate below.
- **Done (#4135):** floating panels open in OS windows of their own on the desktop, so they can sit on another monitor; on the web and on Wayland they float inside the window.
- **Done:** View › New View… and Edit Views… ([public source](https://helpx.adobe.com/illustrator/using/viewing-artwork.html)): a document keeps up to 25 named views (zoom, centre and rotation), saved with the file; every one is listed by name at the bottom of the View menu (`view.goto1`–`view.goto25`), and choosing one changes the current window to it. Edit Views… lists them to rename or delete, applied together on OK as one undo step (`view.saved.edit {edits}`); a file's views are checked when it opens. Estimate unchanged.
- **Missing:** multiple windows and Window › Arrange (Cascade, Tile, Float in Window, Consolidate All Windows).
- **Estimate:** 15–25 h.

## G6 Automation and scripting

- **Missing:** a scripting surface over the command registry (Illustrator: JavaScript, File › Scripts;
  #1183 is open). Actions, Batch, Variables (text, visibility, image and graph), data set CSV import and
  export, and full agent control over MCP are done.
- **Estimate:** 4–8 h.
- **Done (2026-10-16):** the MCP server hears the client during a long call: `notifications/cancelled` stops a queued or running export (PDF pages, flattening, baking, drawing, artboards) without writing anything or changing the document, `ping` and the list methods answer at once, any other request that would wait behind a call running for two seconds or more is refused as busy (`-32003`, naming the call), and `_meta.progressToken` gets page progress (#2165, #2171, #2172). Reading the document during an export is not served (refused as busy); estimate unchanged.
- **Exposed in MCP:** `add_text {vertical: true}` uses the existing vertical point, area and path type commands. Horizontal writing remains the default; the broader automation gap and estimate are unchanged.

## G15 Performance budgets and a GPU renderer

- **Improved (#1257):** Craquelure, Mosaic Tiles and Stained Glass reuse squared distances during nearest-cell searches, with the original distance ordering retained near rounding/tie boundaries. A brute-force boundary regression and full-viewport pixel comparisons agree with the old result. `texture_drag_perf` reproduces a moved CMYK rounded rectangle at 190.07% and DPI 1.5; timings from a busy development machine are supporting evidence, with idle Windows drag QA still pending.
- **Missing:** an idle-machine run of `clerdovector-cli perf` at 50k paths (2026-10-11 at 25k paths, every budget
  met, 400% zoom 12.3 ms against 16 ms, measured; runs of 0.10.0 on Linux with 4 logical CPUs, reported in #1172
  on 2026-10-13, missed all three render budgets at 20k paths), a spatial index for zoomed-in frames, dirty-region
  rendering, the GPU backend spike (M5). Large-selection edits, cached live effects and Pathfinder Trim, Merge,
  Crop and Outline landed in #1173.
- **Estimate:** 15–25 h (the budget run and fixes block beta).

## G16 Localization: the three missing key languages

- **Evidence:** Brazilian Portuguese catalog complete under the tests (menus, literals, messages; one|other plurals; #1914). Vietnamese catalog complete under the tests (menus, literals, messages; one plural form). Czech render-site bypasses reduced in file dialogs, diagnostics, preset names and contributor tooltips; gradient grammar and printer-mark context corrected (see [localization-parity.md](localization-parity.md)). Native review and the remaining missing-language work remain.

- **Missing:** Hindi, Arabic, Indonesian catalogs; a Korean UI font for the web build; a right-to-left interface and interface
  shaping for Arabic and Devanagari; messages for Simplified Chinese; native review of
  every catalog. Measured in [localization-parity.md](localization-parity.md).
- **Estimate:** 50–85 h plus human review (Brazilian Portuguese completed 2026-10-14).

## G17 Ecosystem: third-party plug-ins

- **Missing:** Illustrator's ecosystem is a C++ plug-in SDK and a large commercial plug-in market. ClerdoVector
  has sandboxed WebAssembly plug-ins for object filters, live effects and file formats it writes
  ([plugins.md](plugins.md)) and MCP; missing are tool plug-ins, panel plug-ins, importers, a plug-in directory and
  any third-party plug-ins (#710).
- **Done:** exporter plug-ins (`"kind": "exporter"`). A plug-in adds a write-only file format that File › Export ›
  Export As lists after the built-in ones and `document.export` writes (`plugin.<id>`, or a path ending in one of its
  extensions). It is given the document as the native file's JSON and its parameters, with a larger budget than a
  filter's, which Background Export's Cancel stops; the host writes the file it returns, and nothing when it declines
  or returns bad data. Engine, ABI and headless Export As regressions.
- **Done:** a plug-in's parameters say how its dialogs show them: `label`, `unit`, `help`, `helpUrl`, `group`
  (sections that fold) and `visibleWhen` (shown only while the checkboxes and choices it names have one of the
  listed values). `string` and `numberList` parameters join the others, and a plug-in declares up to 256, at most 64
  outside groups. The filter, live-effect and exporter dialogs use them, and the Appearance panel's effect editor the
  labels, units, help and conditions; hosts from before the keys ignore them. Manifest unit tests and kittest dialog
  regressions.
- **Done:** a live effect is given its object's fills and strokes, read only, as a filter is, and `_context.item`
  says which fill or stroke it sits on; a change of paint runs it again. Engine and unit regressions.
- **Done:** a filter is given its objects' effect records and their strokes' join, cap and miter limit, and may
  return both: the records are admitted as `effect.apply` admits them (one refused refuses the output), so a tool can
  set up objects for a live effect in one step. A path a filter hands back unchanged stays a live shape. Engine and
  unit regressions.
- **Done:** a filter keeps data with the document (`documentData`, at most 1 MiB, in the same undo step as its
  objects) and reads it back in `_context.documentData`. It is the document's foreign data (`unknown.pluginData`), so
  the native file keeps it with no section of its own, and an exporter finds it in the document it is given. Engine
  and unit regressions.
- **Done:** a filter may ask for a heavy budget (`"budget": "heavy"`: 1 G instructions and 300 s a run), for routing,
  packing or simulation. Installing it asks the user first (`allowHeavy`; folders skip it), and the app runs it on a
  thread of its own with a progress window and Cancel, applying its result as one undo step unless the document
  changed meanwhile. ABI, engine and headless UI regressions.
- **Estimate:** 20–45 h for the APIs; the ecosystem itself isn't agent work.

## G18 Generative AI features

- **Missing:** Text to Vector Graphic, Generative Recolor, Generative Shape Fill (#766), Generative Expand,
  Concept to Vector, Prompt to Edit, Rewrite Text. Adobe's run on its cloud models; ours would be a pluggable
  provider.
- **Needs:** an owner decision on models and providers.
- **Estimate:** 30–60 h after that decision.

## G19 Accessibility

- **Missing:** AccessKit is on by default, so egui widgets reach screen readers, but custom panels, the canvas and its objects aren't described and nothing has been tested with VoiceOver, Narrator or Orca; keyboard-only canvas operation; high
  contrast. Illustrator's own accessibility is limited, so this is mostly beyond parity.
- **Done:** PDF export writes `/ViewerPreferences << /DisplayDocTitle true >>` when File Info has a Document Title (or an explicit export title is supplied), so viewers and accessibility checkers use that title instead of the file name (#3646).
- **Estimate:** 15–25 h.

## Highlighted-text Eyedropper (2026-10-21)

Type/Eyedropper switches preserve highlighted characters; sampling changes only their enabled
font/paint attributes. Enabled non-neutral source or target object transparency is refused
atomically; disable Transparency to preserve object transparency while sampling the range.
`tests/eyedropper.rs` covers ranges, Unicode, history and transparency; tool tests cover brush-key
sampling coexistence. Scores and estimates unchanged. See [development.md](development.md#highlighted-text-eyedropper-2026-10-21).

## Drag artwork between document tabs (2026-10-22)

Selection drags activate another visible document tab after 400 ms, then copy editable artwork
on canvas release with the original grab point. Source and clipboard preservation, resource
rendering, swatch conflicts, cancellation, rotated/zoomed views and target Undo/Redo have regression
coverage (`tests_document_drag`, `tests_clipboard`). Overflow-menu and cross-window drag remain
separate. Scores and hours unchanged. [Behavior and command details](development.md#drag-artwork-between-document-tabs-2026-10-22).

## Pathfinder › Trap (2026-10-23)

The Pathfinder panel menu's Trap… (`object.pathfinder.trap`) and Effect › Pathfinder › Trap…
(`pathfinder.trap`) ship, from the public [trapping](https://helpx.adobe.com/illustrator/using/trapping.html)
and [Pathfinder panel](https://helpx.adobe.com/illustrator/using/pathfinder-panel-overview.html) pages: where two
objects' visible areas of different solid colours meet, an overprinting strip of the lighter colour goes into the
darker one (Reverse Traps: the other way, but not a rich black), with Thickness, Height/Width, Tint Reduction and
Traps with Process Color, and in the effect Precision and Remove Redundant Points. The panel's Trap is one undo
step and groups the traps above the topmost selected object. Effect › Pathfinder is complete (13/13); the Live
vector effects missing list no longer names Hard Mix, Soft Mix or Trap. Left: gradient and pattern fills don't
trap, and the defaults (0.25 pt, 100%, 40%) are ClerdoVector's own, the pages give none. Tests:
`crates/engine/src/tests/trap.rs`, `crates/effects/src/trap.rs`, `crates/pathops/src/trap.rs`.

## EPS Save evidence (2026-10-26)
Ordinary Save and Save As keep EPS as EPS, with the chosen page frame, transparent margins and the complete editable ClerdoVector document. Existing import-loss confirmation and transparency warnings remain; the Save TIFF preview is capped at 1024 pixels, while Export retains 2048. See [file-format-parity.md](file-format-parity.md#eps-save-and-source-protection) for limits and regression evidence. This narrows G9 save-back workflow coverage; native Illustrator editing-data fidelity, parity scores and hour estimates are unchanged.
