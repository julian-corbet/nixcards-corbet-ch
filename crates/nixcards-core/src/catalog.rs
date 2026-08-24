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
    pub source_path: String,
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

struct SetSource {
    directory: String,
    set: CardSet,
}

impl Catalog {
    pub fn from_sources<'a>(
        sources: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) -> Result<Self, CatalogError> {
        let mut files = Vec::new();
        let mut paths = HashSet::new();
        for (path, source) in sources {
            if !paths.insert(path) {
                return Err(CatalogError::new(path, "duplicate catalogue source path"));
            }
            files.push((path, source));
        }
        files.sort_by_key(|(path, _)| *path);

        let mut set_ids = HashSet::new();
        let mut set_sources = Vec::new();
        for (path, source) in &files {
            if !path.ends_with("/set.md") {
                continue;
            }
            let set = parse_set_manifest(path, source)?;
            if !set_ids.insert(set.id.clone()) {
                return Err(CatalogError::new(
                    *path,
                    format!("duplicate set ID {}", set.id),
                ));
            }
            set_sources.push(SetSource {
                directory: path.trim_end_matches("/set.md").to_owned(),
                set,
            });
        }

        for left in &set_sources {
            for right in &set_sources {
                if left.directory != right.directory
                    && right.directory.starts_with(&format!("{}/", left.directory))
                {
                    return Err(CatalogError::new(
                        &right.set.source_path,
                        format!("card sets must not be nested inside {}", left.set.id),
                    ));
                }
            }
        }

        let mut canonical_ids = HashSet::new();
        for (path, source) in files {
            if path.ends_with("/set.md") {
                continue;
            }
            let owners: Vec<_> = set_sources
                .iter()
                .enumerate()
                .filter(|(_, candidate)| path.starts_with(&format!("{}/", candidate.directory)))
                .map(|(index, _)| index)
                .collect();
            let [owner_index] = owners.as_slice() else {
                return Err(CatalogError::new(
                    path,
                    "card file is not owned by exactly one set",
                ));
            };
            let owner = &mut set_sources[*owner_index];
            let card = parse_card(path, source, &owner.directory, &owner.set.id)?;
            if !canonical_ids.insert(card.canonical_id.clone()) {
                return Err(CatalogError::new(
                    path,
                    format!("duplicate canonical card ID {}", card.canonical_id),
                ));
            }
            owner.set.cards.push(card);
        }

        let mut sets = Vec::new();
        for mut source in set_sources {
            if source.set.cards.is_empty() {
                return Err(CatalogError::new(
                    &source.set.source_path,
                    "card set contains no card files",
                ));
            }
            source
                .set
                .cards
                .sort_by(|left, right| left.id.cmp(&right.id));
            sets.push(source.set);
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

fn parse_set_manifest(path: &str, source: &str) -> Result<CardSet, CatalogError> {
    let (metadata_source, overview) = split_frontmatter(path, source)?;
    let metadata = parse_metadata(path, metadata_source)?;

    let id = required(path, "id", metadata.id)?;
    validate_dotted_id(path, &id, "set ID", 2, 5)?;
    validate_set_path(path, &id)?;

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
    if contains_raw_html(overview) {
        return Err(CatalogError::new(path, "set overview contains raw HTML"));
    }

    Ok(CardSet {
        id,
        title,
        language,
        license,
        attribution,
        tags,
        sources,
        source_path: path.to_owned(),
        cards: Vec::new(),
    })
}

fn split_frontmatter<'a>(path: &str, source: &'a str) -> Result<(&'a str, &'a str), CatalogError> {
    let body = source
        .strip_prefix("---\n")
        .ok_or_else(|| CatalogError::new(path, "missing opening metadata delimiter"))?;
    body.split_once("\n---\n").ok_or_else(|| {
        CatalogError::new(
            path,
            "missing closing metadata delimiter or non-Unix line endings",
        )
    })
}

fn parse_card(
    path: &str,
    source: &str,
    set_directory: &str,
    set_id: &str,
) -> Result<Card, CatalogError> {
    let id = card_id_from_path(path, set_directory)?;
    let normalized = source.replace("\r\n", "\n");
    let (heading, answer) = normalized
        .split_once('\n')
        .ok_or_else(|| CatalogError::new(path, "card must contain a question and answer"))?;
    let question = heading
        .strip_prefix("# ")
        .map(str::trim)
        .filter(|question| !question.is_empty())
        .ok_or_else(|| CatalogError::new(path, "card must start with one level-one question"))?;
    let answer = answer.trim();
    if answer.is_empty() {
        return Err(CatalogError::new(
            path,
            format!("card {id} has an empty answer"),
        ));
    }
    if contains_raw_html(answer) {
        return Err(CatalogError::new(
            path,
            format!("card {id} contains raw HTML"),
        ));
    }
    Ok(Card {
        canonical_id: format!("{set_id}#{id}"),
        id,
        question: question.to_owned(),
        answer: answer.to_owned(),
        source_path: path.to_owned(),
    })
}

fn card_id_from_path(path: &str, set_directory: &str) -> Result<String, CatalogError> {
    let relative = path
        .strip_prefix(&format!("{set_directory}/"))
        .and_then(|relative| relative.strip_suffix(".md"))
        .ok_or_else(|| CatalogError::new(path, "card path must end in .md inside its set"))?;
    let id = relative.replace('/', ".");
    validate_dotted_id(path, &id, "card ID", 1, 5)?;
    Ok(id)
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

fn validate_dotted_id(
    path: &str,
    id: &str,
    label: &str,
    min_segments: usize,
    max_segments: usize,
) -> Result<(), CatalogError> {
    let segments: Vec<_> = id.split('.').collect();
    if !(min_segments..=max_segments).contains(&segments.len()) {
        return Err(CatalogError::new(
            path,
            format!("{label} must contain {min_segments} to {max_segments} dotted segments"),
        ));
    }
    for segment in segments {
        validate_segment(path, segment, &format!("{label} segment"))?;
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

fn validate_set_path(path: &str, id: &str) -> Result<(), CatalogError> {
    let expected = format!("cards/{}/set.md", id.replace('.', "/"));
    if path != expected {
        return Err(CatalogError::new(
            path,
            format!("set ID {id} requires path {expected}"),
        ));
    }
    Ok(())
}
