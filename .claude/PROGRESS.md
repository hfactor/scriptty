# Scriptty — Development Progress

## Status: Phase 1 Active

---

## Completed

### Infrastructure
- [x] Tauri 2 + SvelteKit scaffold — desktop window
- [x] Claude Code config — CLAUDE.md, 3 sub-agents, hooks
- [x] Project structure scaffolded
- [x] Bundled fonts — Noto Sans Malayalam (Regular + Bold), Manjari (Regular + Bold)
- [x] Rust backend structs — ScreenplayDocument, ScreenplayMeta, ScreenplaySettings
- [x] App binary renamed to scriptty, identifier updated
- [x] App icon — ഋ clapperboard, all platform sizes generated

### Editor
- [x] ProseMirror schema — 8 node types
- [x] Tab/Enter navigation keymap — full Hollywood element flow
- [x] Shift+Enter — new scene heading from anywhere
- [x] Shift+Tab — convert Action to Scene Heading
- [x] Cmd+Z / Cmd+Shift+Z — undo/redo
- [x] Auto-uppercase for scene headings and character names (Latin only, Malayalam passthrough)
- [x] Hollywood screenplay CSS formatting — fixed pixel margins, centered content area
- [x] Page background — dark paper-on-desk aesthetic
- [x] Font rendering via :global() CSS

### Input Methods
- [x] InputModeManager — Ctrl+Space toggle English/Malayalam
- [x] Inscript 1 — static keymap
- [x] Inscript 2 — static keymap
- [x] Mozhi — full transliteration engine (greedy longest-match, conjuncts, chillus, geminate caps)
- [x] Input scheme switcher UI in status bar
- [x] Default scheme: Mozhi

### File I/O
- [x] .screenplay file format — JSON with content, meta, settings
- [x] save_screenplay, open_screenplay, new_screenplay Tauri commands
- [x] saveWithDialog() — native save dialog, Cmd+S shortcut
- [x] openDocument() — native open dialog, Cmd+O shortcut
- [x] Title derived from filename on first save
- [x] Dirty state tracking — orange dot indicator

### Scene Navigator
- [x] Collapsible left panel — Ctrl+B toggle
- [x] Auto-numbered scene list
- [x] Click-to-jump
- [x] Reactive updates on every keystroke

### Metadata
- [x] MetadataModal — title, author, contact, draft number, draft date
- [x] Meta button in TitleBar
- [x] Metadata persisted in .screenplay file

### Font Selection
- [x] Font selector UI — Noto | Manjari
- [x] Live font switching in editor
- [x] Font persisted in document settings

### PDF Export
- [x] Typst compiler integration — ScreenplayWorld trait, in-memory compilation
- [x] Hollywood single-column PDF — A4, all element types, page break rules
- [x] Indian two-column PDF — 50/50 grid, character/dialogue alignment, page break rules
- [x] Title page — auto-generated from metadata
- [x] Bundled font embedding in PDF
- [x] Export (Hollywood) and Export (Indian) buttons in TitleBar
- [x] 17 unit tests passing

---

## Phase 1 Remaining

- [ ] Character autocomplete — trigger after 2 chars in Character element, Unicode-aware
- [ ] Fountain export — UTF-8, .fountain file
- [ ] Plain text export
- [ ] Save As
- [ ] App menu — macOS native menu bar (File, Edit)

---

## Phase 2 (Deferred)

- [ ] Mozhi — Varnam JS integration for prediction/learning
- [ ] Revision tracking
- [ ] FDX export
- [ ] Fountain import
- [ ] Rachana font
- [ ] Beat board
- [ ] Script statistics
- [ ] Collaboration / cloud sync
- [ ] Mobile
