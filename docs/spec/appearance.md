# Appearance
## Purpose
Apperception should feel as polished as Obsidian. Appearance is the theme system: light and dark modes, an accent colour, fonts, and a layered token system that themes and snippets override with CSS, as in Obsidian.

## Vocabulary
**Token** — a named value (a colour, a size, a font, a duration) that components use instead of literal values, so a theme changes it in one place.
**Snippet** — a small user-written CSS file applied on top of the theme.

## Data
The app has a base theme with light and dark variants, built from tokens in four layers: a raw palette, semantic colours, component tokens, and scales for spacing, type, icons, and motion. A theme is a CSS file that overrides tokens; snippets are CSS files applied after the theme. Fonts ship with the app; nothing loads from the network.

## Behavior rules
Nothing yet.

## Open questions
- OPEN: The token names and values for slice 1; Obsidian's layering is the model, the values are Apperception's own.
- OPEN: Which fonts ship; Obsidian bundles Inter Variable and Source Code Pro, both under open licences.
- OPEN: Where themes and snippets are stored: inside the vault directory (per vault) or globally.
- OPEN: Whether the user can pick an accent colour, a base font size, and separate interface, text, and monospace fonts, as in Obsidian.
- OPEN: Whether the app follows the system light or dark mode by default.
