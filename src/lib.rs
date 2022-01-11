use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[derive(Serialize, Deserialize)]
pub struct Page {
    pub mozabook_version: String,
    #[serde(rename(deserialize = "$value"))]
    pub file_version: Option<Enum>,
}

#[derive(Serialize, Deserialize)]
pub enum Enum {
    A(String),
    B(bool),
}

#[wasm_bindgen]
pub fn parse(s: &str) -> Result<JsValue, JsValue> {
    let s = from_str::<Page>(s).unwrap();
    let js_value = serde_wasm_bindgen::to_value(&s)?;
    Ok(js_value)
}
