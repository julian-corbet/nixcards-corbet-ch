import type { ProgressFile } from './types';

const DATABASE = 'nixcards';
const VERSION = 1;
const STORE = 'state';
const KEY = 'progress';

export function emptyProgress(): ProgressFile {
  return { schema_version: 1, events: [] };
}

function openDatabase(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open(DATABASE, VERSION);
    request.onupgradeneeded = () => {
      if (!request.result.objectStoreNames.contains(STORE)) {
        request.result.createObjectStore(STORE);
      }
    };
    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error ?? new Error('Could not open IndexedDB'));
  });
}

export async function loadProgress(): Promise<ProgressFile> {
  const database = await openDatabase();
  return new Promise((resolve, reject) => {
    const transaction = database.transaction(STORE, 'readonly');
    const request = transaction.objectStore(STORE).get(KEY);
    request.onsuccess = () => resolve((request.result as ProgressFile | undefined) ?? emptyProgress());
    request.onerror = () => reject(request.error ?? new Error('Could not read progress'));
    transaction.oncomplete = () => database.close();
  });
}

export async function saveProgress(progress: ProgressFile): Promise<void> {
  const database = await openDatabase();
  await new Promise<void>((resolve, reject) => {
    const transaction = database.transaction(STORE, 'readwrite');
    transaction.objectStore(STORE).put(progress, KEY);
    transaction.oncomplete = () => resolve();
    transaction.onerror = () => reject(transaction.error ?? new Error('Could not save progress'));
  });
  database.close();
}

export function downloadProgress(progress: ProgressFile): void {
  const blob = new Blob([JSON.stringify(progress, null, 2)], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  const link = document.createElement('a');
  link.href = url;
  link.download = `nixcards-progress-${new Date().toISOString().slice(0, 10)}.json`;
  link.click();
  URL.revokeObjectURL(url);
}

export async function readProgressFile(file: File): Promise<ProgressFile> {
  const parsed = JSON.parse(await file.text()) as unknown;
  if (!parsed || typeof parsed !== 'object') {
    throw new Error('The progress file is not an object.');
  }
  const candidate = parsed as Partial<ProgressFile>;
  if (candidate.schema_version !== 1 || !Array.isArray(candidate.events)) {
    throw new Error('Unsupported nixcards progress file.');
  }
  return candidate as ProgressFile;
}

