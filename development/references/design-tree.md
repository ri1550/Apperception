# Design tree: Apperception's reason to exist

Running ledger of the interview that settles top-level decision 4 from `spec-decision-map.md`. Each round records what the developer settled, in their words where possible. Settled items are the raw material for spec Purpose and Data sections; nothing here is a numbered rule yet. Started 2026-11-09.

## Root (stated by the developer before round 1)

Apperception is a PKM purpose-built for a Zettelkasten practitioner to use LLM tools to accelerate learning, not replace cognitive work. Every PKM has three requirements: stored data is future-proof (ownership and control), knowledge complexity can grow over time (interconnection, emergence, synthesis), and entry and retrieval are informal and open-ended (multimedia, frictionless). Material falls into three classes. The knowledge base is first-class; the other two are second-class. The name is Kant's apperception: a representation counts as mine only if "I think" can accompany it; new information is consciously integrated into existing knowledge.

## Round 1 (settled 2026-11-09)

Audience. Built for the developer first, as an autodidact aiming to be a polymath. Open source so others may adopt it. Single user. No collaboration, no publishing. A server-hosted, web-served version may come later.

Method. Luhmann-style Zettelkasten: fleeting notes, permanent notes, literature notes; note IDs; internal and external links; indexes.

Future-proofing. The store is a database. Import and export to plain text is mandatory and one-to-one: one plain-text file imported makes one atom; one atom exports as one plain-text file. Only the knowledge base must be future-proof; source and synthetic material need not be.

Vocabulary. A vault holds one knowledge base. The knowledge base is the set of atoms in the vault. An atom is a single unit of content, resembling a note. Atoms are temporary (fleeting) or permanent (evergreen). Atoms are the building blocks for other human-written knowledge material assembled from atoms (no name yet). Source material is internal (a stored PDF) or external (a URL). Synthetic material is LLM-generated from knowledge material or source material.

Three requirements, in the developer's terms. Interconnection is how knowledge is connected (structure). Emergence is discovery, machine-surfaced or human-surfaced (process). Synthesis is new material, human-written or machine-written (material).

Material rules. Properties apply to a whole atom. An atom may pull source material in by reference but does not contain it. Synthetic material is never pulled into an atom and an atom never contains synthetic material. Material never changes class. Everything in the knowledge base is written by the user as a new atom, which may link to source material. Synthetic material must carry provenance to the knowledge or source material it was generated from. The LLM writes text only to create synthetic material.

Multimedia. Parity with Obsidian: text, images, PDF, audio, video, math and code blocks, embedded web media (web pages, YouTube).

Retrieval. Reference points are Obsidian plus the Omnisearch plugin, then Atomic's search, with the aim of improving on both.

LLM. Provider-agnostic; local host and OpenRouter are the two main options; data-control settings including per-vault opt-in.

Reading and learning. Source material is read inside the app and atoms are written beside it. Resurfacing, suggested connections, spaced repetition, and other learning aids are in scope.

Relationship to references. Apperception replaces both Obsidian and Atomic for the developer. From Atomic: semantic search, suggested links, auto-tagging, wiki synthesis, scheduled reports, chat, MCP server, canvas are all in.

The name as a rule. Yes: whether a note has been consciously connected into existing knowledge is one of the differences between a fleeting note and an evergreen note.

Deferred from round 1: where internal source material is stored (question 10); what "second-class" costs source and synthetic material operationally (question 11).

Facts found for round 2. Omnisearch 1.32.0 (GPL-3, study only) is a BM25 engine over MiniSearch with typo tolerance, weights for basename, directory, headings, tags and custom properties, a recency boost, excerpts with highlights, vault and in-file modes, quoted phrases and `-exclusions`, file-type filters, and indexing of PDFs, Office documents and images through a separate Text Extractor plugin (OCR). Atomic's search is hybrid: SQLite FTS5 keyword plus sqlite-vec cosine over chunk embeddings, with similarity thresholds 0.3 for search and 0.5 for related atoms.

## Round 2 (settled 2026-11-09)

Atom kinds. Three kinds of note: fleeting, permanent, literature. A literature note is its own kind, not a permanent atom with a source reference.

Molecule. The human-written material assembled from atoms is called a Molecule. Indexes and hubs are not Molecules.

Indexes. An index is synthetic material, generated from criteria the user defines. It is not hand-curated.

Identity. Every atom has an immutable ID assigned at creation and a title. Both are required.

Fleeting to permanent. A state change on the same atom, caused by integrating it into the vault, that is, connecting it to other atoms. Fleeting atoms live indefinitely by default and are surfaced for review; they are never auto-deleted; the user can configure fleeting-note behaviour.

Tags and links. Suggested by the model, accepted and applied by the user. Auto-tagging (the model applies tags without acceptance) exists as an opt-in setting.

Synthetic material. Read-only to the human. Conversations are stored as synthetic material with provenance, outside the knowledge base, deletable. Derived synthetic is flagged stale when its sources change and regenerated on demand. Synthetic is generated directly from knowledge (and source) material, never from other synthetic. Suggestions (links, tags, resurfacing) are ephemeral, not material; dismissals are remembered. Scheduled reports: all of Atomic's report features, with reports created and designed by the user. Wiki synthesis is written per tag.

Source material. Machine-extracted text (OCR, PDF text, transcripts) is source material with provenance to the original: searchable and citable, not synthetic. Highlights and annotations attach to the source, not to atoms, with a path from an annotation to a new atom that cites the exact position. No web-capture or collection system.

Retrieval. One search box with blended lexical, semantic and fuzzy ranking and explicit operators to narrow, in the spirit of Obsidian's query language. Atoms are ranked by default; source and synthetic material are reachable only through an explicit filter and never mixed into the default ranking.

Learning. Gap detection is in. Spaced repetition design is an open question for later. Explain-it-back, reading queue, curricula, progress tracking are out for now. Fields of study are emergent through tags, indexes and links, never an explicit hierarchy.

Entry and platform. Quick capture (hotkey, CLI, MCP, share sheet, email) is not in scope. Mobile comes later through the served web version.

LLM. Every knowledge-base operation works with no model and no network; LLM features are additive, never load-bearing. Each model role (embedding, generation, transcription, vision) can use a different provider. Data control is per vault; finer controls decided later.

Integration. MCP is read-only. A manual canvas is a Molecule (in the knowledge base); the semantic force-directed layout is a retrieval view (outside it).

Project. Multiple independent vaults per user; the vault registry lives outside any vault. Licence decided later.

Open after round 2: internal source storage location (10); operational cost of being second-class (11); spaced repetition design (40); finer data controls (47); licence (51).

## Round 3 (settled 2026-11-09) and closure of "reason to exist"

Molecule is a kind of atom: one identity scheme, one export rule, and "the knowledge base is the atoms" stays literally true. Atom kinds are therefore fleeting, permanent, literature, molecule.

Hubs are dropped. There are only indexes, which are synthetic.

External sources are rendered read-only inside the app, with annotations stored against the URL.

Still open at closure: internal source storage location (10, now folded into the source-of-truth branch below); operational cost of being second-class (11); spaced repetition design (40); promotion trigger and reversion for fleeting to permanent (54); finer data controls (47); licence (51).

Statement of the reason to exist, as settled. Apperception is an open-source, single-user PKM built for a Luhmann-style Zettelkasten practitioner who is an autodidact. It stores a knowledge base of atoms (fleeting, permanent, literature, molecule), each with an immutable ID and a title, written only by the user, in a database that imports and exports one atom per plain-text file. Around the knowledge base sit two second-class material classes: source material (internal files or external URLs, read and annotated inside the app, with machine-extracted text counted as source) and synthetic material (indexes, wiki articles per tag, user-designed scheduled reports, conversations; read-only, provenance-bearing, stale-flagged, regenerable, never an input to further synthesis). LLMs suggest links and tags for the user to accept, power blended search and gap detection, and write only synthetic material; every knowledge operation works with no model configured. It replaces both Obsidian and Atomic for its author and borrows Obsidian's multimedia and polish and Atomic's database, pipeline and LLM features.

# Design tree: source of truth (top-level decision 1)

Root, stated by the developer: "Atomic is our truth." The database is the source of truth; plain-text files are import and export, not storage.

## Round 1 of decision 1 (settled 2026-11-11)

Engine. SQLite only.

Vault on disk. Provisionally a self-contained directory, as Obsidian does. Needs prototyping, testing and an ADR before it is final. Internal source storage (58), single versus split database files (59, two files most likely), concurrent access (77) and backup and sync (78) all hang off that prototype and ADR.

Registry. A registry outside any vault, most likely a SQLite file in the XDG data directory.

Scale. The developer's previous vault was thousands of files and 1.1 million words. Apperception must handle vaults of multiple millions of words.

Round trip. One atom exports as one Markdown file with YAML frontmatter carrying its properties; the atom body is Markdown in the database. Titles are unique within a vault. The exported filename is `<title>.md` with the ID in frontmatter. Inside the app, links are made by atom title, as in Obsidian; on export they are written as `[[title]]`. Export then import yields an identical knowledge base, and the spec will pin that with a test. Export is on demand, with an opt-in per-vault mirror folder for git-friendly plain-text tracking. Exported or mirrored files are never read back automatically; import is always explicit. Knowledge is always exported; source and synthetic material are optional exports.

Import. There are no folders in Apperception; an imported folder path becomes a tag path unless a better scheme appears. Frontmatter that Apperception has not defined and cannot use is dropped on import; the developer notes this cuts against the ownership rule and accepts it to avoid future design problems. Attachments on import (74): deferred. Whether properties are stored structured or as frontmatter text (63): deferred.

Fleeting notes are gone. The fleeting versus permanent distinction is dropped. There are only linked and orphaned notes: an orphaned note has no links, and linking it to another note stops it being orphaned. This reshapes rounds 2 and 3: the promotion trigger (54) is resolved because the status is derived from links, and the "fleeting behaviour" settings (27) become settings for surfacing orphaned notes.

History. Some form of atom history is wanted; which design is a prototype-and-ADR question. Flagged as important: it must be resolved before it becomes impossible to add for past data, that is, before the first real vault is written.

Deletion. Trash with configurable retention; permanent deletion is an explicit second act by default.

Encryption at rest. Left to the OS; may be reopened.

Items the developer has marked for prototype, test and ADR: 57 vault layout, 58 source storage, 59 database split, 75 history, 77 concurrency, 78 backup and sync.

## Round 2 of decision 1 (settled 2026-11-11) and closure of "source of truth"

Orphaned. Confirmed: atom kinds no longer include fleeting; the name-as-rule is connected versus orphaned; the old fleeting-behaviour settings become settings for surfacing orphaned atoms; the promotion trigger is gone because orphaned is derived. An atom is orphaned when it has no atom-to-atom links in either direction. Links to source material and tags do not count.

Links by title. Renaming an atom rewrites the link text in every atom that links to it, automatically and silently, as Obsidian does. An atom may link to a title that does not exist; activating that link creates the atom; unresolved links appear in the graph. Title constraints (filename-safe on every OS, no `# ^ |`, length ceiling) are enforced at creation and at rename.

Import. A file whose frontmatter ID already exists updates that atom in place, but the user is shown both versions side by side and accepts or denies the update, like a file-manager overwrite prompt. Kind is a frontmatter property on export and import; files without it become plain notes. No report of dropped frontmatter.

Speed. Opening a vault: about three seconds at most. Opening an atom: instant. Writing in an atom: no delay. Search: a small delay is acceptable. Overall target is Obsidian's feel.

Parked. Literature notes and molecules are parked as future ideas. For now there is one atom kind, the note. The manual canvas (49) was defined as a molecule and is parked with it.

Open at closure: properties structured versus frontmatter text (63); attachments on import (74); the prototype-and-ADR list (57, 58, 59, 75, 77, 78).

# Design tree: Obsidian compatibility (top-level decision 2)

Root: how far Apperception matches Obsidian's on-disk format and behaviour. Already narrowed by decision 1: the body is Obsidian-flavoured Markdown, export is `<title>.md` with frontmatter and `[[title]]` links, import reads Obsidian vaults with folders becoming tag paths, and there is no shared live folder because the database is the truth.

## Round 1 of decision 2 (settled 2026-11-11) and closure of "Obsidian compatibility"

Level: same Markdown flavour only. No reading of `.obsidian/` configuration, no running of Obsidian plugins. The opt-in mirror folder is a folder Obsidian can open as-is, and Apperception writes nothing Obsidian-specific into it.

Importers: plain Markdown folders only. Users export from other apps to Markdown first.

Links: every Obsidian link construct round-trips exactly: `[[title|alias]]`, `[[title#heading]]`, `[[title#^block]]` with `^block-id` markers, `![[...]]` embeds of atoms and source files, and `[text](title.md)` Markdown links. Link features match Obsidian's.

Tags: a `#tag` in the body is included in the atom's `tags` property, which is where tags are tracked; same grammar as Obsidian. The developer has further ideas about tags to raise during building.

Aliases: only the per-link display text `[[title|alias]]`, which is local to that atom body. There is no `aliases` property and no alias uniqueness.

Other Obsidian extensions (callouts, highlights, comments, footnotes, task lists, math, tables, Mermaid): all preserved exactly; all rendered except Mermaid, which is deferred to the rendering domain.

Defined properties: `id`, `title`, `created`, `modified`, `tags`. Nothing else survives import.

`.canvas` and `.base` files: skipped on import. The developer wants an Apperception canvas system later.

Default keymap: copy Obsidian's (or Atomic's) hotkeys where the same action exists; expected to change later. No `obsidian://` handling; only Apperception's own scheme, if any.

Frontier empty at closure. Nothing deferred.

# Design tree: UI technology (top-level decision 3)

Facts found before round 1 (2026-11-11).

The developer's machine: Arch Linux, Hyprland on Wayland, NVIDIA RTX 3070, webkit2gtk-4.1 2.52.6 installed, Rust 1.98, Node 26.

Tauri 2 on Linux renders through WebKitGTK. NVIDIA plus Wayland is the known bad case: blank windows, flicker, and "Error 71 (Protocol error) dispatching to Wayland display" from the DMA-BUF renderer requesting buffer formats the NVIDIA driver lacks. Tauri documents workarounds (`WEBKIT_DISABLE_DMABUF_RENDERER=1`, `__NV_DISABLE_EXPLICIT_SYNC=1`), and NVIDIA drivers 555+ with explicit sync remove the crash on current stacks. Sources: https://v2.tauri.app/develop/debug/linux-graphics/ , https://github.com/tauri-apps/tauri/issues/9394 .

Tauri 3.0.0-alpha (September 2026) adds a Chromium Embedded Framework runtime (`tauri-runtime-cef`) beside the system-webview runtime, on Windows, macOS and Linux. It ships its own Chromium, so bundles grow by hundreds of megabytes and memory use rises, but it is the same engine class as Obsidian's Electron. Servo/Verso as a Tauri runtime is in limbo for lack of resources. Sources: https://v3.tauri.app/develop/cef/ , https://github.com/orgs/tauri-apps/discussions/15546 .

Native Rust GUI in 2026: still fragmented. gpui has no built-in text input widget; Slint and egui have the best IME and accessibility; iced is Elm-style; Dioxus is pivoting to its Blitz renderer. No native toolkit has a Markdown live-preview editor comparable to CodeMirror 6, and none renders web pages, YouTube embeds or PDFs without embedding a browser engine anyway. Sources: https://blog.wybxc.cc/blog/rust-gui-survey-2026/ , https://www.boringcactus.com/2025/04/13/2025-survey-of-rust-gui-libraries.html .

Implication: the settled requirements (multimedia parity with Obsidian, in-app rendering of URLs, PDF reading, video) need a browser engine somewhere in the app. Obsidian itself is a web view, so the Obsidian-speed target is achievable in one.

## Round 1 of decision 3 (settled 2026-11-11)

Web view, probably; the developer wants to look at how Obsidian does it, since Obsidian handles multimedia and web embeds and Atomic does not. Leaning to Electron, as Obsidian uses, rather than Tauri; whether Chromium must be bundled is to be resolved. Leaning to Rust for the frontend; TypeScript only where needed, possibly with a Rust and TypeScript bridging tool. Framework choice follows from the language. CodeMirror 6 confirmed as the editor. Transport between frontend and core: deferred. Repository layout (Rust core as components, desktop base and project, frontend folder outside Cargo) confirmed with refinement allowed. Themes: users can write custom themes in CSS, as in Obsidian, and possibly other code. Offline rule confirmed: every asset ships in the bundle. Linux first, then macOS, then Windows. Mobile not now. PDF and media playback: prototype, test and ADR. Mermaid deferred.

Facts found for round 2. Electron is Chromium plus Node.js in one bundle; there is no system-webview mode, so bundling Chromium is not optional (Obsidian's Linux build is 224 MB unpacked, 24 MB of which is the app). "tsrust" is the name of several unrelated tools: a TypeScript compiler written in Rust, a Rust build CLI, and an LLM-written port of tsc; the library that generates TypeScript types from Rust types is `ts-rs` (https://github.com/Aleph-Alpha/ts-rs). Rust-to-WebAssembly frontends in 2026: Leptos (fine-grained signals, smallest WASM, about 25 KB minimal) and Dioxus (React-like, signals since 0.6, also targets native) are the two live options, both at parity with the fast JavaScript frameworks in js-framework-benchmark; Yew is older and slower. Both call JavaScript through wasm-bindgen, which is how CodeMirror 6, pdf.js and media elements would be reached. Sources: https://reintech.io/blog/leptos-vs-yew-vs-dioxus-rust-frontend-framework-comparison-2026 , https://www.pistack.xyz/posts/2026-08-29-dioxus-vs-yew-vs-leptos-rust-wasm-frameworks-comparison/ .

## Round 2 of decision 3 (settled 2026-11-11) and closure of "UI technology"

Shell: Electron now. The Rust core runs as a Node native addon (napi-rs) called in-process from Electron's main process; revisit later. This partly resolves the deferred transport question (105): renderer to main over Electron IPC, main to Rust directly. Frontend language: a prototype-and-ADR question; several languages in the repo are acceptable as long as the goals are met. Framework, if Rust: Leptos by default, under BSSN. CodeMirror 6 extensions live in a TypeScript package; Rust owns everything else. Themes are CSS, as in Obsidian, plus CSS snippets. Linux packaging: revisit when it matters. Prototype list confirmed: an Electron window with a napi-rs Rust addon and a Leptos renderer running CodeMirror on the developer's machine; pdf.js plus media playback.

Open at closure: transport detail (105, partly resolved); frontend language and framework (114, 115, prototype and ADR); Linux packaging (118). Prototype-and-ADR list now: 57, 58, 59, 75, 77, 78, 111, 114, 115, 119.

# Design tree: what ships first (top-level decision 5)

Root: the first deployable project in the Polylith queue, and the order of what follows.
