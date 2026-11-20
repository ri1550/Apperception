# Tags
## Purpose
[Tags](00-glossary.md) group atoms by topic without folders, which Apperception does not have. A tag in an atom's body and a tag in its `tags` property are one thing, tracked in the property, so that the file carries every tag.
This domain owns the tag grammar, nesting, and how body tags fold into the property.

## Vocabulary
**Tag tree** — the nesting of tags by `/`, where `#inbox/reading` is a child of `#inbox`.

## Data
A tag is a string made of letters, digits, `_`, `-`, `/`, and other commonly accepted Unicode characters, with at least one non-digit, compared case-insensitively. An atom has one set of tags, drawn from its `tags` property and from `#tags` in its body. The vault knows every tag in use and how many atoms carry each.

## Behavior rules
Nothing yet.

## Open questions
- OPEN: The developer has further ideas about tags to raise during building; nothing beyond Obsidian's behaviour is settled.
- OPEN: Whether renaming a tag across every atom is an operation, and whether it rewrites bodies as well as properties.
- OPEN: Which casing a tag displays with when atoms use different casings; Obsidian shows the first seen.
- OPEN: Whether tags inside code blocks or inline code count, as they do not in Obsidian.
- OPEN: Whether folder paths become tags at all when a folder is adopted. Ledger 71 said a folder path becomes a tag path; ledger 123 said Apperception is blind to folder structure and only moves the files. If they do, whether the full path is one nested tag or each segment its own tag.
