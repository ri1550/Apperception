# Atomic v1.45.0: how it is built

Notes on `Atomic/atomic` (MIT, github.com/kenforthewin/atomic), read 2026-11-08. Atomic is the primary code reference for Apperception because it is open source and already Rust. These notes say what it does, how it is shaped, and where it differs from Obsidian, so the decision map can say which parts to borrow.

## What Atomic is

A personal knowledge base that treats notes ("atoms") as input to an AI pipeline. Each atom is Markdown. On save, a background pipeline chunks it, embeds the chunks with a configured provider, extracts hierarchical tags with an LLM, and builds similarity edges to other atoms. On top of that index it offers semantic search, a force-directed canvas, LLM-written wiki articles with citations, scheduled "reports", RSS ingestion, a web clipper, an MCP server, and an agentic chat. It runs as a Tauri desktop app, a headless server, or both.

The important difference from Obsidian is where the truth lives. In Obsidian the Markdown files are the database. In Atomic a SQLite database is the database and Markdown is the content type of one column. There is an Obsidian vault importer (`crates/atomic-core/src/import/obsidian.rs`) and an Obsidian plugin that pushes notes to an Atomic server, but Atomic does not watch a folder of files as its source of truth.

## Shape of the code

Cargo workspace with seven members:

| Crate | Role |
|---|---|
| `crates/atomic-core` | All business logic. No framework dependencies. `AtomicCore` is a `Clone` handle over `Arc<Database>` with roughly 230 public methods. |
| `crates/atomic-server` | actix-web REST (about 78 routes) plus WebSocket plus Streamable HTTP MCP endpoint. Also the CLI (`serve`, `token`, `migrate`). |
| `crates/mcp-bridge` | stdio to HTTP bridge so Claude Desktop can talk to the local server. |
| `src-tauri` | Desktop shell. Spawns `atomic-server` as a sidecar and exposes one IPC command that returns the sidecar's URL and token. |
| `crates/atomic-cloud` | Multi-tenant hosting. |
| `crates/atomic-bench`, `crates/atomic-test-support` | Benchmarks and test fixtures. |

Frontend is React 18, TypeScript, Vite, Tailwind v4, Zustand, CodeMirror 6 (`@codemirror/lang-markdown` plus their own `@atomic-editor/editor` package), react-markdown for rendering, d3-force and sigma for the canvas. Mobile is Capacitor around the same React app. The `plugins/obsidian-plugin` folder is a conventional Obsidian community plugin.

Three design choices worth copying regardless of product direction:

1. Core plus thin wrappers. Every transport (Tauri, HTTP, MCP) calls the same `AtomicCore` methods and does nothing else. The desktop app does not even use Tauri IPC for data; it talks HTTP to its own sidecar. That is why one React codebase serves desktop, web, and mobile unchanged.
2. Callback-based events in core. Long operations take an `Fn(Event)` closure. The server turns those into a `tokio::sync::broadcast` channel and WebSocket frames; Tauri would turn them into `emit`. Core never knows.
3. Storage behind traits. `storage/traits.rs` is implemented twice, `storage/sqlite/` and `storage/postgres/`, with the same per-feature file split (atoms, tags, chunks, chat, wiki, feeds, reports, settings, search, task_runs). Migrations are numbered SQL files for Postgres and inline `CREATE TABLE IF NOT EXISTS` blocks in `db.rs` for SQLite.

Things to be careful about if borrowing:

- `AtomicCore` is one struct with everything on it. Their own AGENTS.md documents a trap where `get_settings` silently routes to the registry database instead of the per-database settings. A Polylith split into components would avoid a god object, but the facade idea itself (one handle, many bricks behind it) is sound.
- The frontend "transport" layer maps command names to HTTP specs by hand in `src/lib/transport/command-map.ts`. It works but is a second copy of the API that can drift.
- Much of the codebase is AI-pipeline plumbing (providers, chunking, embedding, tagging, wiki synthesis, reports, scheduler). If Apperception is first a note-taking app, most of this is not needed on day one.

## Data model

SQLite tables in a data database (`db.rs`): `atoms` (id, content, source_url, created_at, updated_at, embedding_status, tagging_status; later migrations add title, kind, published_at), `tags` (id, name case-insensitive unique, parent_id), `atom_tags`, `atom_chunks` (chunk_index, content, embedding blob) with `vec_chunks` as a sqlite-vec virtual table, `atom_links` (source atom, target atom nullable, raw_target, label, target_kind, status, offsets), `semantic_edges`, `atom_positions`, `atom_clusters`, `wiki_articles` with versions, citations, links, proposals, `conversations`, `chat_messages`, `chat_tool_calls`, `chat_citations`, `feeds`, `feed_items`, `reports`, `report_findings`, `task_runs`, `atom_pipeline_jobs`, `settings`. FTS5 virtual tables exist for atoms, chunks, chat messages, wiki articles.

A separate `registry.db` holds `databases`, `api_tokens`, `oauth_clients`, `oauth_codes`, and global `settings`.

`atom_links` is the closest thing to Obsidian's resolved and unresolved link maps: it stores `[[wikilinks]]` found in content with a nullable target, so unresolved links survive. Tags are a real tree with parent ids, while Obsidian derives the hierarchy from `/` in the tag string. Atomic has no folders, no attachments as first-class files, no frontmatter properties with types, no workspace layout persistence, no block references, no headings index.

## Build and release

Tauri v2 with a size-optimised release profile (`opt-level = "z"`, LTO, strip, `panic = "abort"`) and a separate `server` profile for Docker builds. GitHub releases ship desktop binaries; GHCR ships `atomic-server` and `atomic-web` images.

## What Apperception can take from Atomic

Direct reuse candidates (MIT, attribution required):

- The Rust Obsidian vault importer: frontmatter parsing with `yaml_rust2`, tag extraction formats, default excludes `.obsidian .trash .git node_modules`, folder-to-tag mapping.
- Markdown-aware chunking (`chunking.rs`) if semantic search is ever wanted.
- The provider trait design (`providers/traits.rs`) if any AI feature is wanted.
- The sidecar pattern in `src-tauri` if Apperception ships a desktop shell over a local server.

Patterns to adopt in Apperception's own code:

- One core handle, transports as thin adapters. In Polylith terms, components hold the logic and each base (desktop, CLI, server) is a thin adapter.
- Events as closures in components, adapted to channels at the base.
- Storage traits with the implementation chosen by the project.

Things Obsidian has that Atomic does not, which Apperception will have to design itself: the file-watching vault, the metadata cache, the workspace tree, the command registry, properties with types, the plugin API, and the theme token system.
