use crate::{CardSet, ReviewRating};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct CramSession {
    queue: VecDeque<String>,
    initial_count: usize,
    reviews: usize,
    known: usize,
}

impl CramSession {
    pub fn new(set: &CardSet, seed: u32) -> Self {
        let mut cards: Vec<_> = set
            .cards
            .iter()
            .map(|card| card.canonical_id.clone())
            .collect();
        shuffle(&mut cards, seed);
        let initial_count = cards.len();
        Self {
            queue: cards.into(),
            initial_count,
            reviews: 0,
            known: 0,
        }
    }

    pub fn current(&self) -> Option<&str> {
        self.queue.front().map(String::as_str)
    }

    pub fn answer(&mut self, rating: ReviewRating) -> Option<String> {
        let card = self.queue.pop_front()?;
        self.reviews += 1;
        match rating {
            ReviewRating::Again => self.queue.push_back(card.clone()),
            ReviewRating::Known => self.known += 1,
        }
        Some(card)
    }

    pub fn remaining(&self) -> usize {
        self.queue.len()
    }

    pub fn initial_count(&self) -> usize {
        self.initial_count
    }

    pub fn reviews(&self) -> usize {
        self.reviews
    }

    pub fn known(&self) -> usize {
        self.known
    }

    pub fn is_complete(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn order(&self) -> Vec<String> {
        self.queue.iter().cloned().collect()
    }
}

fn shuffle<T>(items: &mut [T], seed: u32) {
    let mut state = if seed == 0 { 0x9e37_79b9 } else { seed };
    for index in (1..items.len()).rev() {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        let target = state as usize % (index + 1);
        items.swap(index, target);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Card;

    fn set() -> CardSet {
        CardSet {
            id: "test.set".into(),
            title: "Test".into(),
            language: "en".into(),
            license: "CC-BY-NC-SA-4.0".into(),
            attribution: "Test".into(),
            tags: vec!["test".into()],
            sources: vec!["https://example.com".into()],
            source_path: "cards/test/set/set.md".into(),
            cards: ["one", "two"]
                .into_iter()
                .map(|id| Card {
                    id: id.into(),
                    canonical_id: format!("test.set#{id}"),
                    question: id.into(),
                    answer: id.into(),
                    source_path: format!("cards/test/set/{id}.md"),
                })
                .collect(),
        }
    }

    #[test]
    fn missed_cards_return_until_known() {
        let mut session = CramSession::new(&set(), 1);
        let first = session.current().unwrap().to_owned();
        session.answer(ReviewRating::Again);
        assert!(session.order().contains(&first));
        while !session.is_complete() {
            session.answer(ReviewRating::Known);
        }
        assert_eq!(session.known(), 2);
        assert_eq!(session.reviews(), 3);
    }
}
