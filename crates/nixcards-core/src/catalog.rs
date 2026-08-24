use crate::markdown::contains_raw_html;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Card {
    pub id: String,
    pub canonical_id: String,
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CardSet {
    pub id: String,
    pub title: String,
    pub language: String,
    pub license: String,
    pub attribution: String,
    pub tags: Vec<String>,
    pub sources: Vec<String>,
    pub source_path: String,
    pub cards: Vec<Card>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Catalog {
    pub sets: Vec<CardSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchHit {
    pub set_id: String,
    pub set_title: String,
    pub card_id: String,
    pub canonical_id: String,
    pub question: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogError {
    pub path: String,
    pub message: String,
}

impl CatalogError {
    fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for CatalogError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.path, self.message)
    }
}

impl std::error::Error for CatalogError {}

#[derive(Default)]
struct Metadata {
    id: Option<String>,
    title: Option<String>,
    language: Option<String>,
    license: Option<String>,
    attribution: Option<String>,
    tags: Option<Vec<String>>,
    sources: Option<Vec<String>>,
}

impl Catalog {
    pub fn from_sources<'a>(
        sources: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) -> Result<Self, CatalogError> {
        let mut sets = Vec::new();
        let mut set_ids = HashSet::new();
        let mut canonical_ids = HashSet::new();

        for (path, source) in sources {
            let set = parse_set(path, source)?;
            if !set_ids.insert(set.id.clone()) {
                return Err(CatalogError::new(
                    path,
                    format!("duplicate set ID {}", set.id),
                ));
            }
            for card in &set.cards {
                if !canonical_ids.insert(card.canonical_id.clone()) {
                    return Err(CatalogError::new(
                        path,
                        format!("duplicate canonical card ID {}", card.canonical_id),
                    ));
                }
            }
            sets.push(set);
        }

        sets.sort_by(|left, right| left.id.cmp(&right.id));
        Ok(Self { sets })
    }

    pub fn set(&self, set_id: &str) -> Option<&CardSet> {
        self.sets.iter().find(|set| set.id == set_id)
    }

    pub fn card(&self, set_id: &str, card_id: &str) -> Option<&Card> {
        self.set(set_id)
            .and_then(|set| set.cards.iter().find(|card| card.id == card_id))
    }

    pub fn card_by_canonical_id(&self, canonical_id: &str) -> Option<(&CardSet, &Card)> {
        self.sets.iter().find_map(|set| {
            set.cards
                .iter()
                .find(|card| card.canonical_id == canonical_id)
                .map(|card| (set, card))
        })
    }

    pub fn search(&self, query: &str) -> Vec<SearchHit> {
        let needle = query.trim().to_lowercase();
        let mut hits = Vec::new();

        for set in &self.sets {
            for card in &set.cards {
                let matches = needle.is_empty()
                    || set.id.to_lowercase().contains(&needle)
                    || set.title.to_lowercase().contains(&needle)
                    || set
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&needle))
                    || card.question.to_lowercase().contains(&needle)
                    || card.answer.to_lowercase().contains(&needle);
                if matches {
                    hits.push(SearchHit {
                        set_id: set.id.clone(),
                        set_title: set.title.clone(),
                        card_id: card.id.clone(),
                        canonical_id: card.canonical_id.clone(),
                        question: card.question.clone(),
                    });
                }
            }
        }

        hits
    }

    pub fn card_count(&self) -> usize {
        self.sets.iter().map(|set| set.cards.len()).sum()
    }
}

fn parse_set(path: &str, source: &str) -> Result<CardSet, CatalogError> {
    let normalized = source.replace("\r\n", "\n");
    let body = normalized
        .strip_prefix("---\n")
        .ok_or_else(|| CatalogError::new(path, "missing opening metadata delimiter"))?;
    let (metadata_source, cards_source) = body
        .split_once("\n---\n")
        .ok_or_else(|| CatalogError::new(path, "missing closing metadata delimiter"))?;
    let metadata = parse_metadata(path, metadata_source)?;

    let id = required(path, "id", metadata.id)?;
    validate_dotted_id(path, &id)?;
    validate_path(path, &id)?;

    let title = required(path, "title", metadata.title)?;
    let language = required(path, "language", metadata.language)?;
    let license = required(path, "license", metadata.license)?;
    if license != "CC-BY-NC-SA-4.0" {
        return Err(CatalogError::new(
            path,
            "card sets must use CC-BY-NC-SA-4.0",
        ));
    }
    let attribution = required(path, "attribution", metadata.attribution)?;
    let tags = required(path, "tags", metadata.tags)?;
    let sources = required(path, "sources", metadata.sources)?;
    if tags.is_empty() {
        return Err(CatalogError::new(path, "tags must not be empty"));
    }
    if sources.is_empty() || sources.iter().any(|source| !source.starts_with("https://")) {
        return Err(CatalogError::new(
            path,
            "sources must contain at least one public HTTPS URL",
        ));
    }

    let cards = parse_cards(path, &id, cards_source)?;
    Ok(CardSet {
        id,
        title,
        language,
        license,
        attribution,
        tags,
        sources,
        source_path: path.to_owned(),
        cards,
    })
}

fn parse_metadata(path: &str, source: &str) -> Result<Metadata, CatalogError> {
    let mut metadata = Metadata::default();
    for (index, line) in source.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (key, value) = line.split_once(':').ok_or_else(|| {
            CatalogError::new(path, format!("invalid metadata line {}", index + 2))
        })?;
        let value = value.trim();
        match key.trim() {
            "id" => set_once(path, "id", &mut metadata.id, parse_scalar(value))?,
            "title" => set_once(path, "title", &mut metadata.title, parse_scalar(value))?,
            "language" => set_once(
                path,
                "language",
                &mut metadata.language,
                parse_scalar(value),
            )?,
            "license" => set_once(path, "license", &mut metadata.license, parse_scalar(value))?,
            "attribution" => set_once(
                path,
                "attribution",
                &mut metadata.attribution,
                parse_scalar(value),
            )?,
            "tags" => set_once(path, "tags", &mut metadata.tags, parse_list(path, value)?)?,
            "sources" => set_once(
                path,
                "sources",
                &mut metadata.sources,
                parse_list(path, value)?,
            )?,
            unknown => {
                return Err(CatalogError::new(
                    path,
                    format!("unknown metadata field {unknown}"),
                ));
            }
        }
    }
    Ok(metadata)
}

fn parse_scalar(value: &str) -> String {
    value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .unwrap_or(value)
        .trim()
        .to_owned()
}

fn parse_list(path: &str, value: &str) -> Result<Vec<String>, CatalogError> {
    let inner = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .ok_or_else(|| CatalogError::new(path, "metadata lists must use [item, item] syntax"))?;
    Ok(inner
        .split(',')
        .map(parse_scalar)
        .filter(|item| !item.is_empty())
        .collect())
}

fn set_once<T>(path: &str, key: &str, slot: &mut Option<T>, value: T) -> Result<(), CatalogError> {
    if slot.replace(value).is_some() {
        return Err(CatalogError::new(
            path,
            format!("metadata field {key} occurs more than once"),
        ));
    }
    Ok(())
}

fn required<T>(path: &str, key: &str, value: Option<T>) -> Result<T, CatalogError> {
    value.ok_or_else(|| CatalogError::new(path, format!("missing metadata field {key}")))
}

fn parse_cards(path: &str, set_id: &str, source: &str) -> Result<Vec<Card>, CatalogError> {
    let mut cards = Vec::new();
    let mut card_ids = HashSet::new();
    let mut current: Option<(String, String, Vec<&str>)> = None;

    for line in source.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            if let Some((id, question, answer_lines)) = current.take() {
                cards.push(finish_card(path, set_id, id, question, answer_lines)?);
            }
            let (question, id) = parse_heading(path, heading)?;
            if !card_ids.insert(id.clone()) {
                return Err(CatalogError::new(path, format!("duplicate card ID {id}")));
            }
            current = Some((id, question, Vec::new()));
        } else if let Some((_, _, answer_lines)) = current.as_mut() {
            answer_lines.push(line);
        } else if !line.trim().is_empty() {
            return Err(CatalogError::new(
                path,
                "content before the first level-two card heading",
            ));
        }
    }

    if let Some((id, question, answer_lines)) = current {
        cards.push(finish_card(path, set_id, id, question, answer_lines)?);
    }
    if cards.is_empty() {
        return Err(CatalogError::new(path, "card set contains no cards"));
    }
    Ok(cards)
}

fn parse_heading(path: &str, heading: &str) -> Result<(String, String), CatalogError> {
    let heading = heading.trim();
    let marker = heading
        .strip_suffix('}')
        .and_then(|heading| heading.rsplit_once(" {#"))
        .ok_or_else(|| CatalogError::new(path, "card heading must end with {#stable-card-id}"))?;
    let question = marker.0.trim().to_owned();
    let id = marker.1.trim().to_owned();
    if question.is_empty() {
        return Err(CatalogError::new(path, "card question must not be empty"));
    }
    validate_segment(path, &id, "card ID")?;
    Ok((question, id))
}

fn finish_card(
    path: &str,
    set_id: &str,
    id: String,
    question: String,
    answer_lines: Vec<&str>,
) -> Result<Card, CatalogError> {
    let answer = answer_lines.join("\n").trim().to_owned();
    if answer.is_empty() {
        return Err(CatalogError::new(
            path,
            format!("card {id} has an empty answer"),
        ));
    }
    if contains_raw_html(&answer) {
        return Err(CatalogError::new(
            path,
            format!("card {id} contains raw HTML"),
        ));
    }
    Ok(Card {
        canonical_id: format!("{set_id}#{id}"),
        id,
        question,
        answer,
    })
}

fn validate_dotted_id(path: &str, id: &str) -> Result<(), CatalogError> {
    let segments: Vec<_> = id.split('.').collect();
    if !(2..=5).contains(&segments.len()) {
        return Err(CatalogError::new(
            path,
            "set ID must contain two to five dotted segments",
        ));
    }
    for segment in segments {
        validate_segment(path, segment, "set ID segment")?;
    }
    Ok(())
}

fn validate_segment(path: &str, segment: &str, label: &str) -> Result<(), CatalogError> {
    let valid = !segment.is_empty()
        && segment.len() <= 64
        && !segment.starts_with('-')
        && !segment.ends_with('-')
        && !segment.contains("--")
        && segment.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        });
    if !valid {
        return Err(CatalogError::new(
            path,
            format!("invalid {label} {segment:?}"),
        ));
    }
    Ok(())
}

fn validate_path(path: &str, id: &str) -> Result<(), CatalogError> {
    let expected = format!("cards/{}/set.md", id.replace('.', "/"));
    if path != expected {
        return Err(CatalogError::new(
            path,
            format!("set ID {id} requires path {expected}"),
        ));
    }
    Ok(())
}
