<script lang="ts">
  import { documentStore } from '$lib/stores/documentStore.svelte';

  // Props using Svelte 5 $props rune
  let { open = $bindable(false) } = $props<{ open: boolean }>();

  // Local form state — initialized from document meta when modal opens
  let title = $state('');
  let author = $state('');
  let contact = $state('');
  let draftNumber = $state(1);
  let draftDate = $state('');

  // When modal opens, populate form from current document meta
  $effect(() => {
    if (open && documentStore.document) {
      const meta = documentStore.document.meta;
      title = meta.title || '';
      author = meta.author || '';
      contact = meta.contact || '';
      draftNumber = meta.draft_number || 1;
      draftDate = meta.draft_date || '';
    }
  });

  function handleSave() {
    if (documentStore.document) {
      documentStore.document.meta.title = title;
      documentStore.document.meta.author = author;
      documentStore.document.meta.contact = contact;
      documentStore.document.meta.draft_number = draftNumber;
      documentStore.document.meta.draft_date = draftDate;
      documentStore.markDirty();
    }
    open = false;
  }

  function handleCancel() {
    open = false;
  }

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
</script>

{#if open}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div class="modal-backdrop" onclick={handleBackdropClick} onkeydown={handleKeydown} role="dialog" aria-modal="true" tabindex="-1">
    <div class="modal-content">
      <h2>Screenplay Info</h2>

      <div class="form-group">
        <label for="meta-title">Title</label>
        <input id="meta-title" type="text" bind:value={title} placeholder="Untitled Screenplay" />
      </div>

      <div class="form-group">
        <label for="meta-author">Author</label>
        <input id="meta-author" type="text" bind:value={author} placeholder="Writer name" />
      </div>

      <div class="form-group">
        <label for="meta-contact">Contact</label>
        <textarea id="meta-contact" rows="3" bind:value={contact} placeholder="Address / Phone / Email"></textarea>
      </div>

      <div class="form-row">
        <div class="form-group half">
          <label for="meta-draft">Draft #</label>
          <input id="meta-draft" type="number" min="1" bind:value={draftNumber} />
        </div>
        <div class="form-group half">
          <label for="meta-date">Draft Date</label>
          <input id="meta-date" type="date" bind:value={draftDate} />
        </div>
      </div>

      <div class="modal-actions">
        <button class="btn-cancel" onclick={handleCancel}>Cancel</button>
        <button class="btn-save" onclick={handleSave}>Save</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal-content {
    background: #2a2a2a;
    border: 1px solid #444;
    border-radius: 8px;
    padding: 24px;
    width: 400px;
    max-width: 90vw;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
  }

  .modal-content h2 {
    margin: 0 0 20px;
    font-size: 16px;
    color: #e0e0e0;
    font-family: system-ui, sans-serif;
    font-weight: 600;
  }

  .form-group {
    margin-bottom: 14px;
  }

  .form-group label {
    display: block;
    margin-bottom: 4px;
    font-size: 12px;
    color: #999;
    font-family: system-ui, sans-serif;
  }

  .form-group input,
  .form-group textarea {
    width: 100%;
    padding: 8px 10px;
    font-size: 13px;
    color: #e0e0e0;
    background: #1a1a1a;
    border: 1px solid #444;
    border-radius: 4px;
    font-family: system-ui, sans-serif;
    box-sizing: border-box;
  }

  .form-group input:focus,
  .form-group textarea:focus {
    outline: none;
    border-color: #4fc3f7;
  }

  .form-group textarea {
    resize: vertical;
  }

  .form-row {
    display: flex;
    gap: 12px;
  }

  .form-group.half {
    flex: 1;
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    margin-top: 20px;
  }

  .btn-cancel, .btn-save {
    padding: 6px 16px;
    font-size: 12px;
    border-radius: 4px;
    cursor: pointer;
    font-family: system-ui, sans-serif;
    border: 1px solid #444;
  }

  .btn-cancel {
    color: #ccc;
    background: #333;
  }

  .btn-cancel:hover {
    background: #444;
  }

  .btn-save {
    color: #fff;
    background: #2979ff;
    border-color: #2979ff;
  }

  .btn-save:hover {
    background: #448aff;
  }
</style>
