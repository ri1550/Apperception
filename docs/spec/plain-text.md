# Plain text
## Purpose
The knowledge base must be future-proof: at any time it can leave the app as plain-text files that any Markdown application can read, and come back without loss. This domain is that round trip.
Import is the only way files enter a vault and is always an explicit act. Export is on demand, with an optional mirror folder kept for plain-text tracking. Files are never read back automatically.

## Vocabulary
**Mirror** — an opt-in folder, per vault, that holds an exported copy of the knowledge base kept up to date by the app.
**Conflict** — an imported file whose frontmatter ID already belongs to an atom in the vault.

## Data
One atom corresponds to one Markdown file named `<title>.md` whose YAML frontmatter carries `id`, `title`, `created`, `modified`, and `tags`, and whose body is the atom's body with links written as `[[title]]`. Frontmatter the app does not define is dropped on import. An imported folder path becomes a tag path (see [tags.md](tags.md)). Files that are not Markdown, including `.canvas` and `.base`, are skipped.
Export then import yields an identical knowledge base. Knowledge is always exported; source and synthetic material are optional exports once they exist.

## Behavior rules
Nothing yet.

## Open questions
- OPEN: Whether the mirror is written on every save or only on demand (by a command or a timer).
- OPEN: What happens on import when a file's title matches an existing atom but its ID does not (or it has no ID): a conflict prompt like the ID case, a renamed duplicate, or a refusal.
- OPEN: Whether `created` and `modified` on import come from frontmatter, from the file's timestamps when frontmatter lacks them, or from the moment of import.
- OPEN: What import does with attachments referenced by imported notes before source material exists (ledger 74).
- OPEN: What the import summary reports (counts of new, updated, conflicts, skipped) given that dropped frontmatter is not reported.
- OPEN: What the exported form of source and synthetic material is, once they exist.
