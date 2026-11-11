# Domain and brick plan

Written 2026-11-11 after the five top-level decisions closed (see `design-tree.md`), then revised the same day after a six-agent review: three independent proposals (a Polylith-purist lens, an Atomic-derived lens, a BSSN lens), one reconciliation against the first draft, and two adversarial checks (one hunting for missing or mis-bounded bricks, one for premature ones). Changes the review forced are marked "review". Items only the developer can settle are collected at the end.

Nothing here exists yet. Once the developer confirms, each domain file is created from `docs/spec/TEMPLATE.md` with Purpose and Data drawn from the ledger and the unsettled items as `OPEN:` lines; rule sentences are then drafted one domain at a time for confirmation.

## Domains

One file per product concern, in product language. "Slice" is from decision 5. "Ledger" names the interview questions whose answers feed the file.

| Domain | Owns | Slice | Ledger |
|---|---|---|---|
| `vaults` | A vault as a self-contained directory holding one knowledge base; create, open, close; the registry of vaults outside any vault; many independent vaults; the rule that deleting the rebuildable cache loses nothing the user wrote (review: moved here from the dropped `derived-data`); layout, split, concurrency and backup as OPEN lines | 1 | 50, 56-60, 77, 78 |
| `atoms` | The atom: immutable ID, unique constrained title, created, modified, tags, Markdown body; create, edit, rename, trash, restore, purge after retention (review: trash owned here, not by vaults); orphaned is cited from `links`, not redefined (review); history as the OPEN line that must be settled before the first real vault is written | 1 | 24, 25, 62, 64, 72, 75, 76, 80, 84 |
| `links` | Title links, per-link display text, heading and block anchors, embeds, unresolved links and create-on-activate, rename rewriting, backlinks, the single definition of orphaned (review); OPEN: embeds and links whose target has a file extension before source material exists; OPEN: whether links to or from a trashed atom count; OPEN at slice 2: the graph view's node and edge rules | 1 | 66, 81-83, 92, 94 |
| `tags` | Tag grammar, body tags written into the `tags` property, nesting by `/`, folder paths from import; OPEN: the developer's further tag ideas | 1 | 71, 93 |
| `markdown` | The Obsidian-flavoured construct list and the exact-preservation rule: which constructs exist, block ids, callouts, footnotes, math, tables; Mermaid deferred. How a construct displays belongs to `rendering`; editing behaviour to `editor` (review: the three no longer overlap) | 1 | 92, 95 |
| `plain-text` | The round trip: one atom per `<title>.md` with frontmatter (`id title created modified tags`), `[[title]]` links, dropped unknown properties, ID-conflict prompt, `.canvas` and `.base` skipped, import of Markdown folders, on-demand export, opt-in mirror, never read back, lossless round-trip rule, source and synthetic as optional exports; OPEN: mirror written on every save or on demand | 1 | 3, 62-70, 73, 85-87, 90, 91, 97 |
| `editor` | Live preview as the main view, CodeMirror 6, link and tag autocompletion, find and replace, the no-delay rule | 1 | 88, 104, 116, 122 |
| `search` | One box, Obsidian-style operators, atoms only in slice 1 (review: the source and synthetic filters arrive with those classes), lexical ranking now, blended semantic and fuzzy ranking later; OPEN: which slice brings fuzzy (typo-tolerant) ranking and where its index lives (review: it was settled in round 2 but no brick carried it); latency target | 1, 3 | 6, 38, 39, 88 |
| `workspace` | Tabs, sidebars, split tree, layout persistence, which views exist; mostly OPEN because the shape is a UI prototyping question | 1 | 123, 124 |
| `commands` | Command registry, hotkeys with Obsidian's defaults where actions match, command palette; slice 1 holds only what the live-preview view needs | 1 | 98 |
| `settings` | Which settings are global and which per vault, the resolution order, the settings view. The meaning of each key lives in its own domain (retention in `atoms`, mirror folder in `plain-text`, cloud opt-in in `providers`) (review) | 1 | 60 |
| `appearance` | Light and dark tokens in slice 1; CSS themes and snippets; bundled fonts | 1 | 107, 108 |
| `rendering` | Reading-mode display of each Markdown construct, code highlighting, math; Mermaid OPEN | 2 | 95 |
| `providers` | Provider-agnostic model access, local host and OpenRouter, one provider per role, per-vault cloud opt-in, the single statement that every knowledge operation works with no model (other domains cite it) | 3 | 13, 45-47 |
| `discovery` | Suggested links and tags accepted by the user, opt-in auto-tagging, ephemeral suggestions with remembered dismissals, resurfacing of orphaned and old atoms, gap detection; spaced repetition OPEN | 3, 6 | 4, 15, 28, 32, 40, 41 |
| `source-material` | Internal (stored, content-addressed) and external (URL) sources; machine-extracted text as source with provenance; in-app reading of PDF, web (read-only), audio, video, images; annotations against the source; annotation to atom citing position; no web capture; storage location OPEN; the pdf.js prototype (111) lives here (review: moved from `rendering`) | 4 | 5, 10, 14, 35-37, 55, 58, 74, 111, 125 |
| `synthetic-material` | Indexes from user criteria, wiki per tag, user-designed scheduled reports, conversations; read-only, provenance, stale flag, regenerate on demand, never input to synthesis, deletable, optional export | 5 | 9, 23, 29-34 |
| `mcp` | Created when slice 5 starts: which resources and tools the read-only MCP server exposes and the read-only guarantee (review: replaces the `integration` grab-bag, which held only absences no test could assert) | 5 | 48 |

Dropped by the review: `derived-data` (its content is the brick's business, which `docs/spec/README.md` keeps out of a domain; its one product rule moved to `vaults`) and `integration` (see `mcp`). Not created by decision: `canvas` (parked with molecules), `learning` beyond what `discovery` holds, `sync-and-publish`, `extensions`, `server`.

Glossary candidates (used by more than one domain): vault, knowledge base, atom, title, link, orphaned, tag, source material, synthetic material, provenance, cache, view.

## Bricks

Polylith rules from `AGENTS.md`: components hold logic and never depend on bases; bases are thin entry points; projects are composition roots. Namespace `apperception`. Bricks for slices after 1 are forecast only: they do not appear in `Cargo.toml` or the queue until their slice starts (review).

### Storage versus vault

All three proposers and both checkers agree: merge. In the first draft the two differed only by layer (the SQLite mechanism versus the product object), Polylith splits by capability, there is no second backend because SQLite is settled, and `storage` had no spec domain to cite. Opening a vault and opening its database are one act. One `vault` component owns the directory, the registry, the truth file with its pragmas and migration runner, and the connection and transaction handle the other components write through. Recorded for later: the registry (global, cross-vault) could split out of `vault` if the MCP or server deployables ever need a vault without the registry.

### Components, slice 1 (`components/<name>`, crate `apperception-<name>`)

Listed in dependency order, which is also the queue order.

| Component | Logic | Depends on | Domains |
|---|---|---|---|
| `markdown` | Pure text code, no I/O. In slice 1 the parser yields positions for what the cache and the round trip need: frontmatter, wikilinks with alias, heading and block anchors, embeds, Markdown links, body tags, headings, block ids, plus skip regions for code, comments and math. The remaining constructs are added when a Rust consumer lands (review: narrowed; live preview parses with CodeMirror's own grammar regardless, and the editor package must agree with this construct list). Frontmatter parse and emit live here. No serializer: the body is stored verbatim and rename rewriting is a position-based splice (review) | none | `markdown`, `links`, `tags`, `plain-text` |
| `tag` | Grammar and normalisation, nesting by `/`, folding body tags into the property, folder path to tag path. Tree building waits for a consumer (review) | `markdown` | `tags` |
| `vault` | As above. Concurrency is SQLite WAL plus busy timeout and nothing more until OPEN 77 is answered (review: no lock asserted) | none | `vaults` |
| `settings` | Typed settings in two scopes, global in the registry file and per vault in the truth file, with the resolution order; owns which key lives in which file, not what the keys mean (developer: kept separate) | `vault` | `settings` |
| `atom` | Create with immutable ID, edit body, rename with title constraints and vault-wide uniqueness, trash, restore, purge after a retention period passed in, created, modified, the tags property with body tags folded in, a multi-atom transaction scope that `link` uses for rename. Mutations return the set of touched atom IDs; no event callbacks in slice 1 (review: the callback pattern waits for a consumer, which is slice 3's embedding progress). Events Trashed, Restored, Purged are visible to the cache | `vault`, `markdown`, `tag` | `atoms` |
| `cache` | The rebuildable store, Obsidian's metadata cache as one brick (review: renamed from `derived`, which collides with the product's use of "derived" for synthetic and orphaned status). Per-atom parsed metadata (resolved and unresolved link targets, embeds, tags, headings, blocks), the cross-atom link map, the lexical index. Opens its own store and reconciles per atom by modified timestamp; a full rebuild is a command, not the open path, because open must stay under three seconds on multi-million-word vaults (review). Trashed atoms leave the index, link map and autocompletion candidates and return on restore. Its location (own file or tables in the truth file) is OPEN 59 (review: not pre-settled) | `vault`, `atom`, `markdown`, `tag` | `vaults`, `links`, `tags`, `search` |
| `link` | Title resolution, unresolved links and create-on-activate, backlinks, the orphaned predicate, rename with silent rewriting of link text in every linking atom in one transaction (through `atom`'s transaction scope) | `atom`, `cache`, `markdown` | `links` |
| `search` | Query-language parsing with the operators, lexical ranking and excerpts over the cache; atoms only in slice 1 | `cache` | `search` |
| `plaintext` | Import in two phases: scan and classify (new, update, conflict, skipped `.canvas` and `.base`) with progress callbacks, then apply the accepted set in one transaction after the UI answers conflicts (review). On-demand export of one `<title>.md` per atom with `[[title]]` links. The opt-in mirror. Paths come from the caller, not from settings | `atom`, `tag`, `markdown`, `vault` | `plain-text` |

Known cost of the napi-rs decision (review): the workspace denies `unsafe_code`, and napi-derive's generated wrappers are unsafe, so the desktop base carries a one-line `allow` with its reason in the crate doc.

### Bases and projects, slice 1

| Brick | Role |
|---|---|
| `bases/desktop` | The napi-rs addon surface: one function per operation Electron's main process calls, each a thin call into a component, with types mapped at the boundary. After a mutation the base calls the cache refresh and the mirror write with the touched IDs; it holds no other logic. Depends only on the components it actually calls (review). Its tests must link without Node (napi's noop feature or napi types kept off the tested surface) |
| `projects/desktop` | The Node native addon. Open question on shape: the template's `cargo poly check` requires a project to be a binary with one `main.rs`, and ADR-0004 says so in words. Prototype 119 runs the zero-tooling shape first (the base itself built as `cdylib` plus `rlib`, which passes the check today); if the project shape is preferred, `check.rs` learns a second project shape (a `cdylib` with only `lib.rs`) and the developer starts an ADR superseding ADR-0004's project definition (review) |

### Frontend (`frontend/`, outside Cargo), slice 1

| Package | Language | Role |
|---|---|---|
| `electron` | TypeScript | Electron main process, as thin as Obsidian's: windows, registry handoff through the addon, IPC forwarding, loading the addon. The vault picker is a renderer page, not main-process code (review) |
| `app` | Rust to WebAssembly with Leptos by default (prototype and ADR, 114 and 115) | The renderer: workspace tree, views, command registry and palette, hotkeys, appearance tokens and CSS themes, the vault picker, the settings view, the search box and results, backlinks and tag panes, the import conflict prompt, hosting the editor package. It implements view-level rules of `workspace`, `commands`, `appearance`, `editor`, `settings`, `search`, `links`, `tags`, `plain-text` |
| `editor` | TypeScript | CodeMirror 6 extensions: live-preview decorations, `[[` and `#` autocompletion fed by the app, create-on-activate, find and replace, the no-delay rule. Construct recognition here is CodeMirror's grammar and must agree with the `markdown` component's list |

Tooling gap (review): `tools/poly/src/spec.rs` counts rule citations only under `components/*/tests` and `bases/*/tests`, so rules implemented by frontend packages would show as never pinned. Either `spec.rs` learns a frontend test root or those domains are recorded as frontend-trusted. Settle before the first frontend test cites a rule.

### Forecast for later slices

| Brick | Slice | Logic | Depends on |
|---|---|---|---|
| `components/provider` | 3 | Per-role model traits with Ollama, OpenRouter, OpenAI-compatible implementations; configuration passed in, never read from settings | none |
| `components/embedding` | 3 | Markdown-aware chunking, the embedding pipeline with progress callbacks (the first consumer of an event callback), chunk vectors in the cache, similarity queries and thresholds, re-embedding on model change (review: added; Atomic keeps this apart from search for the same reason) | `cache`, `provider`, `markdown` |
| `components/discovery` | 3, 6 | Suggestions, dismissals, auto-tagging, resurfacing, gap detection | `embedding`, `link`, `tag`, `atom`, `provider` |
| `components/source` | 4 | Content-addressed source store, external URLs, extracted text with provenance, annotations, annotation to atom | `vault`, `atom`, `provider` |
| `components/synthetic` | 5 | Indexes, wiki per tag, reports, conversations, the shared lifecycle; "what is due" for reports, with the clock in the base | `vault`, `atom`, `cache`, `search`, `provider`, `source` |
| `bases/mcp` and `projects/mcp` | 5 | Read-only MCP over stdio; contingent on OPEN 77 allowing a second process on one vault, otherwise MCP becomes functions on the desktop base | `vault`, `atom`, `link`, `tag`, `search`, `synthetic` |
| `bases/server` and `projects/server` | 8 | Nothing designed |  |

### Queue shape for slice 1

`markdown`, `tag`, `vault`, `settings`, `atom`, `cache`, `link`, `search`, `plaintext`, then `bases/desktop` and `projects/desktop`, then `electron`, `editor`, `app`. The `atoms` history question stays an OPEN line; the `atom` step cites only rules that exist and the `plaintext` import step carries `after: atom` (review: a prototype is not a queue step).

## Settled by the developer on 2026-11-11

`settings` is its own component. `tag` stays a brick. The domain is `discovery`. Plural domain names against singular brick names are accepted. The truth schema is one ordered migration list owned by `vault`. The property list is five (`id title created modified tags`); `kind` is dropped because there is one atom kind now and properties can be added later. Fuzzy ranking's slice is unknown and stays an OPEN line in `search`.
