use nixcards_core::{Catalog, ReviewRating, bundled_catalog};

const VALID_SET: &str = r#"---
id: cloud.example
title: Example
language: en
license: CC-BY-NC-SA-4.0
attribution: Test
tags: [cloud]
sources: [https://example.com]
---
"#;

const VALID_CARD: &str = r#"# What is it?

An **answer**.
"#;

fn valid_catalog() -> Catalog {
    Catalog::from_sources([
        ("cards/cloud/example/set.md", VALID_SET),
        ("cards/cloud/example/what.md", VALID_CARD),
    ])
    .unwrap()
}

#[test]
fn bundled_catalog_obeys_the_public_contract() {
    let catalog = bundled_catalog().expect("bundled catalogue should validate");
    assert!(!catalog.sets.is_empty());
    assert!(catalog.card_count() >= 30);
    assert!(
        catalog
            .set("cloud.bearingpoint.interview.senior-consultant")
            .is_some()
    );
}

#[test]
fn search_includes_answer_text() {
    let hits = valid_catalog().search("answer");
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].canonical_id, "cloud.example#what");
}

#[test]
fn nested_card_paths_form_dotted_ids() {
    let catalog = Catalog::from_sources([
        ("cards/cloud/example/set.md", VALID_SET),
        (
            "cards/cloud/example/platform/landing-zone.md",
            "# What is a landing zone?\n\nA governed starting point.\n",
        ),
    ])
    .unwrap();
    let card = &catalog.sets[0].cards[0];
    assert_eq!(card.id, "platform.landing-zone");
    assert_eq!(card.canonical_id, "cloud.example#platform.landing-zone");
}

#[test]
fn set_path_must_mirror_the_dotted_id() {
    let error = Catalog::from_sources([
        ("cards/wrong/path/set.md", VALID_SET),
        ("cards/wrong/path/what.md", VALID_CARD),
    ])
    .unwrap_err();
    assert!(error.to_string().contains("requires path"));
}

#[test]
fn duplicate_source_paths_fail_validation() {
    let error = Catalog::from_sources([
        ("cards/cloud/example/set.md", VALID_SET),
        ("cards/cloud/example/what.md", VALID_CARD),
        ("cards/cloud/example/what.md", VALID_CARD),
    ])
    .unwrap_err();
    assert!(
        error
            .to_string()
            .contains("duplicate catalogue source path")
    );
}

#[test]
fn raw_html_fails_validation() {
    let error = Catalog::from_sources([
        ("cards/cloud/example/set.md", VALID_SET),
        (
            "cards/cloud/example/what.md",
            "# What is it?\n\n<script>alert(1)</script>\n",
        ),
    ])
    .unwrap_err();
    assert!(error.to_string().contains("raw HTML"));
}

#[test]
fn a_manifest_without_card_files_fails_validation() {
    let error = Catalog::from_sources([("cards/cloud/example/set.md", VALID_SET)]).unwrap_err();
    assert!(error.to_string().contains("no card files"));
}

#[test]
fn rating_serialization_is_stable() {
    assert_eq!(
        serde_json::to_string(&ReviewRating::Again).unwrap(),
        "\"again\""
    );
    assert_eq!(
        serde_json::to_string(&ReviewRating::Known).unwrap(),
        "\"known\""
    );
}
