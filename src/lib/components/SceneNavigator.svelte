<script lang="ts">
  import { TextSelection } from 'prosemirror-state';
  import { documentStore } from '$lib/stores/documentStore.svelte';
  import { editorStore } from '$lib/stores/editorStore.svelte';

  // Props: whether the panel is open
  let { isOpen }: { isOpen: boolean } = $props();

  // Scene heading extracted from ProseMirror JSON content
  interface SceneEntry {
    number: number;
    text: string;
    // Index of this scene_heading in the top-level content array
    index: number;
  }

  // Extract scene headings from the ProseMirror JSON document.
  // documentStore.document.content is the ProseMirror doc JSON.
  // We look for nodes with type === 'scene_heading' in doc.content array.
  let scenes = $derived.by(() => {
    const doc = documentStore.document;
    if (!doc || !doc.content) return [];

    // ProseMirror JSON structure: { type: "doc", content: [ { type: "scene_heading", content: [...] }, ... ] }
    const content = doc.content as { type?: string; content?: Array<{ type?: string; content?: Array<{ text?: string }> }> };
    if (!content.content) return [];

    const entries: SceneEntry[] = [];
    let sceneNumber = 0;

    content.content.forEach((node, index) => {
      if (node.type === 'scene_heading') {
        sceneNumber++;
        // Extract text from the node's content array
        let text = '';
        if (node.content) {
          text = node.content
            .map((child) => child.text ?? '')
            .join('');
        }
        entries.push({
          number: sceneNumber,
          text: text || '(empty)',
          index,
        });
      }
    });

    return entries;
  });

  // Navigate to a scene heading in the editor
  function scrollToScene(sceneIndex: number) {
    const view = editorStore.view;
    if (!view) return;

    const doc = view.state.doc;
    let targetPos = -1;

    // Walk through the document's top-level children to find the node
    // at the given index
    doc.forEach((node, offset, index) => {
      if (index === sceneIndex) {
        // Position inside the node (offset is the position before the node,
        // +1 gets inside it)
        targetPos = offset + 1;
      }
    });

    if (targetPos === -1) return;

    // Move the cursor to the scene heading and scroll it into view
    const tr = view.state.tr.setSelection(
      TextSelection.create(view.state.doc, targetPos)
    );
    tr.scrollIntoView();
    view.dispatch(tr);
    view.focus();
  }
</script>

<aside class="scene-navigator" class:open={isOpen}>
  <div class="navigator-content">
    <h3 class="navigator-title">Scenes</h3>
    {#if scenes.length === 0}
      <p class="empty-message">No scenes yet</p>
    {:else}
      <ul class="scene-list">
        {#each scenes as scene}
          <li>
            <button
              class="scene-item"
              onclick={() => scrollToScene(scene.index)}
            >
              <span class="scene-number">{scene.number}.</span>
              <span class="scene-text">{scene.text.toUpperCase()}</span>
            </button>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
</aside>

<style>
  .scene-navigator {
    width: 0;
    min-width: 0;
    overflow: hidden;
    background: var(--surface-base);
    border-right: 1px solid var(--border-subtle);
    transition: width 200ms cubic-bezier(0.4, 0, 0.2, 1),
                min-width 200ms cubic-bezier(0.4, 0, 0.2, 1);
    flex-shrink: 0;
  }

  .scene-navigator.open {
    width: 220px;
    min-width: 220px;
  }

  .navigator-content {
    width: 220px;
    padding: 12px;
    overflow-y: auto;
    height: 100%;
  }

  .navigator-title {
    font-family: system-ui, -apple-system, sans-serif;
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin: 0 0 12px 0;
    padding: 0 4px;
  }

  .empty-message {
    font-family: system-ui, -apple-system, sans-serif;
    font-size: 12px;
    color: var(--text-muted);
    padding: 0 4px;
  }

  .scene-list {
    list-style: none;
    margin: 0;
    padding: 0;
  }

  .scene-item {
    display: flex;
    align-items: baseline;
    gap: 6px;
    width: 100%;
    height: 32px;
    padding: 0 12px;
    border: none;
    border-left: 2px solid transparent;
    border-radius: 0 4px 4px 0;
    background: transparent;
    color: var(--text-secondary);
    font-family: system-ui, -apple-system, sans-serif;
    font-size: 12px;
    text-align: left;
    cursor: pointer;
    line-height: 32px;
    transition: background 120ms ease, color 120ms ease;
  }

  .scene-item:hover {
    background: var(--surface-hover);
    color: var(--text-primary);
  }

  .scene-item:active {
    background: var(--accent-muted);
    border-left-color: var(--accent);
    color: var(--text-primary);
  }

  .scene-number {
    color: var(--text-muted);
    flex-shrink: 0;
    font-variant-numeric: tabular-nums;
    font-size: 11px;
  }

  .scene-text {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
