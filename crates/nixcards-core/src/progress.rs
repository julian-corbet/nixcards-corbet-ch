use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const PROGRESS_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReviewRating {
    Again,
    Known,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReviewEvent {
    pub card_id: String,
    pub rating: ReviewRating,
    pub reviewed_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProgressFile {
    pub schema_version: u32,
    pub events: Vec<ReviewEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProgressSummary {
    pub reviewed_cards: usize,
    pub known_cards: usize,
    pub review_events: usize,
    pub latest: HashMap<String, ReviewRating>,
}

impl Default for ProgressFile {
    fn default() -> Self {
        Self {
            schema_version: PROGRESS_SCHEMA_VERSION,
            events: Vec::new(),
        }
    }
}

impl ProgressFile {
    pub fn from_json(json: &str) -> Result<Self, String> {
        let progress: Self = serde_json::from_str(json).map_err(|error| error.to_string())?;
        if progress.schema_version != PROGRESS_SCHEMA_VERSION {
            return Err(format!(
                "unsupported progress schema {}; expected {}",
                progress.schema_version, PROGRESS_SCHEMA_VERSION
            ));
        }
        Ok(progress)
    }

    pub fn to_json_pretty(&self) -> Result<String, String> {
        serde_json::to_string_pretty(self).map_err(|error| error.to_string())
    }

    pub fn record(&mut self, card_id: impl Into<String>, rating: ReviewRating, reviewed_at: u64) {
        self.events.push(ReviewEvent {
            card_id: card_id.into(),
            rating,
            reviewed_at,
        });
    }

    pub fn summary(&self) -> ProgressSummary {
        let mut latest = HashMap::new();
        for event in &self.events {
            latest.insert(event.card_id.clone(), event.rating);
        }
        ProgressSummary {
            reviewed_cards: latest.len(),
            known_cards: latest
                .values()
                .filter(|rating| **rating == ReviewRating::Known)
                .count(),
            review_events: self.events.len(),
            latest,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summary_uses_the_latest_review_per_card() {
        let mut progress = ProgressFile::default();
        progress.record("set#one", ReviewRating::Known, 1);
        progress.record("set#one", ReviewRating::Again, 2);
        progress.record("set#two", ReviewRating::Known, 3);
        let summary = progress.summary();
        assert_eq!(summary.reviewed_cards, 2);
        assert_eq!(summary.known_cards, 1);
        assert_eq!(summary.review_events, 3);
    }
}
