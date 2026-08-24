mod catalog;
mod markdown;
mod progress;
mod session;

#[cfg(feature = "wasm")]
mod wasm_api;

pub use catalog::{
    CATALOG_INDEX_SCHEMA_VERSION, Card, CardSet, Catalog, CatalogError, CatalogIndex,
    CatalogIndexSet, SearchHit,
};
pub use markdown::{markdown_to_plain_text, render_markdown_safe};
pub use progress::{ProgressFile, ProgressSummary, ReviewEvent, ReviewRating};
pub use session::CramSession;

include!(concat!(env!("OUT_DIR"), "/bundled_cards.rs"));

pub fn bundled_catalog() -> Result<Catalog, CatalogError> {
    Catalog::from_sources(BUNDLED_CARD_SOURCES.iter().copied())
}
