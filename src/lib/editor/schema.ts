// ProseMirror screenplay element schema: SceneHeading, Action, Character, Parenthetical, Dialogue, Transition

import { Schema, type NodeSpec } from 'prosemirror-model';

/**
 * Union type representing the six screenplay element types.
 * Used throughout the app to identify which kind of block a node is.
 */
export type ScreenplayNodeType =
	| 'scene_heading'
	| 'action'
	| 'character'
	| 'parenthetical'
	| 'dialogue'
	| 'transition';

/**
 * Node specifications for every node type in the screenplay schema.
 *
 * Each screenplay element is a block-level <p> tag distinguished by a
 * `data-type` attribute so ProseMirror can round-trip them through the DOM.
 * The `text` node is ProseMirror's built-in inline text node.
 */
const nodes: Record<string, NodeSpec> = {
	doc: {
		// The top-level document must contain one or more block nodes.
		content: 'block+'
	},

	scene_heading: {
		group: 'block',
		content: 'text*',
		toDOM() {
			return ['p', { class: 'scene-heading', 'data-type': 'scene_heading' }, 0];
		},
		parseDOM: [{ tag: 'p[data-type="scene_heading"]' }]
	},

	action: {
		group: 'block',
		content: 'text*',
		toDOM() {
			return ['p', { class: 'action', 'data-type': 'action' }, 0];
		},
		parseDOM: [{ tag: 'p[data-type="action"]' }]
	},

	character: {
		group: 'block',
		content: 'text*',
		toDOM() {
			return ['p', { class: 'character', 'data-type': 'character' }, 0];
		},
		parseDOM: [{ tag: 'p[data-type="character"]' }]
	},

	parenthetical: {
		group: 'block',
		content: 'text*',
		toDOM() {
			return ['p', { class: 'parenthetical', 'data-type': 'parenthetical' }, 0];
		},
		parseDOM: [{ tag: 'p[data-type="parenthetical"]' }]
	},

	dialogue: {
		group: 'block',
		content: 'text*',
		toDOM() {
			return ['p', { class: 'dialogue', 'data-type': 'dialogue' }, 0];
		},
		parseDOM: [{ tag: 'p[data-type="dialogue"]' }]
	},

	transition: {
		group: 'block',
		content: 'text*',
		toDOM() {
			return ['p', { class: 'transition', 'data-type': 'transition' }, 0];
		},
		parseDOM: [{ tag: 'p[data-type="transition"]' }]
	},

	text: {
		group: 'inline'
	}
};

/**
 * The screenplay ProseMirror schema.
 * No marks are defined — all text is plain (no bold/italic/underline).
 */
export const screenplaySchema = new Schema({ nodes, marks: {} });
