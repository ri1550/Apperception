# Links
## Purpose
[Links](00-glossary.md) are how knowledge connects: an atom becomes part of the knowledge base by being linked into it, which is what the name Apperception means. This domain owns link syntax resolution, backlinks, unresolved links, rename rewriting, and the single definition of [orphaned](00-glossary.md).
Links are made by [title](00-glossary.md), as in Obsidian, so a user can read and write them without the app; the app keeps them correct when titles change.

## Vocabulary
**Backlink** — a link seen from its target: the set of atoms that link to a given atom.
**Unresolved link** — a link whose title names no atom in the vault.
**Embed** — a link written with a leading `!` that shows the target's content in place instead of a reference.
**Anchor** — the optional part of a link after `#` that names a heading, or after `#^` that names a block, inside the target atom.

## Data
A link lives in one atom's body and names a target title, optionally an anchor, and optionally display text that is local to that link. A link is resolved when an atom with that title exists, otherwise unresolved. Embeds are links to atoms or to source material.
Links are directed. For each atom the vault knows its outgoing links and its backlinks. An atom with no atom-to-atom links in either direction is orphaned; links to source material and tags do not count.

## Behavior rules
Nothing yet.

## Open questions
- OPEN: Whether a link or embed whose target has a file extension (for example `![[photo.png]]` from an imported vault) is an unresolved atom link that creates an atom on activation, or a source reference that is preserved but never created and ignored by the orphaned computation until source material exists.
- OPEN: Whether links to or from a trashed atom count for backlinks and for orphaned status.
- OPEN: Whether title matching in links is case-insensitive, as it is in Obsidian.
- OPEN: What a link to a heading or block does when the target atom exists but the anchor does not.
- OPEN: Whether Markdown-style links whose target is a title with a `.md` suffix are resolved exactly like wikilinks, including on rename rewriting.
- OPEN: The graph view's data rules (what is a node, what is an edge, whether unresolved links and orphans are shown, local-graph depth), to be settled when slice 2 is queued.
