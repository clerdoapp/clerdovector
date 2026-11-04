# File format parity

> **Last reviewed:** 2026-10-10 · **Last updated:** 2026-11-01 · **Change:** minor (native format: dynamic symbols, #4938) · **Target:** Adobe Illustrator 2026 (30.x)

Every format Illustrator opens, places, saves or exports, against ClerdoVector's format table (`FORMATS`,
`OPEN_EXTS`, `PLACE_EXTS` in `crates/engine/src/cmd/fileio/formats.rs`). Illustrator's list comes from its public
documentation (helpx "file formats" and Save/Export pages), not from its installed bundle: the clean-room rule
forbids reading the install. Part of [target-app-parity.md](target-app-parity.md).

**Fidelity** is estimated unless a test is named. Test sources: unit and property tests per crate
(`crates/svg/tests`, `crates/pdf/tests`, `crates/format/tests/prop_format.rs`), import fuzzing
(`crates/engine/tests/import_fuzz.rs`), the Affinity corpus (`cargo xtask corpus --affinity`, 51 pinned public
files, CI job `affinity-corpus.yml`). There is **no real-world Illustrator corpus**: test inputs are written
from the public specifications, and users' own `.ai` files are used locally only.

## Summary

| | Illustrator | ClerdoVector | Notes |
|---|---:|---:|---|
| Formats opened or placed | ~30 | 28 | missing: FreeHand, CorelDRAW, DWG, HEIC/HEIF, PICT, Pixar, PS |
| Formats saved (Save As) | 6 (AI, AIT, PDF, EPS, SVG, SVGZ) | 6 + native | `.ai` written PDF-compatible (no Illustrator editing data) or as legacy version 7 (layers and groups, one artboard per file) |
| Formats exported | ~16 | 17 | missing: DWG, 3D (OBJ/USDA/glTF); extra: GIF/PNG-8 palette export, DXF up to 2018, layered PSD |
| **Overall (weighted by use)** | | **~75%** | `.ai` and PDF dominate the weighting |

EPS editing-data follow-up (2026-10-16): both mesh-corner layouts (with and without opacity)
read in RGB, CMYK and gray. Vector opacity-mask art and its Clip, Invert, Disabled and Linked
flags are retained. Winding-declared legacy compound member groups keep their shared paint even
when followed by `9 () XW`. Fully rasterized print pages are compared using the same filtered
minification for both representations; the existing error thresholds and ordinary vector-page
checks remain. Synthetic regressions reject truncated meshes, non-unit rational weights,
unbalanced mask art, unknown mask operators, missing artwork and wrong page framing. Real
user files remain local; no SVG replacement or tracing is used. Small rendering differences
continue to produce import warnings.

## The main format: `.ai`

The format that decides beta. Illustrator's `.ai` is a PDF (or, for old files, PostScript) plus Illustrator's
private editing data. ClerdoVector:

- **Reads** the PDF page (vectors, text, images, colour, transparency, layers via optional content) and the
  *structure* of the editing data (below). Opens Illustrator EPS the same way through its own PostScript
  Level 3 interpreter. A `.ai` in PostScript form (versions 3 to 8, and what Rhino and other CAD apps
  write) is its own editing data: it opens through the layers its program has, compared with its page
  when the program runs, from the layers alone when it names its prolog without including it (#1027), and with its art on one
  layer when it has no layers (Illustrator 3, and image editors' exported paths).
  RGB in the PDF page's `ICCBased` spaces keeps the file's numbers in a `.ai` (the `uncalibrated_rgb`
  import option, on for editor documents, off for PDFs), so `.ai` files tagged with a non-sRGB RGB profile
  open with different, unconverted RGB colours than before.
  A PDF-compatible `.ai` opens on its PDF part's trim boxes (its artboards), the bleed boxes round them
  becoming the document's bleed, so art in the bleed shows outside the artboard, within the bleed guide (#1266).
  A PDF saved with its editing data (`AIPDFPrivateData`) opens the same way.
- **Writes** a PDF-compatible `.ai` (Save As › `ai`): Illustrator opens it as plain art. It never writes
  Illustrator's private editing data, which is undocumented.
- **Writes** a legacy `.ai` of version 7 (Save As › `ai` › Version: Legacy (version 7), `aiVersion: "7"`): the
  PostScript form, from the published *Adobe Illustrator File Format Specification* version 7.0
  ([PRONOM fmt/423](https://www.nationalarchives.gov.uk/PRONOM/fmt/423)), which apps reading `.ai` open with its
  structure (#868). It keeps layers (names, order, visibility, lock), groups, compound paths, clipping groups,
  solid colours, linear and radial gradients and RGB images. The format has one artboard: each artboard chosen in
  the options dialog's artboard picker (thumbnails to check, default all) is a file of its own,
  `{name}-{artboard}.ai` when there are several. It has no transparency: objects are written opaque, and images
  with see-through pixels, opacity or a blending mode are flattened (an opaque image of themselves and the art
  under them, clipped to where they show; that art stays) or omitted (Transparent Images,
  `transparentImages: "flatten"|"omit"`). Type is written as editable text objects (point type, area type and type
  on a path, with their fonts' PostScript names, sizes, colours, alignment, indents, leading, tracking and
  scaling), which the reading app sets in those fonts; characters outside Windows-1252, vertical type, ruby, inline
  graphics and gradient or pattern fills make that type outlines, and Text: Outlines (`outlineText: true`) writes
  all type as outlines. The format can hide only layers: hidden objects go to a hidden layer of their own above
  their layer, are omitted, or are made visible (Hidden Objects, `hiddenObjects: "layer"|"omit"|"visible"`).
  Sublayers become groups; appearances, effects and brushes are expanded; object names are left out; layer
  names, spot color and gradient names and the title are written in Windows-1252, with `?` for characters outside
  it (two spot colors or two gradients whose names become the same are numbered).
  The file is also a PostScript program that runs: its prolog, a procedure set of ClerdoVector's own written from
  the PostScript Language Reference, gives the format's operators their meaning, so it prints and previews
  (gradients filled with their first stop's colour, type left to the apps that read the format). ClerdoVector
  opens a file with that prolog from its program, as the format.
- **Known failures in users' files:** layer structure flattened or multiplied (#951: 109 layers instead of
  14; #868: round trip with Illustrator 2018 loses layers and groups), guides and non-printing construction
  lines lost (#779), some files 4× slower to open since the editing data is read whole (#758), live effects,
  brushes and symbols come in as their drawn look (#637; pattern fills are read since #1025); legacy `.ai`
  without the prolog opens since #1027, its legacy type (`To` … `TO`) since #868.
- Estimate: **~65%** for real exchange with Illustrator users (open: ~75%; save: PDF-compatible, or legacy
  version 7 with layers and groups; not yet tested by users who hand files back).
  Closing the open-side issues: 10–20 h with users' files.

### Illustrator's private editing data

the app's own editing copy of a file (an EPS after its `%%EOF`, a `.ai`'s `AIPrivateData` streams, a PDF's `AIPDFPrivateData` streams when it was saved with the editing data (#868), Zstandard or zlib, the EPS's in ASCII85) is not documented as a whole. ClerdoVector reads its structure: layers and sublayers (names, order, visibility, lock, printing, preview, dimming, colour), groups as they were nested, compound paths (painted by their last path, or the last painted one, with the paths of groups inside them kept as their paths: #1410), clipping groups and layer clipping, object names, hidden and locked objects, fills and strokes (grey, CMYK, RGB, named and spot colours, spot colours defined in Lab with their Lab values (#1032), global process colours as global swatches at their tint), linear and radial gradients, pattern fills with their patterns as pattern swatches (#1025), opacity, blend modes, isolation and knockout, embedded images where their matrix puts them, rotated ones too (#2269), with their alpha channel (CMYK images keep their ink amounts, as they do when a `.ai` opens from its PDF part, and an alpha channel that hides part of a CMYK image becomes a grayscale opacity mask), placed (linked) files as linked images in their place, each showing the low-resolution preview the file keeps of it until its file is found or relinked in the Links panel (#1264), guides, and every artboard where it is. Objects on Path groups open live, with their path, their objects in order along it, Pivot and Rotate; one with a dragged spacing or other options opens as a plain group. A gradient with stops in the middle runs its whole ramp across its shape, and a gradient object does not pass its fill to the next path (#1390). An object whose art style is one fill and one stroke at most (solid colours, its own gradient, a variable-width profile) with an Outer Glow or Drop Shadow comes in as itself with them; one with more, other effects, a brush or dashes aligned to corners comes in as its drawn look (a group named after it). Type keeps its art style as a live appearance whatever its stack, so it can be retyped: each fill and stroke of the style's stack (solid colours, gradients and pattern fills, placed as the drawn look places them) with its own opacity, blend mode and effects, the effects on the whole object, and where the characters paint among them; the effects read are Warp, Offset Path, Transform (with copies), Drop Shadow, Outer Glow, Feather, Gaussian Blur and Bas Relief (its settings at their defaults), and a style with others keeps the type's drawn look. The page's drawing of such a style isn't held to look like ClerdoVector's where its drawn look lay. Gradients take their matrix (`Xm`, radials and skewed ones too), focal point and the order of stops sharing a ramp point; gradient meshes, a clipping path's own fill and the artboards' names are read too. `editingData: "always"` (`convert --editing-data`) opens a `.ai` or an Illustrator EPS from its editing data even where it draws differently from its page, which can be older, with a note. Type is made from the file's text document, whole and editable, shown or not: point type (its frame's matrix, which newer files leave out when it is neither turned nor scaled, read as none), area type (a story in several frames as threaded type) and type on a path, with its characters, fonts, size, leading, tracking, scaling, baseline shift, fill and stroke (grey, RGB or CMYK), alignment, indents and paragraph spacing (a character attribute a run doesn't set comes from its character style, then the style that one is based on, then the document's defaults); the page's drawing of it (often in pieces) is left out. When it makes the type of every text object that shows, the page's type is only compared with it, each text object keeps its own group, frame and layer, and a line break inside a paragraph stays one. Where it can't make a text object's type (a frame of another kind), that text object takes the page's type where the text document puts its lines, or else what is left of the page's type in painting order (no group around it); the page's type where made type draws stays that type's, and hidden type is left out with a note. Kerning, underline and the other character and paragraph options aren't read yet. A `.ai` saved without PDF compatibility (a placeholder page) opens from its editing data alone, its point type, area type and type on a path made from the text document. A `.ai` whose PDF part shows only text isn't taken for that placeholder when its page states an art, trim or bleed box: it opens from its editing data alone where that reads, else from its PDF part (#1387). A `.ai` whose PDF part draws nothing at all (an empty page, with or without those boxes) opens from its editing data alone where that reads too: there is no page art to check its layers against, and the empty page would lose all of it. Text objects with an appearance keep their own text and frame, type of hidden layers stays hidden, and type on later artboards lands on its text objects (#1391); area type whose frame is too short for a line of the fallback font shows its first line (the frame is made taller, with a warning in the open result) (#1392). Symbols, pattern fills whose pattern it can't read, placed files the data keeps no preview of and anything else it doesn't read make the file come in as its page (on a layer that doesn't show, they are left out of that layer), with a warning saying what. An unread object on a shown layer whose box can be found instead comes in as a group of the page's objects inside that box, named after it, with the rest of the layers and groups kept and a warning listing what came in as the page's art; the file still comes in as its page when live art or another unread object shares that page art, when the object spans two artboards, or when the page art doesn't carry its transparency (#4806). The layers' art is also compared with the page (by lightness, so colours written as their CMYK equivalents compare alike, and by whether a pixel is inked at all); the page is used when they differ by more than 5% (away from type, when all the type is made from the text document), where one draws an object the other doesn't (away from type, whose lines may be laid out afresh), and when an EPS whose page is its art's box has art printed outside it (#735). An EPS whose page is its art's box keeps that page when it comes in through its layers, not the editing data's artboard, and a text object counts as drawn by the page by its colours, not only its lightness, so a red stroke stacked over a dark red one stays (#1102). `textAs: "outlines"` opens a `.ai` whose type shows from its PDF part, which has the outlines; `editingData: false` opens any of them as its page or PDF part only, for print pipelines (#735). A file whose import left something out (hidden text, art or layers) is never written over by an export or Save As to its own path unless `acknowledgeLoss: true` (`convert --replace-lossy`), so the original keeps what the document lacks. Where a `.ai` can't be read that way (or its layers are turned off in the import options), we read its PDF-compatible part, so Illustrator-only live objects arrive as appearance, and plain groups arrive ungrouped: that part draws each object on its own and marks only layers, clipping groups and groups with opacity, blending or a mask (those come back). Art outside the artboards isn't in that part at all (Illustrator's PDF part leaves it out), so it doesn't open; the import says so in a note (#472). Its pages still go where the editing data puts the artboards (a grid stays a grid), with the art on each, when it has as many artboards as pages, of the same sizes (#1068). A `.ai` whose PDF part's pages are cropped to its art (each page the box of all the art, which is the editing data's bounding box) instead of its artboards opens from its layers, with its own artboards, where they draw as that page does. Its pages are laid out so that art one of them draws past its artboard (notes beside a design, which the part doesn't clip) doesn't lie over the page before it in the row, where it was taken for that page's art: the check against the editing data then found art its layers don't have there and opened the file as its PDF part, without the art outside the artboards. That part also has no paragraphs: area type is written line by line, and comes back as area type only where its font is installed and lays the lines out with the same breaks, else as a point type object per line (#508). Art off the page that a PDF does draw opens on the pasteboard.

## Format by format

| Format | Illustrator | Ours: open/place | Ours: save/export | Fidelity and tests |
|---|---|---|---|---|
| ClerdoVector `.clerdovector` / `.vctemplate` | — | yes | yes | lossless JSON, versioned, save down to v1; property-tested round trips (`format/tests/prop_format.rs`); documents with a lot of path data are written as compact JSON, about a fifth of the size (#1492); a save falls back to a plain fsync and to writing elsewhere and copying in when the folder refuses the usual steps (#1625, #1649, part of #1620; not confirmed on a real network share); the size limits on embedded data also apply to files read from memory (#1783), and saving over a private file on Unix leaves no readable temporary copy beside it (#1778); a dynamic symbol's type (`dynamic` on the symbol) and an instance's appearance changes to its art (`symbol_overrides`, each path at most 64 deep, counted against the object budget) are written only when set, so documents without them save as before (#4938) |
| Illustrator template `.ait` | open, save | yes | PDF-compatible only, from Save as Template (`.vctemplate` stays its default) | New from Template opens a saved `.ait` untitled with its layers and art (`tests_pdfedit.rs`) |
| Illustrator `.ai` | open, save | yes (PDF-compatible + editing-data structure with template layers and ruler guides, File Info) | PDF-compatible, or legacy version 7 (layers and groups) | see above; no real-file corpus |
| PDF | open, place, save | yes (layers, masks, editable text, security, overprint, File Info from the XMP packet and the document information dictionary; Crop To › Trim takes the bleed box as the document's bleed) | yes (presets, PDF/X-1a/3/4, layers, marks and bleed, ICC, fast web view, 1.3+) | fuzzed import; shared guarded writer and windowed reader ([pdf-backend.md](pdf-backend.md)), metadata, incomplete-content and dependency-source regressions; PDF text opens where the file draws it, glyph by glyph (#722: kerning or none, spacing per character, text rise as baseline shift, vertical writing, justified text boxes); alpha soft masks drawn as images import with the mask (#1551); a clip only counts as covering the page when its shape does (#1542); a PDF another app copied pastes as editable vector art (#1499); letter-spaced type on a path keeps its length and its glyph-by-glyph stroke comes in as one text object (#1586, #1585); a crafted name tree with shared nodes no longer takes exponential time on open (#1781); fonts named with hyphens (`Noto-Sans-Bold`) or with a composite font's CMap (`Name-Identity-H`) find the installed family; invisible text (rendering mode 3: a scanned page's recognized text) opens as type with no fill or stroke and saves back as invisible, searchable text (#2675); PDF/X-1a and X-3 flattening in a CMYK document keeps CMYK inks where it rasterizes (#2160) |
| EPS | open, place, save | yes (own PostScript L3 interpreter, Illustrator EPS editing data structure) | yes (L2/L3, TIFF previews) | CID-keyed CJK type not read; #505; images encoded with `CCITTFaxDecode` (ITU-T T.4 Group 3 and T.6 Group 4, with `K`, `Columns`, `Rows`, `EndOfBlock`, `EndOfLine`, `EncodedByteAlign` and `BlackIs1`) draw, and damaged data keeps the rows decoded before its first damaged row; Type 42 (TrueType) fonts set type at its size (#1553); a clip only counts as covering the page when its shape does (#1542); an EPS another app copied pastes as editable vector art (`clipboard.importEps`, #1499); a stitched shading keeps its hard colour changes on import (#1654); CMYK images, CMYK JPEGs included, keep their ink amounts on import, as from PDF, and the pixels a `MaskColor` or `ImageType 3` mask hides become a grayscale opacity mask; ruzstd 0.9.1 for compressed editing data (#1807, RUSTSEC-2026-0344) |
| PostScript `.ps` | open, place | yes (the EPS reader reads its first page; `.epsf` files open and place as EPS) | via Print to PostScript | `tests_epsimport.rs` |
| SVG / SVGZ | open, place, save | yes | yes (SVG Options, Preserve Editing, symbols, filters, rich text) | strongest format; tests in `crates/svg/tests`; text on a path follows basic shapes as their SVG 2 equivalent paths (#1527); CSS transforms and the `font` shorthand (#1565), percentage lengths in symbols (#1596), Inkscape flowed text as area type (#1568) and Inkscape pages as artboards (#1601) import; an unsupported filter is named in its warning (#1598); an artboard exported on its own (Export As with Use Artboards, Export for Screens, Asset Export) has the artboard's name as `<title>`, Save the document's (#3544); attributes in other applications' XML namespaces (`plotter:pen="2"`) kept on their objects from import to export, as `data-*` attributes are (`crates/svg/tests/it/foreign_attributes.rs`); simple shapes export as `<rect>`, `<circle>`, `<ellipse>`, `<line>`, `<polygon>` and `<polyline>` where they draw the same as the path (#4652, `crates/svg/tests/it/basic_shapes.rs`) |
| DXF | open, export | yes | yes (R12–2018) | R13–2018 closed SPLINE flags and control-point closure on import verified (#1299); circular cubic paths export as native CIRCLE/ARC where stroke semantics permit; R12 weighted strokes remain POLYLINE while non-circular curves keep the existing fallback (`crates/cad/src/tests.rs`); [Autodesk group 70](https://help.autodesk.com/cloudhelp/2024/ENU/AutoCAD-DXF/files/GUID-E1F884F8-AA90-4864-A215-3182D47A9C74.htm). DXF 2000–2018 LAYOUT objects write the paper image origin as 148/149 (was a stray 158), the shade plot codes (2004+) and the model type flag, and the plot style placeholder carries no subclass marker (#3085; structure test `r2000_and_later_files_have_the_structure_cad_apps_require`, checked with ezdxf's audit; not yet confirmed in Autodesk Fusion). CNC preset and VCarve QA remain open |
| DWG | open, export | no | no | no open spec; DXF instead |
| EMF / WMF | open, export | yes | yes | an oversized embedded PNG or JPEG is refused from its header, before any pixels are decoded (#1776) |
| PSD | open, place, export | yes (merged image, every mode and depth) | yes (layered, max editability) | placing as layers (Photoshop Import Options) missing; the embedded ICC profile's tags are 4-byte aligned (#1584) |
| PSB | open | yes (merged image) | no | |
| PNG / JPEG / GIF / BMP / TIFF / Targa | open, place, export | yes (Targa: `.tga`, `.vda`, `.icb` and `.vst`, raw or run-length encoded, with a 16-bit file's one-bit alpha) | yes; TIFF is also selectable in Export for Screens and Asset Export with its existing options | Targa: `tests_tiffbmp.rs`, `tests/links.rs`; TIFF: grayscale with an alpha channel or other extra channels reads (#1466 in part), and so do files that store each channel as its own plane (PlanarConfiguration 2), CMYK ones with their ink amounts, and 1-bit files compressed with CCITT Group 3 (Compression 2 and 3, one- and two-dimensional rows); the ICC profiles embedded in JPEG and TIFF have 4-byte aligned tags (#1584); PNG export marks its colours as sRGB (`embedIcc`, an `sRGB` chunk; no ICC profile is embedded, #2221) |
| WebP | export (Export for Screens) | yes | yes (lossless only; lossy written lossless with a warning) | |
| HEIC / HEIF | open, place | no | no | |
| PCX | open, place | yes (1 bit in the header palette's first two colors, black and white without them; 1 bit in two to four planes and 4 bits with up to 16 colors; 2 bits in the CGA colors the header's CGA palette settings pick; 8 bits with the 256-color palette, gray without one; 24-bit color in three planes) | no | the header's HDpi and VDpi size the image; a file of another depth or plane count, cut short or past the pixel allowance is refused with a message; tests in `crates/engine/src/cmd/fileio/tests_pcxread.rs` and `crates/engine/tests/import_fuzz.rs` |
| JPEG 2000 | open, place | yes (JP2 and raw codestream images; multiple codestreams and JPX composition or layers are refused) | no | `.jpf`, `.jpx`, `.jp2`, `.j2k`, `.j2c`, `.jpc`; resolution uses the last valid `resc` or `resd` value on each axis, or 72 ppi; bounded decoding; `tests_jp2read.rs` |
| PICT, Pixar PXR | open, place (legacy) | no | no | rarely used |
| FreeHand `.fh*` | open (legacy) | no | no | |
| CorelDRAW `.cdr` | open | no | no | |
| Text `.txt` | place, export | place (Text Import Options) | yes | |
| RTF `.rtf` | place | place (area type with fonts, size, bold, italic, underline, strikethrough, caps, superscript and subscript, color, alignment, indents and paragraph spacing) | no | `tests/textimport.rs`, `place/rtf.rs`; fuzzed (`text_import_fuzz.rs`); `\'hh` bytes read as Windows-1252 or Mac Roman only |
| Word `.docx` | place | place (area type with the document's styles and its fonts, size, bold, italic, underline, strikethrough, caps, superscript and subscript, color, alignment, indents and paragraph spacing) | no | `tests/textimport.rs`, `place/docx.rs`; fuzzed (`text_import_fuzz.rs`); list numbers, pictures and text boxes are left out |
| Word `.doc` | place | place (area type with the same formatting as `.docx`, from the style sheet's styles and direct formatting; Word 97 and later) | no | `tests/textimport.rs`, `place/doc.rs`; fuzzed (`text_import_fuzz.rs`); pictures, headers, footers, notes, comments and text boxes are left out with a warning; encrypted and fast-saved documents are refused |
| CSS | export (CSS Properties) | — | yes | |
| OBJ / USDA / glTF | 3D export | — | no | needs 3D (gap G3) |
| Affinity `.af`, `.afdesign`, `.afpub` | no | yes (beyond Illustrator) | no (#665) | 51 pinned files, [affinity-validation.md](affinity-validation.md) |
| Clipboard: PDF, SVG, PNG, EMF, text | yes | yes | yes | copied files paste as art |

## PDF editable imports

- [x] Place from bytes or unlinked paths, and PDF clipboard paste, keep the selected page's live text. Synthetic two-page regressions in `crates/engine/src/cmd/fileio/tests_pdffidelity.rs` cover both sources, page selection and explicit outlines. The single-page importer honors the same text choice as Open; existing PDF fallback rules are unchanged.

## Corpus download integrity

- [x] PngSuite opt-in downloads are pinned, SHA-256 verified and bounded before publication (#1729); no archive extraction. This secures fixture acquisition and does not raise format-fidelity scores.

## EPS Save and source protection

Feature evidence updated 2026-10-26; scores and estimates unchanged.

EPS is offered by Save As, and an opened EPS keeps its own path for ordinary Save. Save writes
one artboard (the first by default, or the named artboard), retaining its page bounds and transparent
margins rather than cropping to the drawn artwork. The whole editable ClerdoVector document remains
embedded in EPS comments, including other artboards. Save bounds the optional TIFF preview to
1024 pixels on its longest side (about 4 MiB of RGBA pixels), avoiding the roughly 16 MiB preview a
large artboard otherwise produces. Export keeps its existing 2048-pixel preview cap. Preview
format choices are unchanged; no public preview-resolution option exists. The EPS PostScript appearance still flattens
transparency; its existing warnings remain. This does not promise native Illustrator editing-data fidelity.

Cropped and selected-only EPS output remains available through Export. Save refuses those options.
An EPS opened from a preview, a partially interpreted program or damaged native editing data keeps
its source-loss warning: writing over that original requires explicit confirmation. File › Save uses
the same engine path and write-failure handling as other document formats.

Synthetic regressions: `tests::save::opened_eps_saves_to_its_own_file_with_editable_data_and_page_margins`,
`eps_save_keeps_all_editing_artboards_while_printing_one_page`,
`eps_save_supports_an_empty_artboard_without_cropping_its_page`,
`partially_read_eps_requires_confirmation_before_save_back`,
`eps_with_damaged_editing_data_requires_confirmation_before_save_back`,
`eps_save_bounds_large_page_preview_without_cropping_vector_or_editing_data`, and the UI regressions
`file_save_writes_opened_eps_back_without_another_save_as_dialog` and
`eps_save_as_keeps_page_options_and_writes_eps_without_export_only_switches`.
