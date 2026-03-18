<script lang="ts">
  import { documentStore } from '$lib/stores/documentStore.svelte';
  import { themeStore } from '$lib/stores/themeStore.svelte';
  import { InputModeManager } from '$lib/editor/input/InputModeManager';

  let { open = $bindable(false) } = $props<{ open: boolean }>();

  const inputManager = InputModeManager.getInstance();

  // We sync local state when the modal opens, just in case
  // it was changed via keyboard shortcuts (like Ctrl+Space).
  let currentMode = $state<'ENGLISH' | 'MALAYALAM'>(inputManager.isMalayalam ? 'MALAYALAM' : 'ENGLISH');
  let currentScheme = $state<'inscript1' | 'inscript2' | 'mozhi'>(inputManager.scheme);

  $effect(() => {
    if (open) {
      currentMode = inputManager.isMalayalam ? 'MALAYALAM' : 'ENGLISH';
      currentScheme = inputManager.scheme;
    }
  });

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      open = false;
    }
  }

  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget) {
      open = false;
    }
  }

  function setLanguageMode(mode: 'ENGLISH' | 'MALAYALAM') {
    if (mode === 'MALAYALAM' && !inputManager.isMalayalam) {
      inputManager.toggle();
    } else if (mode === 'ENGLISH' && inputManager.isMalayalam) {
      inputManager.toggle();
    }
    currentMode = mode;
  }

  function setScheme(scheme: 'inscript1' | 'inscript2' | 'mozhi') {
    inputManager.setScheme(scheme);
    currentScheme = scheme;
  }
</script>

{#if open}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div class="modal-backdrop" onclick={handleBackdropClick} onkeydown={handleKeydown} role="dialog" aria-modal="true" tabindex="-1">
    <div class="modal-card">
      <div class="modal-header">
        <h2>Settings</h2>
        <button class="btn-close" onclick={() => { open = false; }}>&times;</button>
      </div>

      <div class="settings-section">
        <div class="section-label">Appearance</div>
        
        <div class="setting-row">
          <span class="setting-name">Theme</span>
          <div class="segmented">
            <button
              class="segmented-item"
              class:active={!themeStore.isDark}
              onclick={() => { if (themeStore.isDark) themeStore.toggle(); }}
            >Light</button>
            <button
              class="segmented-item"
              class:active={themeStore.isDark}
              onclick={() => { if (!themeStore.isDark) themeStore.toggle(); }}
            >Dark</button>
          </div>
        </div>

        <div class="setting-row">
          <span class="setting-name">Editor Font</span>
          <div class="segmented">
            <button
              class="segmented-item"
              class:active={documentStore.currentFont === 'noto-sans-malayalam'}
              onclick={() => documentStore.setFont('noto-sans-malayalam')}
            >Noto</button>
            <button
              class="segmented-item"
              class:active={documentStore.currentFont === 'manjari'}
              onclick={() => documentStore.setFont('manjari')}
            >Manjari</button>
          </div>
        </div>
      </div>

      <div class="settings-section">
        <div class="section-label">Typing & Language</div>
        
        <div class="setting-row">
          <div class="setting-name-group">
            <span class="setting-name">Input Mode</span>
            <span class="setting-hint">Ctrl+Space to toggle</span>
          </div>
          <div class="segmented">
            <button
              class="segmented-item"
              class:active={currentMode === 'ENGLISH'}
              onclick={() => setLanguageMode('ENGLISH')}
            >English</button>
            <button
              class="segmented-item"
              class:active={currentMode === 'MALAYALAM'}
              onclick={() => setLanguageMode('MALAYALAM')}
            >Malayalam</button>
          </div>
        </div>

        {#if currentMode === 'MALAYALAM'}
          <div class="setting-row nested">
            <span class="setting-name">Keyboard Scheme</span>
            <div class="segmented">
              <button
                class="segmented-item"
                class:active={currentScheme === 'mozhi'}
                onclick={() => setScheme('mozhi')}
              >Mozhi</button>
              <button
                class="segmented-item"
                class:active={currentScheme === 'inscript2'}
                onclick={() => setScheme('inscript2')}
              >Inscript 2</button>
              <button
                class="segmented-item"
                class:active={currentScheme === 'inscript1'}
                onclick={() => setScheme('inscript1')}
              >Inscript 1</button>
            </div>
          </div>
        {/if}
      </div>

    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal-card {
    background: var(--surface-float);
    border: 1px solid var(--border-medium);
    border-radius: 12px;
    padding: 24px;
    width: 480px;
    max-width: 90vw;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
    animation: modal-in 150ms ease-out;
    font-family: system-ui, -apple-system, sans-serif;
  }

  @keyframes modal-in {
    from { opacity: 0; transform: scale(0.97); }
    to { opacity: 1; transform: scale(1); }
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 24px;
  }

  .modal-header h2 {
    margin: 0;
    font-size: 15px;
    color: var(--text-primary);
    font-weight: 600;
  }

  .btn-close {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    border-radius: 6px;
    background: transparent;
    color: var(--text-muted);
    font-size: 18px;
    cursor: pointer;
    transition: background 120ms ease, color 120ms ease;
  }

  .btn-close:hover {
    background: var(--surface-hover);
    color: var(--text-primary);
  }

  .settings-section {
    margin-bottom: 24px;
  }
  
  .settings-section:last-child {
    margin-bottom: 0;
  }

  .section-label {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: 12px;
    border-bottom: 1px solid var(--border-subtle);
    padding-bottom: 6px;
  }

  .setting-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 0;
  }

  .setting-row.nested {
    padding-top: 4px;
    padding-bottom: 10px;
  }

  .setting-name-group {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .setting-name {
    font-size: 13px;
    color: var(--text-primary);
  }

  .setting-hint {
    font-size: 11px;
    color: var(--text-muted);
  }

  .segmented {
    display: flex;
    background: var(--surface-base);
    border-radius: 6px;
    padding: 2px;
    gap: 1px;
    border: 1px solid var(--border-subtle);
  }

  .segmented-item {
    padding: 4px 12px;
    border-radius: 4px;
    border: none;
    font-size: 12px;
    font-family: system-ui, -apple-system, sans-serif;
    color: var(--text-muted);
    background: transparent;
    cursor: pointer;
    transition: background 100ms, color 100ms;
  }

  .segmented-item:hover {
    color: var(--text-secondary);
  }

  .segmented-item.active {
    background: var(--surface-elevated);
    color: var(--text-primary);
    box-shadow: 0 1px 2px rgba(0,0,0,0.1);
  }
</style>
