<script lang="ts">
  import { documentStore } from '$lib/stores/documentStore.svelte';

  let { onClose }: { onClose: () => void } = $props();

  /** Parsed scene data with auto-populated and manual fields */
  interface SceneCardData {
    sceneNumber: number;
    heading: string;
    location: string;
    time: string;
    characters: string[];
    pageEstimate: string;
    /** Index into the top-level ProseMirror content array */
    contentIndex: number;
    /** Manually written description */
    description: string;
    /** Manually written shoot notes */
    shootNotes: string;
  }

  /** Parse INT./EXT., location, and time from a scene heading */
  function parseSceneHeading(heading: string): { location: string; time: string } {
    const match = heading.match(/^(?:INT\.|EXT\.|INT\.\/EXT\.)\s*(.+?)\s*-\s*(.+)$/i);
    if (match) {
      return { location: match[1].trim(), time: match[2].trim() };
    }
    return { location: heading, time: '' };
  }

  /** Build scene cards from the ProseMirror content and stored scene_cards data */
  let cards = $derived.by((): SceneCardData[] => {
    const doc = documentStore.document;
    if (!doc || !doc.content) return [];

    const content = doc.content as {
      type?: string;
      content?: Array<{
        type?: string;
        content?: Array<{ text?: string }>;
      }>;
    };
    if (!content.content) return [];

    const result: SceneCardData[] = [];
    let sceneNumber = 0;
    let currentCharacters: string[] = [];
    let currentCharCount = 0;
    let currentHeading = '';
    let currentLocation = '';
    let currentTime = '';
    let currentContentIndex = -1;

    function pushCurrentScene() {
      if (sceneNumber === 0) return;
      const storedCard = doc.scene_cards.find((c) => c.scene_index === sceneNumber - 1);
      const pages = Math.max(0.1, currentCharCount / 3000);
      result.push({
        sceneNumber,
        heading: currentHeading,
        location: currentLocation,
        time: currentTime,
        characters: [...currentCharacters],
        pageEstimate: `~${pages.toFixed(1)} pages`,
        contentIndex: currentContentIndex,
        description: storedCard?.description ?? '',
        shootNotes: storedCard?.shoot_notes ?? '',
      });
    }

    for (let i = 0; i < content.content.length; i++) {
      const node = content.content[i];

      if (node.type === 'scene_heading') {
        pushCurrentScene();
        sceneNumber++;
        currentCharacters = [];
        currentCharCount = 0;
        currentContentIndex = i;

        currentHeading = (node.content ?? []).map((c) => c.text ?? '').join('');
        const parsed = parseSceneHeading(currentHeading);
        currentLocation = parsed.location;
        currentTime = parsed.time;
      } else if (node.type === 'character') {
        const name = (node.content ?? []).map((c) => c.text ?? '').join('').trim();
        if (name && !currentCharacters.includes(name)) {
          currentCharacters.push(name);
        }
      }

      if (node.content) {
        currentCharCount += node.content.reduce((sum, c) => sum + (c.text ?? '').length, 0);
      }
    }

    pushCurrentScene();
    return result;
  });

  /** Update the description for a scene card */
  function updateDescription(sceneIndex: number, value: string) {
    if (!documentStore.document) return;
    const existing = documentStore.document.scene_cards.find((c) => c.scene_index === sceneIndex);
    if (existing) {
      existing.description = value;
    } else {
      documentStore.document.scene_cards.push({
        scene_index: sceneIndex,
        description: value,
        shoot_notes: '',
      });
    }
    documentStore.markDirty();
  }

  /** Update the shoot notes for a scene card */
  function updateShootNotes(sceneIndex: number, value: string) {
    if (!documentStore.document) return;
    const existing = documentStore.document.scene_cards.find((c) => c.scene_index === sceneIndex);
    if (existing) {
      existing.shoot_notes = value;
    } else {
      documentStore.document.scene_cards.push({
        scene_index: sceneIndex,
        description: '',
        shoot_notes: value,
      });
    }
    documentStore.markDirty();
  }
</script>

<div class="scene-cards-view">
  <div class="cards-header">
    <h2>Scene Cards</h2>
    <button class="btn-ghost" onclick={onClose}>Back to Script</button>
  </div>

  {#if cards.length === 0}
    <p class="empty-message">No scenes in the screenplay yet.</p>
  {:else}
    <div class="cards-grid">
      {#each cards as card}
        <div class="card">
          <div class="card-header">
            <span class="card-number">{card.sceneNumber}.</span>
            <span class="card-heading">{card.heading.toUpperCase()}</span>
          </div>
          <div class="card-meta">
            {#if card.location}
              <span class="meta-item"><strong>Location:</strong> {card.location}</span>
            {/if}
            {#if card.time}
              <span class="meta-item"><strong>Time:</strong> {card.time}</span>
            {/if}
            {#if card.characters.length > 0}
              <span class="meta-item"><strong>Characters:</strong> {card.characters.join(', ')}</span>
            {/if}
            <span class="meta-item page-estimate">{card.pageEstimate}</span>
          </div>
          <div class="card-editable">
            <label class="field-label">Description</label>
            <textarea
              class="card-textarea"
              placeholder="What happens in this scene..."
              value={card.description}
              oninput={(e) => updateDescription(card.sceneNumber - 1, (e.target as HTMLTextAreaElement).value)}
              rows="2"
            ></textarea>
            <label class="field-label">Shoot Notes</label>
            <textarea
              class="card-textarea"
              placeholder="Equipment, stunts, VFX, location notes..."
              value={card.shootNotes}
              oninput={(e) => updateShootNotes(card.sceneNumber - 1, (e.target as HTMLTextAreaElement).value)}
              rows="2"
            ></textarea>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .scene-cards-view {
    width: 100%;
    height: 100%;
    overflow-y: auto;
    background: var(--surface-base);
    padding: 24px;
  }

  .cards-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 24px;
  }

  .cards-header h2 {
    font-family: system-ui, -apple-system, sans-serif;
    font-size: 18px;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }

  .btn-ghost {
    height: 28px;
    padding: 0 12px;
    border-radius: 6px;
    border: none;
    background: transparent;
    color: var(--text-secondary);
    font-size: 12px;
    font-family: system-ui, -apple-system, sans-serif;
    cursor: pointer;
    transition: background 120ms ease, color 120ms ease;
  }

  .btn-ghost:hover {
    background: var(--surface-hover);
    color: var(--text-primary);
  }

  .empty-message {
    font-family: system-ui, -apple-system, sans-serif;
    font-size: 13px;
    color: var(--text-muted);
    text-align: center;
    padding: 60px 0;
  }

  .cards-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: 16px;
  }

  .card {
    background: var(--surface-elevated);
    border: 1px solid var(--border-subtle);
    border-radius: 8px;
    overflow: hidden;
  }

  .card-header {
    padding: 12px 16px;
    border-bottom: 1px solid var(--border-subtle);
    display: flex;
    align-items: baseline;
    gap: 6px;
  }

  .card-number {
    font-family: system-ui, -apple-system, sans-serif;
    font-size: 12px;
    font-weight: 700;
    color: var(--accent);
    font-variant-numeric: tabular-nums;
  }

  .card-heading {
    font-family: system-ui, -apple-system, sans-serif;
    font-size: 12px;
    font-weight: 600;
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .card-meta {
    padding: 10px 16px;
    display: flex;
    flex-direction: column;
    gap: 3px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .meta-item {
    font-family: system-ui, -apple-system, sans-serif;
    font-size: 11px;
    color: var(--text-secondary);
  }

  .meta-item strong {
    color: var(--text-muted);
    font-weight: 600;
  }

  .page-estimate {
    color: var(--text-muted);
    font-style: italic;
  }

  .card-editable {
    padding: 10px 16px 14px;
  }

  .field-label {
    display: block;
    font-family: system-ui, -apple-system, sans-serif;
    font-size: 10px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: 4px;
    margin-top: 8px;
  }

  .field-label:first-child {
    margin-top: 0;
  }

  .card-textarea {
    width: 100%;
    padding: 6px 8px;
    font-size: 12px;
    line-height: 1.5;
    color: var(--text-primary);
    background: var(--surface-base);
    border: 1px solid var(--border-subtle);
    border-radius: 4px;
    font-family: system-ui, -apple-system, sans-serif;
    resize: none;
    box-sizing: border-box;
    transition: border-color 120ms ease;
  }

  .card-textarea:focus {
    outline: none;
    border-color: var(--accent);
  }

  .card-textarea::placeholder {
    color: var(--text-muted);
  }
</style>
