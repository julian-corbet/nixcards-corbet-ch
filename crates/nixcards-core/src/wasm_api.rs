use crate::{CramSession, ProgressFile, ReviewRating, bundled_catalog, render_markdown_safe};
use wasm_bindgen::prelude::*;

fn javascript_error(error: impl ToString) -> JsValue {
    JsValue::from_str(&error.to_string())
}

#[wasm_bindgen]
pub fn catalog_json() -> Result<String, JsValue> {
    serde_json::to_string(&bundled_catalog().map_err(javascript_error)?).map_err(javascript_error)
}

#[wasm_bindgen]
pub fn search_json(query: &str) -> Result<String, JsValue> {
    let catalog = bundled_catalog().map_err(javascript_error)?;
    serde_json::to_string(&catalog.search(query)).map_err(javascript_error)
}

#[wasm_bindgen]
pub fn render_answer_html(set_id: &str, card_id: &str) -> Result<String, JsValue> {
    let catalog = bundled_catalog().map_err(javascript_error)?;
    let card = catalog
        .card(set_id, card_id)
        .ok_or_else(|| javascript_error(format!("unknown card {set_id}#{card_id}")))?;
    Ok(render_markdown_safe(&card.answer))
}

#[wasm_bindgen]
pub fn cram_order_json(set_id: &str, seed: u32) -> Result<String, JsValue> {
    let catalog = bundled_catalog().map_err(javascript_error)?;
    let set = catalog
        .set(set_id)
        .ok_or_else(|| javascript_error(format!("unknown set {set_id}")))?;
    let session = crate::CramSession::new(set, seed);
    serde_json::to_string(&session.order()).map_err(javascript_error)
}

#[wasm_bindgen]
pub struct WebCramSession {
    inner: CramSession,
}

#[wasm_bindgen]
impl WebCramSession {
    #[wasm_bindgen(constructor)]
    pub fn new(set_id: &str, seed: u32) -> Result<WebCramSession, JsValue> {
        let catalog = bundled_catalog().map_err(javascript_error)?;
        let set = catalog
            .set(set_id)
            .ok_or_else(|| javascript_error(format!("unknown set {set_id}")))?;
        Ok(Self {
            inner: CramSession::new(set, seed),
        })
    }

    pub fn current(&self) -> Option<String> {
        self.inner.current().map(str::to_owned)
    }

    pub fn answer(&mut self, rating: &str) -> Result<String, JsValue> {
        let rating = match rating {
            "again" => ReviewRating::Again,
            "known" => ReviewRating::Known,
            other => return Err(javascript_error(format!("unknown rating {other}"))),
        };
        self.inner
            .answer(rating)
            .ok_or_else(|| javascript_error("cram session is complete"))
    }

    pub fn remaining(&self) -> usize {
        self.inner.remaining()
    }

    pub fn initial_count(&self) -> usize {
        self.inner.initial_count()
    }

    pub fn reviews(&self) -> usize {
        self.inner.reviews()
    }

    pub fn is_complete(&self) -> bool {
        self.inner.is_complete()
    }
}

#[wasm_bindgen]
pub fn progress_summary_json(progress_json: &str) -> Result<String, JsValue> {
    let progress = ProgressFile::from_json(progress_json).map_err(javascript_error)?;
    serde_json::to_string(&progress.summary()).map_err(javascript_error)
}
