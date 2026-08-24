export interface Card {
  id: string;
  canonical_id: string;
  question: string;
  answer: string;
}

export interface CardSet {
  id: string;
  title: string;
  language: string;
  license: string;
  attribution: string;
  tags: string[];
  sources: string[];
  source_path: string;
  cards: Card[];
}

export interface Catalog {
  sets: CardSet[];
}

export interface SearchHit {
  set_id: string;
  set_title: string;
  card_id: string;
  canonical_id: string;
  question: string;
}

export type ReviewRating = 'again' | 'known';

export interface ReviewEvent {
  card_id: string;
  rating: ReviewRating;
  reviewed_at: number;
}

export interface ProgressFile {
  schema_version: number;
  events: ReviewEvent[];
}

export interface ProgressSummary {
  reviewed_cards: number;
  known_cards: number;
  review_events: number;
  latest: Record<string, ReviewRating>;
}

