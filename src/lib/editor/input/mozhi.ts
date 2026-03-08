// SMC Mozhi transliteration engine integration via Varnam JS

/**
 * Mozhi transliteration result.
 * - output: the Malayalam character(s) to insert, or null if still buffering
 * - buffer: updated buffer state for the next keypress
 */
export interface MozhiResult {
  output: string | null;
  buffer: string;
}

/**
 * Process a keypress through the Mozhi transliteration engine.
 *
 * TODO: Integrate Varnam JS for actual transliteration.
 * For now, this is a passthrough stub.
 */
export function processMozhiKey(key: string, buffer: string): MozhiResult {
  // TODO: Varnam JS integration — for now, pass through
  return { output: null, buffer: '' };
}
