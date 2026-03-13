# Scriptty — Development Progress

## Status: Phase 2 Active

---

## Phase 1 — Completed

### Infrastructure
- [x] Tauri 2 + SvelteKit scaffold — desktop window
- [x] Claude Code config — CLAUDE.md, 3 sub-agents, hooks
- [x] Project structure scaffolded
- [x] Bundled fonts — Noto Sans Malayalam (Regular + Bold), Manjari (Regular + Bold)
- [x] Rust backend structs — ScreenplayDocument, ScreenplayMeta, ScreenplaySettings
- [x] App binary renamed to scriptty, identifier updated
- [x] App icon — ഋ clapperboard, all platform sizes generated
- [x] adapter-static for Tauri build

### Editor
- [x] ProseMirror schema — 8 node types
- [x] Tab/Enter navigation keymap — full Hollywood element flow
- [x] Shift+Enter — new scene heading from anywhere
- [x] Shift+Tab — convert Action to Scene Heading
- [x] Cmd+Z / Cmd+Shift+Z — undo/redo
- [x] Auto-uppercase for scene headings and character names (Latin only, Malayalam passthrough)
- [x] Hollywood screenplay CSS formatting — fixed pixel margins, centered content area
- [x] Page background — cream page on dark/light desk aesthetic
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
- [x] Save As — Cmd+Shift+S, always opens file dialog
- [x] openDocument() — native open dialog, Cmd+O shortcut
- [x] Title derived from filename on first save
- [x] Dirty state tracking — amber dot indicator

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
- [x] Font selector UI — segmented control (Noto | Manjari)
- [x] Live font switching in editor
- [x] Font persisted in document settings

### PDF Export
- [x] Typst compiler integration — ScreenplayWorld trait, in-memory compilation
- [x] Hollywood single-column PDF — A4, all element types, page break rules
- [x] Indian two-column PDF — 50/50 grid, character/dialogue alignment, page break rules
- [x] Title page — auto-generated from metadata
- [x] Bundled font embedding in PDF
- [x] Export buttons in TitleBar
- [x] 17 unit tests passing

### UI / Design System
- [x] Full UI revamp — CSS custom properties, warm Kerala-rooted palette
- [x] Dark/light theme toggle — themeStore with localStorage persistence
- [x] TitleBar — ghost buttons, segmented font selector, teal primary Save
- [x] Status bar — full-width bottom bar (was floating corner)
- [x] MetadataModal — backdrop blur, scale animation, themed inputs
- [x] SceneNavigator — accent left-border on active, cubic-bezier transitions
- [x] Scrollbar styling — thin, muted color
- [x] App menu — macOS native menu bar (File + Edit) with Tauri 2 menu API

### Phase 1 Remaining (Not Started)
- [ ] Character autocomplete — trigger after 2 chars in Character element, Unicode-aware
- [ ] Fountain export — UTF-8, .fountain file
- [ ] Plain text export

---

## Phase 2 — In Progress

Build order (minimizes dependencies):

### 1. Help/About Menu
- [ ] Add Help submenu to macOS native menu bar
- [ ] "About Scriptty" menu item → emits `menu-about` event
- [ ] AboutModal.svelte — logo, version, developer info, credits
- [ ] "Report an Issue" → opens GitHub issues in browser
- [ ] "View on GitHub" → opens repo in browser
- [ ] Version from tauri.conf.json or package.json

### 2. Story Panel
- [ ] Add `story` field to ScreenplayDocument (Rust + TypeScript)
  - ScreenplayStory struct: idea, synopsis, treatment (all String)
- [ ] StoryPanel.svelte — three collapsible sections with text areas
- [ ] Tab switcher at top of left panel: Scenes | Story
- [ ] Ctrl+Shift+B shortcut to open Story Panel
- [ ] Malayalam input support in Story Panel text areas
- [ ] Data persisted in .screenplay JSON

### 3. Export Modal
- [ ] ExportModal.svelte — replaces separate Hollywood/Indian buttons
- [ ] Checkbox sections: Title Page, Synopsis, Treatment, Screenplay, Scene Cards
- [ ] Format radio: Hollywood / Indian
- [ ] Combined PDF generation in Rust (concatenated sections)
- [ ] Synopsis PDF section: centered heading, prose layout
- [ ] Treatment PDF section: same as synopsis
- [ ] Scene Cards PDF section: table/card layout
- [ ] Single "Export" button in TitleBar opens modal

### 4. Scene Cards
- [ ] Add `scene_cards` field to ScreenplayDocument (Rust + TypeScript)
  - SceneCard struct: scene_index, description, shoot_notes
- [ ] Scene heading parser — extract location, time of day from heading text
- [ ] Character extractor — collect Character elements per scene
- [ ] Page estimate — character count / average chars per page
- [ ] SceneCardsView.svelte — grid of cards, 2–3 columns
- [ ] Editable description and shoot notes per card
- [ ] "Back to Script" button to return to editor
- [ ] Cmd+Shift+K shortcut to toggle view

---

## Deferred (Not Planned Yet)

- [ ] Mozhi — Varnam JS integration for prediction/learning
- [ ] Revision tracking / colored revision pages
- [ ] FDX export
- [ ] Fountain import
- [ ] Rachana font
- [ ] Script statistics
- [ ] Collaboration / cloud sync
- [ ] Mobile
