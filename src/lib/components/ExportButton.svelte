<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { save } from '@tauri-apps/plugin-dialog';
  import { writeFile } from '@tauri-apps/plugin-fs';
  import { documentStore } from '$lib/stores/documentStore.svelte';

  let statusMessage = $state('');
  let statusTimeout: ReturnType<typeof setTimeout> | null = null;

  function showStatus(message: string) {
    statusMessage = message;
    if (statusTimeout) clearTimeout(statusTimeout);
    statusTimeout = setTimeout(() => { statusMessage = ''; }, 3000);
  }

  async function handleExportPdf() {
    if (!documentStore.document) {
      showStatus('No document to export');
      return;
    }

    try {
      showStatus('Generating PDF...');

      // Call Rust backend to compile Typst markup to PDF bytes
      const pdfBytes = await invoke<number[]>('export_pdf', {
        document: documentStore.document
      });

      // Ask user where to save the PDF
      const path = await save({
        filters: [{ name: 'PDF', extensions: ['pdf'] }]
      });

      if (!path) {
        showStatus(''); // User cancelled
        return;
      }

      // Write PDF bytes to disk
      await writeFile(path, new Uint8Array(pdfBytes));
      showStatus('PDF exported');
    } catch (error) {
      console.error('PDF export failed:', error);
      showStatus('Export failed');
    }
  }
</script>

<div class="export-group">
  <button class="export-button" onclick={handleExportPdf}>Export PDF</button>
  {#if statusMessage}
    <span class="status-message">{statusMessage}</span>
  {/if}
</div>

<style>
  .export-group {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .export-button {
    padding: 4px 12px;
    font-size: 12px;
    color: #ccc;
    background: #333;
    border: 1px solid #444;
    border-radius: 4px;
    cursor: pointer;
  }

  .export-button:hover {
    background: #444;
    color: #fff;
  }

  .status-message {
    font-size: 11px;
    color: #888;
    font-family: system-ui, sans-serif;
  }
</style>
