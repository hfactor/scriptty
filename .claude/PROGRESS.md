# Scriptty — Development Progress

## Completed

### Project scaffold
- Tauri 2 + SvelteKit + TypeScript + Vite project initialized
- Directory structure matching CLAUDE.md spec
- SSR disabled for editor page (`+page.ts`)

### Rust backend foundation
- `ScreenplayDocument`, `ScreenplayMeta`, `ScreenplaySettings` structs with serde (de)serialization and defaults
- `screenplay/mod.rs` exports document module; `pdf.rs` placeholder
- `BundledFont` struct and `bundled_fonts()` with `include_bytes!` for all 4 fonts (Noto Sans Malayalam .ttf, Manjari .otf)
- `commands/mod.rs` declares `file` and `export` submodules (placeholder implementations)
- `lib.rs` declares `commands`, `fonts`, `screenplay` modules
- Compiles clean with `cargo check`

### ProseMirror editor
- Screenplay schema with 7 node types: `doc`, `scene_heading`, `action`, `character`, `parenthetical`, `dialogue`, `transition`, `text`
- No marks (no bold/italic/underline)
- Each element renders as `<p>` with `data-type` attribute for DOM round-tripping

### Tab/Enter keymap
- Enter creates new block of correct type per transition table
- Tab changes current node type in-place (action↔character, dialogue→character)
- Shift-Tab reverts character/dialogue to action

### Malayalam input system
- `InputModeManager` singleton: tracks `isMalayalam` toggle and active `scheme`
- Inscript 2 keymap: 80+ mappings (vowels, matras, consonants, chillus, Malayalam numerals)
- Inscript 1 keymap: original layout with differences (e.g. `z`→`െ`, `]`→`്`)
- Mozhi stub: `processMozhiKey()` returns null, ready for Varnam JS integration
- Capture-phase keydown listener on `view.dom` intercepts keys before ProseMirror

### Editor UI
- `Editor.svelte` component with ProseMirror wired up (schema + keymap + history + autoUppercase)
- Ctrl+Space toggles English/Malayalam mode with visual flash feedback
- Status bar showing current mode (ENGLISH/MALAYALAM) and element type
- Hollywood-format CSS with fixed-width 680px content area, pixel-based margins
- `:global()` CSS selectors for ProseMirror-generated DOM nodes
- CSS counter for auto-numbered scene headings
- Dark theme: #141414 background, #1c1c1c editor surface, box-shadow page-on-desk look
- `@font-face` declarations for all 4 bundled fonts
- Auto-uppercase plugin for Latin letters in scene_heading/character (preserves Malayalam)

### File I/O
- `commands/file.rs`: `new_screenplay`, `save_screenplay`, `open_screenplay` Tauri commands
- `documentStore.svelte.ts`: reactive store with `loadTrigger` pattern to avoid $effect re-triggers
- `saveWithDialog()` handles save-as when no path exists
- Title derived from filename if meta.title is empty
- Cmd+S / Cmd+O keyboard shortcuts on window-level keydown

### Scene Navigator
- `SceneNavigator.svelte`: collapsible left panel (220px), toggle with Ctrl+B
- Scans ProseMirror content for scene_heading nodes
- Click-to-scroll via ProseMirror TextSelection
- Auto-numbered scene list

### PDF Export (Hollywood)
- `screenplay/pdf.rs`: full Typst PDF generation pipeline
- `ScreenplayWorld` struct implementing `typst::World` trait for in-memory compilation
- ProseMirror JSON → Typst markup → PDF bytes (no temp files)
- Font embedding: selected font bundled into every PDF
- Page break rules: element grouping into unbreakable blocks
  - Scene headings grouped with first action paragraph
  - Character names grouped with following parentheticals/dialogue
  - `ScreenplayGroup` enum (SceneBlock, CharacterBlock, Standalone)
- `ExportButton.svelte`: calls `export_pdf`, save dialog, writes bytes via Tauri FS plugin
- `export_typst_markup` command for debugging/preview
- 16 unit tests, all passing

### Keymap enhancements
- Shift+Enter: universal "new scene" shortcut from any element
- Shift+Tab at offset 0: action → scene_heading
- Undo/Redo: Mod-z / Shift-Mod-z

## In Progress

Nothing currently in progress.

## Next Up

1. **Fountain export** — UTF-8 plain text for interoperability
2. **Plain text export** — readable draft sharing
3. **Title page** — auto-generated at PDF export from `meta` fields
4. **Character autocomplete** — trigger after 2 chars in Character element, Unicode-aware
5. **Mozhi integration** — Varnam JS for transliteration input
6. **Font selection UI** — switch between Noto Sans Malayalam and Manjari
7. **Indian two-column PDF export** — alternate layout option at export time

## Known Issues

- Default input scheme is `inscript2` (temporary) — should respect user settings once settings UI exists
- Mozhi input scheme is a non-functional stub — selecting it silently does nothing
