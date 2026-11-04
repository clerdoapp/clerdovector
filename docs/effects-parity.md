# Effects parity

> **Last reviewed:** 2026-10-10 · **Last updated:** 2026-10-28 · **Change:** minor (Effect dialogs clamp bounded length inputs immediately on commit and flash rejection on invalid input (#4638); previously: Drop Shadow Darkness uses fixed RGB subtraction and CMYK black addition; scores unchanged; Pathfinder › Trap: the Pathfinder panel's Trap… and Effect › Pathfinder › Trap… add an overprinting strip of the lighter colour where two solid colours meet; recounted with Hard Mix (#3061) and Soft Mix already in the catalogue: Pathfinder 13/13, Illustrator effects 49/50) · **Target:** Adobe Illustrator 2026 (30.x)

Every entry of Illustrator's Effect menu, by its public feature name, against ClerdoVector's effects catalogue
(`effect_catalog()` in `crates/effects/src/lib.rs`) and the Effect menu built from it
(`crates/ui-egui/src/menus.rs`). Counts are **measured** from the catalogue; "depth" notes are estimated.
Part of [target-app-parity.md](target-app-parity.md); open work is in [gaps.md](gaps.md).

## Summary

| Group | Illustrator | ClerdoVector | % | Remaining (h) |
|---|---:|---:|---:|---:|
| Illustrator effects (vector and stylize) | 50 | 49 | 98% | 4–8 |
| Photoshop effects (raster, Effect Gallery) | 57 | 57 | 100% | 1–2 (Effect Gallery thumbnails and in-dialog preview) |
| 3D and Materials | 4 (+3 Classic) | 4 (initial: Extrude & Bevel, Revolve, Inflate, Rotate) | ~25% | 25–45 |
| Beyond Illustrator: Effect › Color Adjustments (6), effect plug-ins (WebAssembly) | — | 6 + plug-ins | — | — |

## Illustrator effects

| Submenu | Items | Ours | Missing |
|---|---|---:|---|
| Convert to Shape | Rectangle, Rounded Rectangle, Ellipse | 3/3 | — |
| Crop Marks | Crop Marks | 1/1 | — |
| Distort & Transform | Free Distort, Pucker & Bloat, Roughen, Transform, Tweak, Twist, Zig Zag | 7/7 | Free Distort's dialog has a handle on each corner of the object's bounds, moved by dragging or with the arrow keys; Transform Patterns and Scale Strokes & Effects on a fill's or stroke's own Transform effect (on the object, Transform has all three options and visible checkboxes, #885); Zig Zag's ridges are evenly spaced along each segment (#1558), and Smooth points make a continuous wave whose peaks and valleys have handles along the path (#2343); Roughen with Smooth keeps turn-backs smooth instead of cusps that draw as spikes (#1646) |
| Path | Offset Path, Outline Object, Outline Stroke | 3/3 | With Use Preview Bounds on, Align, Distribute and the Transform panel still measure type that carries Outline Object by its layout box (preview bounds leave out effects) |
| Pathfinder | Add, Intersect, Exclude, Subtract, Minus Back, Divide, Trim, Merge, Crop, Outline, Hard Mix, Soft Mix, Trap | 13/13 | Trap traps only solid fills (gradients and patterns hide what is below them but don't trap) and spreads each colour's visible area as a whole; the public page gives no default values, so Trap's 0.25 pt, 100% and 40% are ClerdoVector's own |
| Rasterize | Rasterize… (as a live effect) | 1/1 | — |
| Stylize | Drop Shadow, Feather, Inner Glow, Outer Glow, Round Corners, Scribble | 6/6 | Scribble Options' Settings presets |
| SVG Filters | Apply SVG Filter… and the filter list | 0/1 | the whole submenu (SVG filters on import round-trip already) |
| Warp | Arc, Arc Lower, Arc Upper, Arch, Bulge, Shell Lower, Shell Upper, Flag, Wave, Fish, Rise, Fisheye, Inflate, Squeeze, Twist | 15/15 | warp point editing in envelopes; Arc is a circular arc in the object's own proportions (#1578) |

Document Raster Effects Settings exists. When the 64 million pixel cap on an effect image lowers
its resolution, PDF export warns once per object with the requested and the lowest achieved ppi
(`crates/engine/src/tests/pdfraster.rs`).

## Photoshop effects

| Submenu | Items | Ours | Missing |
|---|---|---:|---|
| Artistic | Colored Pencil, Cutout, Dry Brush, Film Grain, Fresco, Neon Glow, Paint Daubs, Palette Knife, Plastic Wrap, Poster Edges, Rough Pastels, Smudge Stick, Sponge, Underpainting, Watercolor | 15/15 (approximate Colored Pencil, Cutout, Dry Brush, Film Grain, Fresco, Neon Glow, Paint Daubs, Palette Knife, Plastic Wrap, Poster Edges, Rough Pastels, Smudge Stick, Sponge, Underpainting, Watercolor) | — |
| Blur | Gaussian Blur, Radial Blur, Smart Blur | 3/3 | — |
| Brush Strokes | Accented Edges, Angled Strokes, Crosshatch, Dark Strokes, Ink Outlines, Spatter, Sprayed Strokes, Sumi-e | 8/8 | — |
| Distort | Diffuse Glow, Glass, Ocean Ripple | 3/3 | — |
| Pixelate | Color Halftone, Crystallize, Mezzotint, Pointillize | 4/4 | — |
| Sharpen | Unsharp Mask | 1/1 | — |
| Sketch | Bas Relief, Chalk & Charcoal, Charcoal, Chrome, Conté Crayon, Graphic Pen, Halftone Pattern, Note Paper, Photocopy, Plaster, Reticulation, Stamp, Torn Edges, Water Paper | 14/14 (approximate Bas Relief, Chalk & Charcoal, Charcoal, Chrome, Conté Crayon, Graphic Pen, Halftone Pattern, Note Paper, Photocopy, Plaster, Reticulation, Stamp, Torn Edges, Water Paper; Chalk & Charcoal, Conté Crayon, Graphic Pen, Note Paper and Water Paper defaults unverified) | — |
| Stylize | Glowing Edges | 1/1 | — |
| Texture | Craquelure, Grain, Mosaic Tiles, Patchwork, Stained Glass, Texturizer | 6/6 | — |
| Video | De-Interlace, NTSC Colors | 2/2 | — |
| Effect Gallery | the combined browser and stack dialog | 1/1 (Effect › Effect Gallery…, #2883: six categories, per-filter settings, ordered stack with add/reorder/hide/remove, live canvas preview, one undo step; `dialogs::effect_gallery::tests::stack_ok_is_one_undo_step`, `stack_preview_is_one_interaction`) | filter thumbnails; a preview area inside the dialog ([Filter Gallery overview](https://helpx.adobe.com/photoshop/desktop/effects-filters/get-started-with-filters/filter-gallery.html)) |

The pixel-filter pipeline (`clerdovector_effects::pixel`) is in place: each new filter is one function, a
catalogue row, a menu entry and translations. Measured throughput: 0.7–1.2 agent hours per filter
(see [calibration](target-app-parity.md#calibration)). Every Photoshop-style filter is in the catalogue, and the Effect
Gallery dialog shipped (#2883); its filter thumbnails and in-dialog preview area are the 1–2 h that remain.

Film Grain (`artistic.filmGrain`, Effect › Artistic) adds stable monochrome noise to shadows and
midtones, with Grain, Highlight Area and Intensity sliders, live preview and appearance editing.
Its smooth grain field follows translations, preserves transparency and stays in document units;
zoomed-out rendering uses bounded approximate footprint filtering. Changing the bounds shifts
the pattern. Command exports bake it to an image in SVG and PDF, while the native document keeps the effect editable. Behaviour is an original visual
approximation based on [Illustrator's public effects reference](https://helpx.adobe.com/illustrator/using/summary-effects.html)
and the [public Artistic filters guide](https://helpx.adobe.com/photoshop-elements/using/artistic-filters.html),
not a pixel-identical proprietary implementation. Intensity is interpreted as highlight strength and
quieter highlight grain. Grain strength stays even across shadows and midtones and rolls off above
mid-grey to a quarter-strength floor at full Intensity before highlight headroom; cream retains
soft grain, black takes light speckles and white stays clean. The tonal curve and endpoint behaviour
are interpretations, not verified proprietary behaviour. Saturated colour channels may clip. The parameter
ranges and defaults are chosen for ClerdoVector, not asserted as measured proprietary defaults.

Cutout, Neon Glow and Poster Edges (`artistic.cutout`, `artistic.neonGlow`, `artistic.posterEdges`)
follow the same two public references, with original algorithms. Cutout softens the object's colours (Edge Simplicity, less with Edge
Fidelity), splits each colour channel into Number of Levels steps, and paints each resulting piece
in the average colour that falls into it, so hues survive. Poster Edges
posterizes each colour channel to Posterization + 2 levels and draws black lines on the edges of
any channel and of the object's outline, wider with Edge Thickness and darker, on fainter edges
too, with Edge Intensity. Neon Glow redraws the object in dimmed greys screened with a glow colour
from its light areas (positive Glow Size) or its dark ones (negative), spread by the size and as
strong as Glow Brightness; with no foreground and background colours in Illustrator's model, the
glow colour is the effect's own parameter. All three keep coverage, work in document units at any
Document Raster Effects resolution, and use the generic effect dialog with live preview. Ranges and
defaults are chosen for ClerdoVector. Evidence: `crates/effects/src/pixel/artistic.rs`, the
`artistic_*`, `cutout_*`, `poster_edges_*` and `neon_glow_*` tests in `crates/effects/src/pixel/tests.rs`,
and `artistic_settings_survive_a_save_and_open` in `crates/engine/src/tests/rasterfilters.rs`.
Smudge Stick (`artistic.smudgeStick`), Sponge (`artistic.sponge`) and Watercolor (`artistic.watercolor`),
Effect › Artistic, are original visual approximations from the public descriptions in
[Illustrator's effects reference](https://helpx.adobe.com/illustrator/using/summary-effects.html) and the
[public Artistic filters guide](https://helpx.adobe.com/photoshop-elements/using/artistic-filters.html), not
pixel-identical copies. Each has live preview, appearance editing, document-unit patterns that follow the
object, bounded work per pixel and kept coverage (they paint nothing outside the object), and bakes to an
image in SVG and PDF. Ranges and defaults are chosen for ClerdoVector, not asserted as measured.
- Smudge Stick (Stroke Length 0–10, Highlight Area 0–20, Intensity 0–10): dark areas smudged along short
  diagonal strokes; light areas smudged with strokes twice as long and lightened towards white.
- Sponge (Brush Size 0–10, Definition 0–25, Smoothness 1–15): the object softened, then lighter and darker
  blobs of the same hue, their contrast set by Definition, their edges by Smoothness.
- Watercolor (Brush Detail 1–14, Shadow Intensity 0–10, Texture 1–3): detail simplified by an edge-keeping
  average whose brush widens as Brush Detail falls, colour saturated and pooled darker where tones change at
  edges, shadows deepened, a faint paper grain. Edges are found from tone only, so a change of hue at equal
  brightness is not saturated.

Tests: `pixel::tests_paint::*`, `tests_rasterfilters`, `dialogs::tests::raster_filter_dialogs_*`,
`tests_labels::effect_menu_lists_the_artistic_filters_*` and the `command_sweep` structured junk.

Near white, highlight headroom can limit grain equally at every Intensity; highlight lift still
depends on Intensity when Highlight Area is nonzero.
The two rotated, variance-normalised noise layers can leave a faint diagonal weave at high
magnification. Removing that remaining direction would require a more expensive field; the
current implementation keeps bounded per-pixel work. Film Grain is still slower than Texture
Grain (Regular), particularly when its wider footprint needs four samples.
Calibration on uniform grey at Grain 20, using 32×32 fine samples per coarse pixel, measured
grain standard deviation within about 15% of the dense area average at pixel widths of 2, 4
and 8 document units (ratios 1.13, 0.89, 0.85). This is evidence for the bounded approximation,
not an accuracy guarantee for arbitrary artwork. Tests: `pixel::tests::film_grain_*`,
`tests_rasterfilters`, `tests_adjust::film_grain_dialog_*` and `tests_labels::effect_menu_lists_the_artistic_filters_*`.

Smudge Stick (`artistic.smudgeStick`), Sponge (`artistic.sponge`) and Watercolor (`artistic.watercolor`),
Effect › Artistic, are original visual approximations from the public descriptions in
[Illustrator's effects reference](https://helpx.adobe.com/illustrator/using/summary-effects.html) and the
[public Artistic filters guide](https://helpx.adobe.com/photoshop-elements/using/artistic-filters.html), not
pixel-identical copies. Each has live preview, appearance editing, document-unit patterns that follow the
object, bounded work per pixel and kept coverage (they paint nothing outside the object), and bakes to an
image in SVG and PDF. Ranges and defaults are chosen for ClerdoVector, not asserted as measured.
- Smudge Stick (Stroke Length 0–10, Highlight Area 0–20, Intensity 0–10): dark areas smudged along short
  diagonal strokes; light areas smudged with strokes twice as long and lightened towards white.
- Sponge (Brush Size 0–10, Definition 0–25, Smoothness 1–15): the object softened, then lighter and darker
  blobs of the same hue, their contrast set by Definition, their edges by Smoothness.
- Watercolor (Brush Detail 1–14, Shadow Intensity 0–10, Texture 1–3): detail simplified by an edge-keeping
  average whose brush widens as Brush Detail falls, colour saturated and pooled darker where tones change at
  edges, shadows deepened, a faint paper grain. Edges are found from tone only, so a change of hue at equal
  brightness is not saturated.

Tests: `pixel::tests_paint::*`, `tests_rasterfilters`, `dialogs::tests::raster_filter_dialogs_*`,
`tests_labels::effect_menu_lists_the_artistic_filters_*` and the `command_sweep` structured junk.

Bas Relief, Charcoal, Photocopy and Stamp (`sketch.basRelief`, `sketch.charcoal`, `sketch.photocopy`,
`sketch.stamp`, Effect › Sketch; `pixel/sketch_ink.rs`) redraw the object in black ink on white paper
and keep its coverage. Bas Relief carves low relief lit from one of eight directions (Detail, Smoothness,
Light Direction); Charcoal draws bold edges and sketches the midtones in diagonal strokes (Thickness,
Detail, Light/Dark Balance); Photocopy prints only where a pixel is darker than its surroundings, so large
dark areas copy around their edges (Detail, Darkness); Stamp reduces the object to solid black and
white (Light/Dark Balance, Smoothness). Sizes are in document points at any Document Raster Effects
resolution, strokes and noise lie in document space around the object's centre, and all four preview
live. Behaviour is an original visual approximation based on
[Illustrator's public effects reference](https://helpx.adobe.com/illustrator/using/summary-effects.html)
and the [public Sketch filters guide](https://helpx.adobe.com/photoshop-elements/using/sketch-filters.html),
not a pixel-identical proprietary implementation; the parameter ranges and defaults are chosen for
ClerdoVector, not asserted as measured proprietary defaults. Tests: `pixel::tests_sketch::*`,
`tests_rasterfilters`, `command_sweep`, `dialogs::tests::raster_filter_dialogs_preview_and_apply` and
`tests_labels::effect_menu_lists_the_sketch_filters_*`.

Effect › Sketch › Halftone Pattern and Note Paper (`sketch.halftonePattern`, `sketch.notePaper`) are
original black-and-white filters guided by [Illustrator's public Sketch effects summary](https://helpx.adobe.com/illustrator/using/summary-effects.html)
and [effects overview](https://helpx.adobe.com/illustrator/using/effects.html) (Halftone Size in points at any Document Raster Effects resolution).
The public help pages name each control but publish no numeric defaults or slider limits; ClerdoVector's ranges
follow those control names and the emboss/halftone behaviour described there, while dialog defaults are chosen
for ClerdoVector and are not verified against proprietary defaults. Evidence: `crates/effects/src/pixel/sketch.rs`,
`halftone_*` and `note_paper_*` in `crates/effects/src/pixel/tests.rs`, `SKETCH` and
`halftone_pattern_settings_survive_a_save_and_open` in `crates/engine/src/tests/rasterfilters.rs`, and
`tests_labels::effect_menu_lists_the_sketch_filters_between_sharpen_and_stylize`.

Effect › Sketch › Conté Crayon (`sketch.conteCrayon`; `pixel/sketch.rs`) redraws the object in a dense
black crayon where it is dark and a pure white crayon where it is light, on mid-grey paper, as
[Illustrator's public effects summary](https://helpx.adobe.com/illustrator/using/summary-effects.html) describes
(black for dark areas, white for light areas). Its controls follow the public
[Photoshop filter reference](https://helpx.adobe.com/photoshop/using/filter-effects-reference.html) and the
[Photoshop Elements Sketch filters guide](https://helpx.adobe.com/photoshop-elements/using/sketch-filters.html)
(foreground and background levels, texture options): Foreground Level and Background Level (0–20) set how far
each crayon reaches into the midtones; Texture (Brick, Burlap, Canvas, Sandstone, the surfaces Texturizer
draws), Scaling, Relief, Light Direction and Invert make the paper's tooth, which breaks the diagonal crayon
strokes, and its relief. The public pages publish no slider ranges or defaults, so those are unverified
ClerdoVector choices; an original approximation, never compared with another application's output. Tests:
`pixel::tests_sketch::conte_crayon_*`, `tests_rasterfilters`, `command_sweep`,
`dialogs::tests::raster_filter_dialogs_preview_and_apply` and
`tests_labels::effect_menu_lists_the_sketch_filters_between_sharpen_and_stylize`.

## 3D and Materials

| Item | Ours |
|---|---|
| Revolve | initial: live, editable profile, rotation and light gizmos, default-on Cap with continuous curved rims, smooth vector shading and round seam joins, SVG/PDF output, Expand Appearance ([revolve.md](revolve.md), #846) |
| Extrude & Bevel, Inflate, Rotate | initial: live, depth, twist, taper, solid or hollow caps, five original bevel shapes, Inflate volume, both sides, twist, taper and cap, Rotate of every fill and stroke, Position presets, rotation and light gizmos, SVG/PDF output, Expand Appearance ([three-d.md](three-d.md)) |
| Materials panel, lighting, ray-traced render | one to ten directional lights with per-light Color (opaque RGB, white by default), Intensity, Rotation, Height, Softness and neutral Ambient Light, Revolve and colored Inflate use smooth vector shading; Extrude & Bevel and Rotate retain planar shading; original Standard, Diffuse, Top Left and Right lighting rigs; no materials, shadows or ray tracing |
| 3D (Classic): Extrude & Bevel, Revolve, Rotate | none |
| Turntable (30.0 beta) | none |
| Export 3D objects (OBJ, USDA, glTF) | none |

The `three-d` crate (depends on `geom` and `pathops`, with no UI or GPU dependencies) builds, rotates, projects, shades and depth-sorts the surfaces;
the effects crate turns them into art. What's left (materials and shadows, ray-traced rendering,
angular cut caps, exact visibility, 3D Classic, Turntable, 3D export) is 25–45 h.
