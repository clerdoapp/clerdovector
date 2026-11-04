# ClerdoVector Roadmap

- PDF.R1: editable PDF import uses the shared reader, scans page content in windows, and rejects known incomplete input before returning a document.
- PDF.W1: PDF export and print composition use the shared optional writer for configuration, metadata, font preparation and guarded finalization; drawing and output policy remain app-owned.

- UI.5: Shared docking adds tab reordering, docking into other groups, dock splits and resizable floating panel groups. Panel positions and return locations persist with workspaces; the native icon rail and Tools panel remain available.

- UI.4: Dock and floating-panel headers use shared accessible tabs with bounded overflow. Widget-state theme application uses `craft-ui`; all four palettes, font stacks and preferences remain app-owned.

**Stage: alpha** · next: beta, ~13 points (ready for real work ~62% → ~75%, and reliable `.ai` exchange) and ~170–270 h away

> **Last reviewed:** 2026-10-10 · **Last updated:** 2026-11-01 · **Change:** minor (Progress log row for the Color Effects add-on's Find Nearest Swatch; headline numbers unchanged) · **Target:** Adobe Illustrator 2026 (30.x)

ClerdoVector is a clean-room, open-source, pure-Rust reimplementation of the Adobe Illustrator workflow. It runs
on macOS, Windows, Linux, FreeBSD and the web (WASM), and agents can drive all of it over MCP, the CLI and a JSON
control channel. This page is the summary; the evidence is in [`docs/target-app-parity.md`](docs/target-app-parity.md)
and the work list in [`docs/gaps.md`](docs/gaps.md).

**Why alpha:** the core workflows exist end to end (drawing and path editing, Pathfinder, paint and appearance,
type, SVG/PDF/EPS, print, export) and feature depth is ~75%, but a professional can't yet switch: Illustrator's
own `.ai` files open with a wrong layer structure in real cases and can't be saved back with Illustrator's
editing data, the interaction details have never been checked side by side, and some machines fail to start the
app. Beta needs those closed (the beta list in [gaps.md](docs/gaps.md)). It passes the core-workflow
gate: all six core workflows (path drawing and editing, paint and style, type, layers/artboards/export, print
production, file exchange) work end to end on macOS with save and reopen, and only the last is partial, without
blocking it ([Alpha gate](docs/roadmap.md#alpha-gate)).

## Headline numbers

| | Value | Kind |
|---|---|---|
| **Feature breadth** (Illustrator's menu items, tools, effects, formats exist) | **~88%** | measured in part: menu items 348/375 wired (92.8%), tools 89/92, Illustrator effects 46/50, Photoshop effects 57/57, 3D 4/4 (initial types, no materials) |
| **Feature depth** (weighted by use, scored by behaviour) | **~75%** | estimated |
| **To beta** | **~170–270 h** one agent · ~50–80 h with 4–6 agents | estimated |
| **To full parity** | **~360–590 h** one agent · ~95–165 h with 4–6 agents | estimated |

**Readiness by audience** (all estimated; method in [target-app-parity.md](docs/target-app-parity.md#ready-for-real-work)):

| Audience | Ready % | Opus 5.5 agent hours to ~95% (one agent) | With 4–6 agents | Work that dominates |
|---|---:|---:|---:|---|
| **Full Illustrator** (ready for real work; decides the stage) | **~62%** (57–67%) | **~340–560 h** | ~95–160 h | the mainstream work below, plus 3D (50–85 h), raster effects (23–39 h), advanced type (20–28 h), generative AI (30–60 h), localization (50–85 h) and ecosystem |
| **Mainstream illustrator** | **~65%** (60–70%) | **~230–380 h** | ~65–110 h | the interaction-fidelity pass (60–90 h), hardening and QA (30–50 h), stability (20–35 h), performance budgets (15–25 h), `.ai` exchange (10–20 h), the remaining depth of the everyday areas |
| **Essentials user** | **~75%** (70–80%) | **~60–105 h** | ~25–45 h | launch and stability on Windows (20–35 h), in-app help and onboarding (8–15 h), basic QA (10–15 h), Shape Builder regions (4–8 h), UI polish |

Hours are Opus 5.5 agent wall-clock hours, calibrated from this repo's merged-PR timestamps (e.g. 0.7–1.2 h per
Photoshop-style filter, ~0.8 h per medium feature); see [calibration](docs/target-app-parity.md#calibration).
The installed Illustrator was **not** inspected: the clean-room rule in [`AGENTS.md`](AGENTS.md) forbids it, so
everything is measured from our code against Illustrator's public documentation.

## By dimension

| Dimension | % | One agent (h) | Doc |
|---|---:|---:|---|
| Features (depth) | 75% | 220–360 | [target-app-parity.md](docs/target-app-parity.md) |
| UI/UX fidelity | 55% | 70–110 | [ui-parity.md](docs/ui-parity.md) |
| File formats | 75% | 15–30 | [file-format-parity.md](docs/file-format-parity.md) |
| Hardware (GPU, pen, touch, displays) | 35% | 25–45 | [hardware-parity.md](docs/hardware-parity.md) |
| Localization (12 key languages) | 74% (measured) | 50–85 | [localization-parity.md](docs/localization-parity.md) |
| Performance | ~60% | 15–25 | [gaps.md › G15](docs/gaps.md#g15-performance-budgets-and-a-gpu-renderer) |
| Stability | ~60% | 30–50 | [gaps.md › G10](docs/gaps.md#g10-launch-and-stability-on-real-machines) |
| Platforms | 80% | 15–25 | [gaps.md › G8](docs/gaps.md#g8-hardening-at-scale) |
| Ecosystem and plug-ins | 15% | 30–60 | [gaps.md › G17](docs/gaps.md#g17-ecosystem-third-party-plug-ins) |
| AI features (generative) | 0% | 30–60 + owner decision | [gaps.md › G18](docs/gaps.md#g18-generative-ai-features) |
| Agent automation | beyond Illustrator | — | [mcp.md](docs/mcp.md) |

The dimension hours overlap (feature rows hold some UI, format and AI work), so they don't add up to the total.

## Features

| Area | % | One agent (h) |
|---|---:|---:|
| Selection, transform & align | 85% | 4–6 |
| Drawing tools | 85% | 8–13 |
| Path operations, Pathfinder, Shape Builder, Live Paint | 80% | 4–8 |
| Colour, swatches, gradients, patterns, mesh, recolor | 92% | 4–7 |
| Strokes, brushes, width profiles | 88% | 8–15 |
| Appearance, transparency, graphic styles, masks | 97% | 1–2 |
| Live vector effects | 85% | 4–8 |
| Raster effects (Effect Gallery) — [effects-parity.md](docs/effects-parity.md) | 85% | 1–2 |
| 3D and Materials | ~25% | 30–55 |
| Type core — [type-parity.md](docs/type-parity.md) | 80% | 14–19 |
| Type advanced (CJK, spelling, Touch Type) | 50% | 20–28 |
| Symbols, blends, envelopes, Repeat, perspective | 87% | 5–9 |
| Image Trace, graphs, image tools | 75% | 5–8 |
| Layers, artboards, document setup | 78% | 5–8 |
| View & navigation | 70% | 9–14 |
| Guides, grids, smart guides, snapping | 80% | 4–7 |
| File formats — [file-format-parity.md](docs/file-format-parity.md) | 75% | 10–20 |
| Export for Screens, Asset Export, slices | 85% | 2–4 |
| Print, colour management, separations | 80% | 2–4 |
| Automation (Actions, Variables, scripting) | 60% | 8–14 |
| UI chrome (panels, Properties, preferences) | 80% | 13–20 |
| Libraries, Links, Package | 82% | 2–3 |
| Generative AI | 0% | 30–60 |

Weights, evidence and the missing items per area: [target-app-parity.md](docs/target-app-parity.md#feature-areas).

## Languages

| Language | Code | Status | Translated |
|---|---|---|---:|
| English | `en` | full | 100% |
| Simplified Chinese | `zh-hans` | partial (messages in English) | 85% |
| Spanish | `es` | full | 100% |
| Hindi | `hi` | none | 0% |
| Arabic | `ar` | none (no right-to-left interface) | 0% |
| French | `fr` | full | 100% |
| Portuguese (Brazil) | `pt-br` | full | 100% |
| Indonesian | `id` | none | 0% |
| Japanese | `ja` | full | 100% |
| German | `de` | full | 98% |
| Korean | `ko` | full | 100% |
| Bosnian | `bs` | full (AI-assisted, native review pending) | 100% |
| Vietnamese | `vi` | full | 100% |

Also shipped: Traditional Chinese (partial, 80%), Bulgarian, Finnish, Greek, Hungarian, Italian, Polish, Russian, Swedish, Ukrainian and Georgian (full), Catalan (full; AI-drafted, native review welcome), Czech (catalog complete; render-site gaps reduced; native review pending).
The Georgian catalog has had a native speaker's review; no other catalog has. Details: [localization-parity.md](docs/localization-parity.md).

## Upcoming

Ranked; detail in [docs/roadmap.md](docs/roadmap.md) and [docs/gaps.md](docs/gaps.md).

1. **Interaction-fidelity pass** (G1), tool by tool from [ui-parity.md](docs/ui-parity.md): 60–90 h.
2. **`.ai` files from real users** (G9): layer structure, guides, speed: 10–20 h.
3. **Launch and stability on real machines** (G10): 20–35 h.
4. **Core-tool correctness** (G11: Shape Builder regions, Asset Export borders): 8–15 h.
5. **Raster effects** (G2) alongside: the Effect Gallery's filter thumbnails and in-dialog preview area (the gallery and every Photoshop-style filter are in), 1–2 h.
6. **Then:** type core and advanced type (G4), the Illustrator 2026 additions (G13), 3D (G3).
