# Apperception: what has to be decided before the spec can be written

Written 2026-11-08 after reading Obsidian 1.15.0 (see `obsidian-analysis.md`) and Atomic v1.45.0 (see `atomic-analysis.md`). This is a proposal, not the spec. Per `docs/spec/README.md`, a domain file exists only when the developer asks for it, and every rule comes from the developer. So this document lists the domains I think the product needs and, for each, the questions that have to be answered before a single rule can be written. Once you confirm a domain, I copy `docs/spec/TEMPLATE.md` to `docs/spec/<domain>.md`, fill Purpose and Data from this document, and move its questions in as `OPEN:` lines. Nothing becomes a numbered rule until you state it.

I have put my own recommendation after questions where I have one. They are opinions, not decisions.

## Decisions that shape everything else

These five are not a domain. They are the answers every domain depends on, so they come first. Progress is tracked in `design-tree.md`: decision 4 was settled on 2026-11-09 (see its closing statement there) and decision 1 is being worked; the text below is the original framing.

1. What is the source of truth? Obsidian: a folder of Markdown files, with a rebuildable cache. Atomic: a SQLite database, Markdown as content. The choice decides whether a user can point Apperception at an existing Obsidian vault, whether Dropbox-style sync works for free, and whether every feature has to survive the files changing under it. Recommendation: files are the truth, exactly as Obsidian, with one derived index that can be deleted at any time. This is the one property that made Obsidian trustworthy and it costs nothing to keep if decided now and everything to retrofit.

2. How compatible with Obsidian's on-disk format? Three levels: (a) same Markdown flavour only; (b) also read and write `.obsidian/` config so both apps can be open on one vault; (c) also run Obsidian community plugins. Recommendation: (a) is mandatory, (b) is cheap for `app.json`, `appearance.json`, `types.json`, `hotkeys.json` and worth doing, (c) is a JavaScript runtime inside a Rust app and should be a separate, later decision.

3. What is the UI technology? Atomic is Tauri with a React web view. Obsidian is Electron. A Rust-native toolkit (egui, iced, gpui, slint, Dioxus native) is the third path. This decides whether Obsidian's CSS token system can be reused in spirit (web view) or has to be translated (native). It also decides whether one codebase covers mobile. Recommendation: decide this before any UI domain is written; the workspace and editor domains read differently under each.

4. What is Apperception's own reason to exist? Obsidian already exists and is free. Atomic's answer was "AI pipeline over notes". Candidates I can see from the two references: open source where Obsidian is not; native and fast where Obsidian is Electron; first-class structured data (Bases-like) rather than a plugin; a real plugin sandbox; local AI. The spec's Purpose sections cannot be written without this.

5. What ships first? A product the size of Obsidian is dozens of domains. The queue needs a first deployable project. Recommendation: vault plus editor plus links plus search, nothing else, so that the first project is "open an Obsidian vault, read and edit notes, follow links, search". Everything in the feature inventory below is then an ordered backlog.

## Proposed domains

Grouped so each domain could become one or a few Polylith components. Names are kebab-case filename stems. Under each: what it owns, then the open questions.

### vault

Owns: a folder on disk as a collection of files and folders, watching it for external changes, create/read/modify/rename/move/delete/trash, attachments, the per-vault config folder, and the global registry of known vaults.

- OPEN: Is a vault strictly a folder on local disk (Obsidian), or can it also be a database or a remote (Atomic allows N databases under one server)?
- OPEN: Which file types are notes and which are attachments? Obsidian's list is `.md`, `.base`, `.canvas`, images, audio, video, PDF. Do we start with `.md` only?
- OPEN: Where does per-vault configuration live? Options: `.obsidian/` (compatible), `.apperception/` (independent), both (read theirs, write ours).
- OPEN: Deletion behaviour. Obsidian offers system trash, a `.trash/` folder in the vault, or permanent. Do we offer all three, and which is default?
- OPEN: Attachment placement policy. Obsidian offers vault root, fixed folder, same folder as note, subfolder next to note. Which do we support at first?
- OPEN: Do we need `Vault.process()` semantics (atomic read-modify-write that refuses to clobber an external edit) from day one? Obsidian added it after data-loss bugs. Recommendation: yes.
- OPEN: How are external edits detected: OS file watcher, polling, or both with a fallback for network drives?
- OPEN: Are symbolic links followed? Obsidian documents pitfalls here.
- OPEN: Multiple vaults open at once in one process, or one vault per process?
- OPEN: Is there a global vault registry (Obsidian's `obsidian.json` with id, path, last-opened), and does it live in XDG config?
- OPEN: Does a "sandbox" or demo vault ship with the app?
- OPEN: Can a single Markdown file outside any vault be opened (Obsidian 1.14 added this)?

### notes

Owns: what a note is, its Markdown flavour, how it is parsed into blocks, headings, list items, frontmatter, tags, links, embeds, footnotes, block ids.

- OPEN: Which Markdown flavour exactly? Obsidian is CommonMark plus GFM plus LaTeX plus wikilinks, embeds, `^block-id`, `[^footnote]`, `%%comment%%`, `==highlight==`, callouts, task lists. Do we match all of it, a subset, or a superset?
- OPEN: Is Markdown rendered inside raw HTML blocks? Obsidian says no on purpose.
- OPEN: Do we parse with an existing Rust CommonMark parser (pulldown-cmark, comrak, markdown-rs) and extend it, or write a parser that produces positions for every construct? The metadata cache needs positions.
- OPEN: What is a block? Obsidian: paragraph, heading, list item, blockquote, callout, code, table, footnote definition, yaml, thematic break, html. Block ids attach to these.
- OPEN: Is the note title the filename (Obsidian) or a frontmatter field (Atomic) or an H1?
- OPEN: Does a note have a stable identity other than its path? Obsidian has none; renames rewrite links. Atomic has a UUID. This matters for links, sync, and history.

### links

Owns: internal link syntax, resolution of a link text to a file, heading and block anchors, aliases, embeds, backlinks, unresolved links, link rewriting on rename, unlinked mentions.

- OPEN: Do we support both `[[wikilinks]]` and `[text](path.md)`, and which does the app generate by default? Obsidian defaults to wikilinks with a setting for Markdown links.
- OPEN: Resolution rule. Obsidian resolves by shortest path that matches the basename, vault-wide, with a per-vault "new link format" (shortest, relative, absolute). Do we copy this exactly for compatibility?
- OPEN: What happens when a link target does not exist? Obsidian creates the note on click and counts it as an unresolved link in the graph.
- OPEN: Are heading anchors (`#Heading`) and block anchors (`#^id`) in scope for the first release?
- OPEN: Is link rewriting on rename automatic, prompted, or off? Obsidian's `alwaysUpdateLinks` setting.
- OPEN: Are aliases (frontmatter `aliases` list) resolved as link targets and shown in autocomplete?
- OPEN: Are embeds (`![[...]]`) of notes, headings, blocks, images, PDFs, and search results each in scope, and in what order?
- OPEN: Unlinked mentions (plain text matching a note name) are expensive. In scope?

### metadata-cache

Owns: the derived index over every note (links, embeds, tags, headings, sections, list items, blocks, frontmatter), the resolved and unresolved link maps, incremental updates on file change, persistence between runs, and full rebuild.

- OPEN: Is the index persisted between runs (Obsidian: IndexedDB, rebuilt if missing) or rebuilt on every start? Persistence is what makes a large vault open fast.
- OPEN: Where does the persisted index live: inside the vault's config folder, in XDG cache, or in a SQLite file? Recommendation: outside the vault so file sync never ships it.
- OPEN: Is the index allowed to be the truth for anything, or always derivable from files? See decision 1.
- OPEN: Which consumers does it serve at first: link resolution, backlinks, search, outline, tag pane, graph, bases? Each one adds fields.
- OPEN: Does it index non-Markdown files (attachments appear in Obsidian's graph)?
- OPEN: How is "excluded files" (Obsidian's `userIgnoreFilters`) applied: hidden from search and graph but still indexed, or not indexed at all?

### properties

Owns: YAML frontmatter as typed key-value data, the property type registry (one type per property name across the vault), built-in properties (`tags`, `aliases`, `cssclasses`), display modes.

- OPEN: Which property types? Obsidian: text, list, number, checkbox, date, datetime, tags. Do we add links, URLs, or nested objects? Obsidian explicitly refuses nested YAML and Markdown in values.
- OPEN: Is the type registry per vault (Obsidian's `types.json`) and compatible with Obsidian's file?
- OPEN: How is frontmatter written back: preserve the user's YAML formatting and key order, or normalise?
- OPEN: Are internal links allowed in property values, and must they be quoted as in Obsidian?
- OPEN: Does the editor show properties as a form (Obsidian's default), as source, or hidden, and is this per note or per vault?

### tags

Owns: tag syntax, case folding, nesting by `/`, the tag index with counts, the tag pane.

- OPEN: Do we copy Obsidian's grammar exactly (letters, digits, `_`, `-`, `/`, Unicode, at least one non-digit, case-insensitive, display case is first seen)?
- OPEN: Is the hierarchy derived from `/` in the string (Obsidian) or stored as a tree with parent ids (Atomic)? These are incompatible models for rename and merge.
- OPEN: Do tags in code blocks and in non-Markdown files count? Obsidian ignores them.
- OPEN: Is there a tag rename operation that rewrites every note?

### editor

Owns: editing a note's text: modes (source, live preview, reading), cursor and selection model, find and replace, multiple cursors, folding, autocompletion for links, tags and properties, slash commands, paste and drop handling, Vim mode, spellcheck.

- OPEN: Which editing modes ship first? Obsidian has three. Live preview is the hard one and is most of what people mean by "polished".
- OPEN: What is the text engine? Under a web view, CodeMirror 6 as both references do. Under a native toolkit, a Rust rope plus our own rendering. This follows decision 3.
- OPEN: Is the editor API (positions, ranges, transactions, undo) exposed to extensions as in Obsidian's `Editor` class?
- OPEN: Which of readable line length, line numbers, indent guides, auto-pair, smart lists, tab size, RTL are in the first release?
- OPEN: Paste behaviour. Obsidian converts HTML to Markdown with Turndown and copies dropped files into the vault as attachments. Both?
- OPEN: Vim mode: in scope at all?

### rendering

Owns: turning Markdown into displayed content: callouts, code highlighting, math, diagrams, footnotes, embeds, images with size, PDF viewing, tables.

- OPEN: Code highlighting engine and which languages.
- OPEN: Math (Obsidian: MathJax) and diagrams (Obsidian: Mermaid) in scope? Both are JavaScript libraries; under a native toolkit they need replacements.
- OPEN: Callout types and whether users can define new ones.
- OPEN: PDF viewing in app or hand off to the OS?

### workspace

Owns: the layout tree (splits, tab groups, leaves, views), left and right sidebars, ribbon, status bar, pinned tabs, linked views, tab stacking, pop-out windows, layout persistence and restore, saved named layouts, per-tab navigation history.

- OPEN: Do we adopt Obsidian's tree exactly (root split with free nesting; side docks limited to tab groups), including its `workspace.json` shape, so a vault's layout survives switching apps?
- OPEN: Are leaves created lazily (Obsidian's deferred views since 1.7.2)? This is the main startup-time trick.
- OPEN: Pop-out windows in the first release?
- OPEN: Which views exist at first: markdown, file explorer, search, backlinks, outline, tags, properties, graph, canvas, bases, empty?
- OPEN: Is there a ribbon, and is it user-orderable?
- OPEN: Is layout saved per vault (Obsidian) or per user?
- OPEN: Mobile layout (drawers instead of side docks) in scope? Follows decision 3.

### commands

Owns: the command registry, command ids and names, enablement checks, hotkeys and the hotkey editor, the command palette, slash commands in the editor, pinned commands.

- OPEN: Is every user action a command, with menus, palette, hotkeys, URI and CLI all routed through it (Obsidian's model)? Recommendation: yes, and build this before any UI chrome.
- OPEN: Hotkey file format: compatible with Obsidian's `hotkeys.json` (keyed by command id) or our own?
- OPEN: Default keymap: copy Obsidian's defaults so switchers keep muscle memory, or design our own?
- OPEN: Scoped keymaps (Obsidian's `Scope` stack) so modals and the editor can shadow global keys?

### search

Owns: full-text search over the vault, the query language, operators, property queries, regex, sort orders, result context, embedded searches.

- OPEN: Query language. Obsidian has `file: path: content: match-case: ignore-case: tag: line: block: section: task: task-todo: task-done:`, `[prop:value]`, `OR`, `-`, parentheses, `/regex/`. Copy it, subset it, or design a new one?
- OPEN: Engine. Scan the metadata cache and files on each query (Obsidian) or maintain an inverted index (tantivy, SQLite FTS5)? Trade-off is startup cost versus query latency on large vaults.
- OPEN: Semantic or vector search (Atomic's core feature) in scope at all? If yes, which domain owns the embedding provider configuration?
- OPEN: Does search cover attachments' filenames and paths (Obsidian: yes via `file:`/`path:`)?

### graph

Owns: the link graph view, global and local, filters, colour groups, forces, orphans, tag and attachment nodes.

- OPEN: In the first release at all? It is the most recognisable Obsidian feature and the least used day to day.
- OPEN: Rendering: GPU canvas (Obsidian uses pixi.js) is needed above a few thousand nodes.
- OPEN: Local graph depth and linked-view behaviour.

### files-view

Owns: the file explorer: tree, sort orders, create/rename/move by drag, folder notes, hidden and excluded files, "show unsupported files".

- OPEN: Sort orders (Obsidian: name, modified, created, each both directions).
- OPEN: Does drag-and-drop move update links (follows the links domain's rename rule)?
- OPEN: Are hidden dotfiles ever shown?

### file-history

Owns: automatic snapshots of notes, restore, diff, retention, and the relationship to any future sync.

- OPEN: Snapshot policy. Obsidian: at least 5 minutes apart, kept 7 days, `.md` and `.canvas` only, stored outside the vault keyed by absolute path.
- OPEN: Store snapshots in the index database, in XDG data, or as a git repository?
- OPEN: Is this on by default?

### templates-and-daily-notes

Owns: template folder, variable substitution (`{{title}} {{date:fmt}} {{time:fmt}}`), daily notes with date-format paths, unique note creator, new-note location policy.

- OPEN: Variable syntax and date format tokens. Obsidian uses Moment.js tokens; Rust has `chrono` with strftime. Compatibility with existing templates means parsing Moment tokens.
- OPEN: Are daily notes a separate domain or a template feature?

### canvas

Owns: `.canvas` files (JSON Canvas: text, file, link, group nodes; edges with labels, colours, directions), the infinite-canvas editor, export to image.

- OPEN: In scope at all for the first year?
- OPEN: If yes, JSON Canvas is an open spec and compatibility is free. Do we commit to it?

### bases

Owns: `.base` files, database-like views (table, list, cards, kanban, map) over notes and their properties, filters, formulas, groups, summaries, inline editing of properties from a view.

- OPEN: In scope at all? This is Obsidian's newest and fastest-growing feature and depends on properties and metadata-cache being solid.
- OPEN: Formula language: copy Obsidian's Bases syntax for compatibility, or design our own?

### extensions

Owns: the plugin model: lifecycle, what a plugin can register (commands, views, ribbon items, status bar items, settings, Markdown post-processors, code block renderers, editor extensions, protocol handlers, CLI handlers), permissions, discovery and install, restricted mode.

- OPEN: Language and sandbox. Options: JavaScript via an embedded engine (Obsidian-compatible plugins become possible), WASM with a capability-based API (real sandbox, no existing ecosystem), Lua or Rhai (small, embeddable, no ecosystem), out-of-process RPC. This is the biggest single architecture choice after decision 3.
- OPEN: Is restricted mode (no third-party code until the user opts in) the default, as in Obsidian?
- OPEN: Is there a permission model at all? Obsidian says it cannot have one. A native app can.
- OPEN: Are themes CSS (only meaningful under a web view) or a token file?

### appearance

Owns: light and dark themes, accent colour, font choices, base font size, the design-token system, CSS snippets or their equivalent, translucency, native menus.

- OPEN: Token model. Obsidian's four layers (raw palette, semantic colours, component tokens, scales) are documented in `obsidian-analysis.md` section 6 and are a good design regardless of toolkit. Adopt the layering with our own values?
- OPEN: Is the user-facing theme format Obsidian-compatible CSS? Only possible under a web view.
- OPEN: Which fonts ship? Obsidian bundles Inter Variable and Source Code Pro, both open licences.

### settings

Owns: the settings UI, per-vault versus global settings, the files they are stored in, settings search, declarative settings for extensions.

- OPEN: Which settings are per vault and which are global? Obsidian puts nearly everything per vault and only the vault list and file-recovery snapshots globally.
- OPEN: File format and names. Compatibility with `app.json` and `appearance.json` or our own.
- OPEN: Do extensions describe their settings declaratively (Obsidian's `SettingDefinition` types) so the app renders them, or render their own?

### integration

Owns: the `apperception://` (or `obsidian://`-compatible) URI scheme and its actions, the CLI, opening files from the OS, "open with", default handler registration, an automation API for other tools (Atomic's MCP server is the reference), web clipper intake.

- OPEN: URI scheme name and whether to also handle `obsidian://` for compatibility.
- OPEN: CLI in the first release? Obsidian's has about 130 subcommands and a TUI; a small one covering open, create, read, append, search, daily is a day's work once commands exist.
- OPEN: An MCP server or other programmatic API, as Atomic has? It falls out of the commands domain if commands are the universal verb.

### sync-and-publish

Owns: nothing yet. Obsidian's Sync and Publish are paid services; Atomic's server is self-hosted. Listed so the question is written down.

- OPEN: Does Apperception have any first-party sync, or does it rely on the vault being a folder (Syncthing, Dropbox, git)? Recommendation: none in scope; the files-are-truth decision makes third-party sync work.
- OPEN: Publishing to a static site?

### ai

Owns: nothing yet. Atomic's whole reason to exist. Listed so the question is written down.

- OPEN: Any AI feature in scope (semantic search, auto-tagging, summaries, chat)? If yes, Atomic's provider traits and chunking are MIT and directly reusable, and this becomes several domains.

## Glossary candidates

Terms more than one domain above already uses, which would go in `docs/spec/00-glossary.md` once you confirm them: vault, note, attachment, link, embed, tag, property, frontmatter, block, heading, view, leaf, tab group, split, command, hotkey, extension, theme, index (for metadata cache). Obsidian's own glossary (`obsidian-help/en/Getting started/Glossary.md`) is a good starting point for the definitions and is written in product language already.

## Suggested order of work

If decisions 1 to 5 are made the way I recommend, the first queue steps would be, in order: vault, notes, metadata-cache, links, commands, workspace (minimal: one split, tabs, markdown view), editor (source mode first), search, properties, tags, files-view. Everything else waits for a working first project. The `greeting` example bricks in the repo should be deleted when the first real brick lands, as the template's CONTRIBUTING.md says.

## What I did not do

I did not create any `docs/spec/*.md` files, add glossary terms, or touch `docs/queue.yaml`. All of that waits on your confirmation of which domains exist and your answers to the five decisions above. I also did not run Obsidian against a vault, so startup order and persistence formats in `obsidian-analysis.md` are inferred from its docs and typings, not observed.
