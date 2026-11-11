# Settings
## Purpose
Settings live in two scopes: global, in the [registry](00-glossary.md), and per [vault](00-glossary.md), in the vault. This domain owns which setting lives where, how the two scopes resolve, and the settings view. What each setting means belongs to the domain that uses it.

## Vocabulary
Nothing yet.

## Data
A setting has a key, a scope (global or vault), a type, a default, and a value. A vault-scoped setting applies only to that vault. The settings view lists every setting with its description and lets the user change it.

## Behavior rules
Nothing yet.

## Open questions
- OPEN: Which keys are global and which are per vault; this must be settled early because storage follows from it.
- OPEN: Whether a global setting can be overridden per vault, or the two scopes hold disjoint keys.
- OPEN: Whether changing a setting applies immediately or some settings require reopening the vault.
- OPEN: Whether the settings view is searchable.
