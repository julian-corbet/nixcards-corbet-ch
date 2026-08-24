import init, {
  WebCramSession,
  catalog_json,
  progress_summary_json,
  render_answer_html,
  search_json
} from './wasm/nixcards_core.js';
import type { Catalog, ProgressFile, ProgressSummary, SearchHit } from './types';

let initialized: Promise<void> | null = null;

export function initializeCore(): Promise<void> {
  if (!initialized) {
    initialized = init().then(() => undefined);
  }
  return initialized;
}

export async function loadCatalog(): Promise<Catalog> {
  await initializeCore();
  return JSON.parse(catalog_json()) as Catalog;
}

export function searchCatalog(query: string): SearchHit[] {
  return JSON.parse(search_json(query)) as SearchHit[];
}

export function renderAnswer(setId: string, cardId: string): string {
  return render_answer_html(setId, cardId);
}

export function summarizeProgress(progress: ProgressFile): ProgressSummary {
  return JSON.parse(progress_summary_json(JSON.stringify(progress))) as ProgressSummary;
}

export { WebCramSession };

