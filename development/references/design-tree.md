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
