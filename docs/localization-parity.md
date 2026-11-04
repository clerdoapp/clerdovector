# Localization parity

> **Last reviewed:** 2026-10-10 · **Last updated:** 2026-10-31 · **Change:** minor (Brazilian Portuguese: 154 rows for the strings the sibling catalogs translate — smart-filter and print presets, dimension lines, swatches, canvas messages; previously minor: Bosnian (`bs`): complete catalog; previously minor: Vietnamese text: stacked diacritics tested and attach in every form, #4491; previously: Catalan (`ca`) catalog added, complete under the tests)

Per-language status of ClerdoVector's interface. How the catalogs work and how to add a language:
[`development.md` › Localisation](development.md#localisation). Part of
[target-app-parity.md](target-app-parity.md).

**How it is measured:** rows with a translation in each `crates/ui-egui/src/i18n/<code>.tsv`, counted on
origin/main `01e165af`, except the rows that give a count date: those were counted on the `main` commit named in the row (the other dated rows on `main` at `cb0a7d9`, 2026-10-21; the Portuguese row on `main` at `d26dd933a`, 2026-10-31).
The reference is the French catalog, which was complete when counted: 3,416 interface
strings (menus, command labels, `tl!` literals: panels, dialogs, tooltips, Preferences) plus 604 `@msg` status and
error messages, 4,020 in all. A language's % is its rows over 4,020 (capped at 100% per part). A missing
translation never fails a test or a build: the interface shows the English text until a translator adds the row.
`cargo xtask ci` prints the untranslated report (`cargo xtask i18n-report` alone), which lists per language the
strings missing from each catalog with the `complete_menus` flag and the messages missing from each language in
the `COMPLETE_MESSAGES` list. Every non-English language marked "full" has the flag, and all of them except
Bulgarian, Danish, Indonesian and Dutch are in the list. No language has help content: there is no translated (or
English) in-app help, and the docs are English.

**Native review:** every catalog was written clean-room from the English meaning, most of it by agents
(`bflatastic`), with community contributors adding or correcting (German by `dwetscher`, Japanese and Traditional
Chinese by `imachus`). Only Georgian is recorded as reviewed by a native speaker; the other catalog headers ask for one.

## The twelve key languages

| Language | Code | UI strings translated | Dialogs / tooltips / messages | Script support | Native review | Status | To `full` (h) |
|---|---|---|---|---|---|---|---|
| English | `en` | source, 100% | all | Latin | — | full | — |
| Simplified Chinese (Mandarin) | `zh-hans` | 5,102 rows, 791 of them `@msg` (counted 2026-10-21): menus, `tl!` literals and messages complete when counted | all | CJK UI font from craft-fonts (BIZ UDPGothic) when embedded, else a system font; IME with marked text at the caret; vertical type partial ([type-parity.md](type-parity.md)) | no | full | — (review) |
| Spanish | `es` | 4,009 / 4,020 (99.7%) | all (604 messages) | Latin | no | full | — (review) |
| Hindi | `hi` | 0 | none | Devanagari: the document's text shapes (HarfRust), with tests of conjuncts, reph, pre-base matras, nukta forms and caret stops that run when an OpenType Devanagari font is installed, but the egui interface does no complex shaping | no | none | 20–30 |
| Arabic | `ar` | 0 | none | document text: bidi layout and Paragraph Direction; interface: no right-to-left layout, no Arabic shaping in egui | no | none | 25–40 |
| French | `fr` | 4,020 / 4,020 (100%) | all | Latin | no | full | — (review) |
| Portuguese | `pt-br` | 6,106 catalog entries (counted 2026-10-31 on `main` at `d26dd933a`): menus, `tl!` literals and messages complete when counted; one/other plurals | all | Latin | contributor is a Brazilian Portuguese speaker; further native review welcome | full | — (review) |
| Indonesian | `id` | 5,279 rows, 750 of them `@msg` (counted 2026-10-21): menus and `tl!` literals complete when counted; its messages are partial; one plural form | dialogs and tooltips all; 750 message rows (French has 796) | Latin | no | full | — (review) |
| Japanese | `ja` | 4,004 / 4,020 (99.6%) | all | CJK UI font as above; Japanese IME; vertical type partial; Japanese crop marks and composition settings | no | full | — (review) |
| German | `de` | 3,952 / 4,020 (98%; menus, literals and messages complete when counted) | all (602 of 604 message rows; landed 2026-10-10) | Latin | contributor-written, not reviewed | full | — (review) |
| Korean | `ko` | 4,870 rows (counted 2026-10-21): menus, `tl!` literals and messages complete when counted; one plural form | all | Hangul from an installed system font on the desktop (Apple SD Gothic Neo, Malgun Gothic, Noto Sans CJK KR…); the web build has no Korean UI font yet (craft-fonts); IME path exists | no (AI-drafted) | full | — (review; web font) |
| Vietnamese | `vi` | 4,886 rows (counted 2026-10-21): menus, `tl!` literals and messages complete when counted; one plural form; 4,046 plain + 18 `@plural` + 28 contextual + 794 `@msg` | all | Latin with Vietnamese diacritics (stacked marks attach in precomposed, decomposed and keyboard forms, #4491; glyph tests pass) | contributor is a Vietnamese speaker; AI-assisted draft, further native review welcome | full | — (review) |

**12-language score: ~82%** (equal weights: 100 + 85 + 100 + 0 + 0 + 100 + 100 + 100 + 100 + 98 + 100 + 100, over 12).
**To `full` for all twelve: ~50–85 h**, most of it Arabic (right-to-left interface) and Hindi (interface
shaping). Each new Latin-script catalog is ~4,000 rows; the existing ones were written in a few agent hours each
plus review. Native-speaker review is human work and isn't in the hours.

## Other shipped languages

| Language | Code | Translated | Status |
|---|---|---|---|
| Traditional Chinese (Taiwan) | `zh-hant` | 4,892 rows, 790 of them `@msg` (counted 2026-10-21): menus, `tl!` literals and messages complete when counted (80% overall) | partial |
| Italian | `it` | 4,011 / 4,020 | full |
| Dutch | `nl` | 5,317 rows, 777 of them `@msg` (counted 2026-10-21): menus and `tl!` literals complete when counted; its messages are partial; one/other plurals | full |
| Danish | `da` | 5,279 rows, 750 of them `@msg` (counted 2026-10-21): menus and `tl!` literals complete when counted; its messages are partial; one/other plurals | full |
| Russian | `ru` | 4,009 / 4,020 | full |
| Ukrainian | `uk` | 4,009 / 4,020 (one/few/many plurals) | full |
| Hungarian | `hu` | 5,253 rows (counted 2026-10-21): menus, `tl!` literals and messages complete when counted; one plural form | full |
| Polish | `pl` | 5,130 rows (counted 2026-10-21): menus, `tl!` literals and messages complete when counted; one/few/many plurals | full; native review pending |
| Finnish | `fi` | 5,318 rows (counted 2026-10-21): menus, `tl!` literals and messages complete when counted; one/other plurals, 797 `@msg` rows | full; AI-assisted, native proofreading pending |
| Czech | `cs` | 5,240 catalog entries, including 795 `@msg` rows (counted 2026-10-21); menus, literals and messages complete when counted | catalog complete; render-site gaps reduced; native review pending |
| Bosnian | `bs` | 5,841 rows (counted 2026-10-30): menus, `tl!` literals and messages complete when counted; one/few/other plurals | full; AI-assisted, native review pending |
| Swedish | `sv` | 5,251 rows, 795 of them `@msg` (counted 2026-10-21): menus, `tl!` literals and messages complete when counted; one/other plurals | full; native review welcome |
| Bulgarian | `bg` | 5,245 rows, 763 of them `@msg` (counted 2026-10-21): menus and `tl!` literals complete when counted; its messages are partial | full; AI-drafted, native review pending |
| Greek | `el` | 5,251 rows, 795 of them `@msg` (counted 2026-10-21): menus, `tl!` literals and messages complete when counted | full; native review welcome |
| Georgian | `ka` | 5,200 rows (counted 2026-10-21): menus, panels, dialogs, plurals (one count-neutral form) and messages, tested complete; reviewed by a native speaker | full |
| Turkish | `tr` | 5,558 rows, 880 of them `@msg` (counted 2026-10-28); 6 menu strings and the default swatch names still fall back to English | full |
| Indonesian | `id` | 5,531 rows, 788 of them `@msg` (counted 2026-10-28): menus and `tl!` literals complete when counted; its messages are partial; one plural form | full |
| Catalan | `ca` | 5,626 rows, 881 of them `@msg` (counted 2026-10-28): menus, `tl!` literals and messages complete when counted; one/other plurals; every `ca-*` locale resolves here | full; AI-drafted, native review welcome |

Twenty-five non-English catalogs in all. Illustrator, as its public documentation lists, ships around two dozen
interface languages (including Polish, Dutch, Turkish, the Nordic languages, Hungarian, and Middle East
editions with Arabic and Hebrew); it doesn't ship Hindi, Indonesian or Vietnamese.

## Czech render-site follow-up

The desktop and web file pickers look up filter labels at assignment time; format names and
extensions stay unchanged. Diagnostics retain numeric values and units. New Document's suggested
preset name uses `Preset {number}`. Contributor tooltips use named placeholders and Czech plurals;
handles and chosen names remain unchanged. Gradient descriptions reuse `{kind} Gradient` with a
contextual radial adjective. PDF/Print mark styles use `mark-style`, so `Japanese` displays as
`Japonské` while the language name stays `Japonština`; their Summary uses the same context.

The search placeholder, Home screen, toolbar/panel tooltips and Preferences already use lookups.
No new `tl!("literal")` keys were needed: only `cs.tsv` adds 19 AI-draft rows and revises the gradient
template. Every other complete catalog (`complete_menus`) has the same rows and a `Roman` row for the
other mark style; the untranslated report lists a catalog that lacks one of them.
Native review is still needed, and catalog coverage does not prove that every variable in the UI is
translated.

## Not done in any language

Locale-aware number and date formats, a right-to-left interface, translated help, and a Traditional Chinese UI
font for the web build.
