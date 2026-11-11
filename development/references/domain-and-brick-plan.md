# Domain and brick plan

Proposal written 2026-11-11 after the five top-level decisions closed (see `design-tree.md`). Two lists: the spec domains under `docs/spec/`, and the Polylith bricks under `components/`, `bases/`, `projects/`. Nothing here exists yet. Once the developer confirms the domain list, each domain file is created from `docs/spec/TEMPLATE.md` with Purpose and Data drawn from the ledger and the unsettled items as `OPEN:` lines; rule sentences are then drafted one domain at a time for confirmation.

## Domains

Each domain is one file, one product concern, in product language. The "slice" column is from decision 5. "Ledger" names the interview questions whose answers feed the file.

| Domain | Owns | Slice | Ledger |
|---|---|---|---|
| `vaults` | A vault as a self-contained directory holding one knowledge base; create, open, close; the registry of vaults outside any vault; many independent vaults; trash and retention; the layout, split and concurrency questions as OPEN lines | 1 | 50, 56-60, 76-78 |
| `atoms` | The atom: immutable ID, unique constrained title, created, modified, tags, Markdown body; create, edit, rename, trash, restore; orphaned as a derived status; history as an OPEN line | 1 | 24, 25, 62, 64, 72, 75, 80, 81, 84 |
| `links` | Title links, per-link display text, heading and block anchors, embeds, unresolved links and create-on-activate, rename rewriting, backlinks, the definition of connected | 1 | 66, 81-83, 92, 94 |
| `tags` | Tag grammar, body tags folded into the `tags` property, nesting by `/`, the tag tree, folder paths from import | 1 | 71, 93 |
| `markdown` | The Obsidian-flavoured Markdown constructs: which are preserved exactly, which are rendered, block ids, callouts, footnotes, math, tables; Mermaid deferred | 1 | 92, 95 |
| `plain-text` | The round trip: one atom per `<title>.md` with frontmatter (`id title created modified tags`), `[[title]]` links, dropped unknown properties, ID-conflict prompt, import of Markdown folders, on-demand export, opt-in mirror, never read back, lossless round-trip rule, source and synthetic as optional exports | 1 | 3, 62-70, 73, 85-87, 90, 91 |
| `editor` | Live preview as the main view, CodeMirror 6, link and tag autocompletion, find and replace, the no-delay rule | 1 | 88, 104, 116, 122 |
| `search` | One box, blended lexical and later semantic and fuzzy ranking, Obsidian-style operators, atoms by default with explicit filters for source and synthetic, the latency target | 1 (lexical), 3 (semantic) | 6, 38, 39, 88 |
| `derived-data` | Everything rebuildable: link maps, tag counts, headings and blocks, the lexical index, embeddings, suggestions and dismissals; persisted outside the truth; deletable at no loss; the database-split question | 1 | 59, 88 |
| `workspace` | Tabs, sidebars, split tree, layout persistence, which views exist; shape is a UI prototyping question so this file starts mostly OPEN | 1 (minimal) | 123, 124 |
| `commands` | Command registry, hotkeys with Obsidian's defaults where actions match, command palette | 1 | 98 |
| `rendering` | Reading-mode rendering of the Markdown constructs, code highlighting, math; Mermaid and the pdf.js prototype as OPEN lines | 2 | 95, 111 |
| `source-material` | Internal (stored, content-addressed) and external (URL) sources; machine-extracted text as source with provenance; in-app reading of PDF, web (read-only), audio, video, images; annotations against the source; annotation to atom citing position; no web capture system; storage location OPEN | 4 | 5, 10, 14, 35-37, 55, 58, 74, 125 |
| `synthetic-material` | Indexes from user criteria, wiki per tag, user-designed scheduled reports, conversations; read-only, provenance, stale flag, regenerate on demand, never input to synthesis, deletable, optional export | 5 | 9, 23, 29-34 |
| `discovery` | The developer's "emergence": suggested links and tags accepted by the user, opt-in auto-tagging, ephemeral suggestions with remembered dismissals, resurfacing of orphaned and old atoms, gap detection; spaced repetition as an OPEN line | 3, 6 | 4, 15, 28, 32, 40, 41 |
| `providers` | Provider-agnostic model access, local host and OpenRouter, one provider per role (embedding, generation, transcription, vision), per-vault cloud opt-in, every knowledge operation works with no model | 3 | 13, 45-47 |
| `appearance` | Light and dark themes, CSS themes and snippets, the token layering, bundled fonts | 1 (minimal) | 107, 108 |
| `settings` | Per-vault versus global settings and where each lives, the settings view | 1 | 60 |
| `integration` | MCP read-only; Apperception's own URI scheme if any; no `obsidian://`; no quick-capture | 5 | 43, 48, 99 |

Not created now, by decision: `canvas` (parked with molecules, slice 7), `learning` beyond what `discovery` holds (spaced repetition deferred, 40), `sync-and-publish` (out), `extensions` (plugins; only CSS themes are settled, and that lives in `appearance`), `server` (slice 8).

Naming notes. "Index" is used in the product for a synthetic, user-criteria view, so the rebuildable store is called `derived-data`, not index. "Emergence" is the developer's term for the process that `discovery` describes; the file can carry either name. `plain-text` rather than `import-export` because the round trip is one contract, not two features.

Glossary candidates (used by more than one domain): vault, knowledge base, atom, title, link, orphaned, tag, source material, synthetic material, provenance, derived data, view.

## Bricks

Polylith rules from `AGENTS.md`: components hold logic and never depend on bases; bases are thin entry points; projects are composition roots with one `main.rs`. Namespace `apperception`. The frontend lives outside the Cargo workspace.

### Components (`components/<name>`, crate `apperception-<name>`)

| Component | Logic | Domains it implements | Slice |
|---|---|---|---|
| `storage` | The SQLite truth database: schema, migrations, transactions, the storage interface everything else calls | `vaults`, `atoms` (persistence only) | 1 |
| `vault` | Vault directory lifecycle, registry, lock, trash and retention | `vaults` | 1 |
| `atom` | Atom create, edit, rename, trash, restore; title validation; history | `atoms` | 1 |
| `markdown` | Parser producing positions for every construct; serializer that preserves text exactly | `markdown`, `links` (syntax), `tags` (syntax) | 1 |
| `link` | Resolution by title, rewriting on rename, unresolved links, backlinks, orphaned status | `links` | 1 |
| `tag` | Grammar, body-to-property folding, tag tree | `tags` | 1 |
| `plaintext` | Import of Markdown folders, export, mirror, round-trip | `plain-text` | 1 |
| `derived` | Rebuildable index store: link maps, headings, blocks, FTS, later vectors | `derived-data` | 1 |
| `search` | Query language, ranking, filters | `search` | 1 |
| `settings` | Per-vault and global settings | `settings` | 1 |
| `source` | Content-addressed source store, extraction, annotations | `source-material` | 4 |
| `provider` | Model provider traits and implementations (Ollama, OpenRouter, OpenAI-compatible) | `providers` | 3 |
| `synthetic` | Indexes, wiki, reports, conversations, provenance, staleness | `synthetic-material` | 5 |
| `discovery` | Suggestions, dismissals, resurfacing, gap detection | `discovery` | 3, 6 |

Not Rust: `commands`, `workspace`, `editor`, `rendering`, `appearance` are frontend concerns and have no Rust component; their spec domains are pinned by frontend tests.

### Bases (`bases/<name>`)

| Base | Entry point | Slice |
|---|---|---|
| `desktop` | The napi-rs addon surface: the functions Electron's main process calls, each a thin call into components | 1 |
| `server` | HTTP and WebSocket surface for the web version | 8 |
| `mcp` | MCP read-only surface | 5 |

### Projects (`projects/<name>`)

| Project | Builds | Note |
|---|---|---|
| `desktop` | The Node native addon (`cdylib`) composed from `bases/desktop` plus the components it needs | The template's `cargo poly check` expects a project to be a binary crate with one `main.rs`. A napi-rs addon is a `cdylib` with one `lib.rs`. Either the check learns about library projects or the addon is built from the base crate directly. This is the first thing to settle when the brick lands. |
| `server` | The server binary | 8 |

### Frontend (`frontend/`, outside Cargo)

| Package | Language | Role |
|---|---|---|
| `electron` | TypeScript | Electron main process, kept as thin as Obsidian's: windows, vault registry handoff, protocol handler, loads the addon |
| `app` | Rust to WebAssembly with Leptos by default (prototype and ADR, 114 and 115) | The renderer: workspace, views, commands, appearance |
| `editor` | TypeScript | CodeMirror 6 extensions: live preview, autocompletion, syntax |

### Queue shape for slice 1

In dependency order: `storage`, `vault`, `atom`, `markdown`, `tag`, `link`, `derived`, `search`, `plaintext`, `settings`, then `bases/desktop` and `projects/desktop`, then the three frontend packages. Each queue step cites the rules of its domain once those rules exist.
