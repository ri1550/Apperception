# Markdown
## Purpose
Atom bodies are Markdown in Obsidian's flavour so that a vault round-trips to plain text any Markdown app can read. This domain is the list of constructs that flavour contains and the rule that bodies are preserved exactly as written.
How each construct is displayed belongs to the rendering domain once it exists; how it is edited belongs to [editor.md](editor.md).

## Vocabulary
**Construct** — one piece of Markdown syntax with a defined meaning, such as a heading, a wikilink, or a callout.
**Block id** — a `^identifier` placed at the end of a block or on its own line after it, which links can target.

## Data
The flavour is CommonMark plus GitHub Flavored Markdown plus LaTeX math, extended with wikilinks `[[title]]`, display text `[[title|text]]`, heading anchors `[[title#heading]]`, block anchors `[[title#^id]]`, embeds `![[...]]`, block ids, footnotes, comments `%%text%%`, highlights `==text==`, callouts `> [!type]`, task lists, tables, and fenced code blocks. A body may begin with a YAML frontmatter block carrying the atom's properties when it is a plain-text file.
A body is stored and exported exactly as written; the app never re-serializes it.

## Behavior rules
Nothing yet.

## Open questions
- OPEN: Mermaid diagram blocks are deferred; whether they are ever rendered is a rendering question.
- OPEN: Whether Markdown inside raw HTML blocks is left unrendered, as Obsidian does on purpose.
- OPEN: The exact set of callout types and whether users can define new ones.
- OPEN: Whether a body may contain its own YAML frontmatter block inside the app, or frontmatter exists only in plain-text files.
