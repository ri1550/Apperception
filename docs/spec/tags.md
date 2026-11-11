# Tags
## Purpose
[Tags](00-glossary.md) group atoms by topic without folders, which Apperception does not have. A tag in an atom's body and a tag in its `tags` property are one thing, tracked in the property, so that export carries every tag.
This domain owns the tag grammar, nesting, how body tags fold into the property, and how imported folder paths become tags.

## Vocabulary
**Tag tree** — the nesting of tags by `/`, where `#inbox/reading` is a child of `#inbox`.

## Data
A tag is a string made of letters, digits, `_`, `-`, `/`, and other commonly accepted Unicode characters, with at least one non-digit, compared case-insensitively. An atom has one set of tags, drawn from its `tags` property and from `#tags` in its body. The vault knows every tag in use and how many atoms carry each.
A folder path in an imported Markdown folder becomes a tag path.

## Behavior rules
Nothing yet.

## Open questions
- OPEN: The developer has further ideas about tags to raise during building; nothing beyond Obsidian's behaviour is settled.
- OPEN: Whether renaming a tag across every atom is an operation, and whether it rewrites bodies as well as properties.
- OPEN: Which casing a tag displays with when atoms use different casings; Obsidian shows the first seen.
- OPEN: Whether tags inside code blocks or inline code count, as they do not in Obsidian.
- OPEN: On import, whether the full folder path becomes one nested tag or each folder segment becomes its own tag.
