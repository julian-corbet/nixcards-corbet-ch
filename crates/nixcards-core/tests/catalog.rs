use nixcards_core::{Catalog, ReviewRating, bundled_catalog};

const VALID: &str = r#"---
id: cloud.example
title: Example
language: en
license: CC-BY-NC-SA-4.0
attribution: Test
tags: [cloud]
sources: [https://example.com]
---

## What is it? {#what}

An **answer**.
"#;

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
    let catalog = Catalog::from_sources([("cards/cloud/example/set.md", VALID)]).unwrap();
    let hits = catalog.search("answer");
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].canonical_id, "cloud.example#what");
}

#[test]
fn path_must_mirror_the_dotted_id() {
    let error = Catalog::from_sources([("cards/wrong/path/set.md", VALID)]).unwrap_err();
    assert!(error.to_string().contains("requires path"));
}

#[test]
fn duplicate_card_ids_fail_validation() {
    let duplicate = format!("{VALID}\n## Again {{#what}}\n\nAnother answer.\n");
    let error =
        Catalog::from_sources([("cards/cloud/example/set.md", duplicate.as_str())]).unwrap_err();
    assert!(error.to_string().contains("duplicate card ID"));
}

#[test]
fn raw_html_fails_validation() {
    let unsafe_source = VALID.replace("An **answer**.", "<script>alert(1)</script>");
    let error = Catalog::from_sources([("cards/cloud/example/set.md", unsafe_source.as_str())])
        .unwrap_err();
    assert!(error.to_string().contains("raw HTML"));
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
