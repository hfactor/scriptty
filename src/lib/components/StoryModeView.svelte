<script lang="ts">
  import { documentStore } from '$lib/stores/documentStore.svelte';
  import { InputModeManager } from '$lib/editor/input/InputModeManager';
  import type { InputScheme } from '$lib/editor/input/InputModeManager';

  let { onClose }: { onClose: () => void } = $props();

  // Get the shared input manager singleton
  const inputManager = InputModeManager.getInstance();

  // Input mode UI state — synced with the singleton
  let currentMode = $state<'ENGLISH' | 'MALAYALAM'>(inputManager.isMalayalam ? 'MALAYALAM' : 'ENGLISH');
  let currentScheme = $state<InputScheme>(inputManager.scheme);
  let modeFlash = $state(false);

  // Reference to the textarea element for cursor manipulation
  let textareaEl = $state<HTMLTextAreaElement | null>(null);

  let narrative = $derived(documentStore.document?.story.narrative ?? '');

  function updateNarrative(event: Event) {
    const value = (event.target as HTMLTextAreaElement).value;
    if (documentStore.document) {
      documentStore.document.story.narrative = value;
      documentStore.markDirty();
    }
  }

  // Word count for the narrative
  let wordCount = $derived(() => {
    const text = narrative.trim();
    if (!text) return 0;
    return text.split(/\s+/).length;
  });

  function selectScheme(scheme: InputScheme) {
    inputManager.setScheme(scheme);
    currentScheme = scheme;
  }

  /**
   * Handle keydown on the textarea — intercepts keys for Malayalam input,
   * Ctrl+Space mode toggle, and Escape to close.
   *
   * This mirrors the Editor.svelte keydown handler but operates on a plain
   * textarea instead of ProseMirror. Instead of dispatching transactions,
   * we manipulate the textarea value and selectionStart/selectionEnd directly.
   */
  function handleKeydown(event: KeyboardEvent) {
    // Escape closes Story Mode
    if (event.key === 'Escape') {
      onClose();
      return;
    }

    // Ctrl+Space — toggle English/Malayalam
    if (event.ctrlKey && event.code === 'Space') {
      event.preventDefault();
      const isNowMalayalam = inputManager.toggle();
      currentMode = isNowMalayalam ? 'MALAYALAM' : 'ENGLISH';
      currentScheme = inputManager.scheme;
      modeFlash = true;
      setTimeout(() => { modeFlash = false; }, 500);
      return;
    }

    // Don't intercept keys with Cmd/Ctrl modifiers (save, undo, select all, etc.)
    if (event.metaKey || event.ctrlKey) return;

    // Reset Mozhi buffer on word boundaries and navigation keys
    if (['Space', 'Enter', 'Backspace', 'Delete', 'ArrowLeft', 'ArrowRight',
         'ArrowUp', 'ArrowDown', 'Home', 'End'].includes(event.code)) {
      inputManager.resetMozhi();
      return;
    }

    // Malayalam input processing — only for single printable characters
    if (inputManager.isMalayalam && event.key.length === 1 && !event.altKey) {
      const result = inputManager.processKey(event.key);
      if (result !== null && textareaEl) {
        event.preventDefault();

        const ta = textareaEl;
        const start = ta.selectionStart;
        const end = ta.selectionEnd;
        const value = ta.value;

        // Delete back characters (for Mozhi backtracking)
        const deleteFrom = start - result.deleteBack;
        // Build new value: text before delete point + new text + text after cursor
        const newValue = value.substring(0, deleteFrom) + result.text + value.substring(end);

        // Update the textarea value
        ta.value = newValue;
        // Set cursor position after the inserted text
        const newCursorPos = deleteFrom + result.text.length;
        ta.selectionStart = newCursorPos;
        ta.selectionEnd = newCursorPos;

        // Sync the value to the document store
        if (documentStore.document) {
          documentStore.document.story.narrative = newValue;
          documentStore.markDirty();
        }
      }
    }
  }
</script>

<div class="story-mode">
  <div class="story-toolbar">
    <div class="toolbar-left">
      <button class="btn-back" onclick={onClose}>Back to Script</button>
    </div>
    <div class="toolbar-center">
      <span class="toolbar-title">Story Mode</span>
    </div>
    <div class="toolbar-right">
      <span class="word-count">{wordCount()} words</span>
    </div>
  </div>

  <div class="story-editor">
    <div class="page">
      <textarea
        class="narrative-textarea"
        placeholder="Write your full-length story here. This is your space for long-form narrative — no formatting constraints, no element types. Just write."
        value={narrative}
        oninput={updateNarrative}
        onkeydown={handleKeydown}
        bind:this={textareaEl}
      ></textarea>
    </div>
  </div>

  <div class="status-bar">
    <div class="status-left">
      <span class="status-mode" class:malayalam={currentMode === 'MALAYALAM'} class:flash={modeFlash}>
        {currentMode}
      </span>
      {#if currentMode === 'MALAYALAM'}
        <span class="status-separator">|</span>
        <span class="scheme-selector">
          <button
            class="scheme-btn"
            class:active={currentScheme === 'mozhi'}
            onclick={() => selectScheme('mozhi')}
          >Mozhi</button>
          <button
            class="scheme-btn"
            class:active={currentScheme === 'inscript2'}
            onclick={() => selectScheme('inscript2')}
          >Inscript 2</button>
          <button
            class="scheme-btn"
            class:active={currentScheme === 'inscript1'}
            onclick={() => selectScheme('inscript1')}
          >Inscript 1</button>
        </span>
      {/if}
    </div>
    <span class="status-hint">Ctrl+Space to toggle language</span>
  </div>
</div>

<style>
  .story-mode {
    flex: 1;
    display: flex;
    flex-direction: column;
    background: var(--surface-base);
    overflow: hidden;
  }

  .story-toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 16px;
    border-bottom: 1px solid var(--border-subtle);
    flex-shrink: 0;
    background: var(--surface-elevated);
  }

  .toolbar-left,
  .toolbar-right {
    width: 160px;
  }

  .toolbar-right {
    text-align: right;
  }

  .toolbar-center {
    flex: 1;
    text-align: center;
  }

  .toolbar-title {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .word-count {
    font-size: 11px;
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
  }

  .btn-back {
    height: 28px;
    padding: 0 12px;
    border-radius: 6px;
    border: 1px solid var(--border-medium);
    background: transparent;
    color: var(--text-secondary);
    font-size: 12px;
    font-family: system-ui, -apple-system, sans-serif;
    cursor: pointer;
    transition: background 120ms ease, color 120ms ease;
  }

  .btn-back:hover {
    background: var(--surface-hover);
    color: var(--text-primary);
  }

  .story-editor {
    flex: 1;
    overflow-y: auto;
    background: var(--surface-base);
    padding: 40px 0;
  }

  .page {
    max-width: 680px;
    margin: 0 auto;
    min-height: 800px;
    background: var(--page-bg);
    border-radius: 2px;
    box-shadow: 0 4px 24px var(--page-shadow), 0 1px 4px rgba(0, 0, 0, 0.2);
  }

  .narrative-textarea {
    width: 100%;
    height: 100%;
    min-height: 800px;
    padding: 60px 72px;
    font-size: 14px;
    line-height: 1.6;
    color: var(--text-on-page);
    background: transparent;
    border: none;
    font-family: system-ui, -apple-system, sans-serif;
    resize: none;
    box-sizing: border-box;
  }

  .narrative-textarea:focus {
    outline: none;
  }

  .narrative-textarea::placeholder {
    color: var(--text-muted);
    font-style: italic;
  }

  /* ─── Status bar (matches Editor.svelte) ─── */

  .status-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 28px;
    padding: 0 16px;
    background: var(--surface-elevated);
    border-top: 1px solid var(--border-subtle);
    font-size: 11px;
    font-family: system-ui, -apple-system, sans-serif;
    color: var(--text-muted);
    user-select: none;
    flex-shrink: 0;
    letter-spacing: 0.04em;
  }

  .status-left {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .status-mode {
    font-weight: 600;
    text-transform: uppercase;
    color: var(--text-muted);
  }

  .status-mode.malayalam {
    color: var(--accent);
  }

  .status-mode.flash {
    background: var(--accent-muted);
    border-radius: 3px;
    padding: 1px 6px;
    transition: background 0.5s ease-out;
  }

  .status-separator {
    color: var(--border-medium);
  }

  .status-hint {
    color: var(--text-muted);
    font-size: 10px;
  }

  .scheme-selector {
    display: flex;
    gap: 1px;
  }

  .scheme-btn {
    background: none;
    border: none;
    padding: 2px 6px;
    font-size: 11px;
    font-family: system-ui, -apple-system, sans-serif;
    color: var(--text-muted);
    cursor: pointer;
    border-radius: 3px;
    transition: background 100ms, color 100ms;
    letter-spacing: 0.04em;
  }

  .scheme-btn:hover {
    color: var(--text-secondary);
    background: var(--surface-hover);
  }

  .scheme-btn.active {
    color: var(--accent);
    font-weight: 600;
  }
</style>
