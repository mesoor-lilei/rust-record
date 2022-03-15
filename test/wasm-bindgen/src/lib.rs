use anyhow::Result;
use serde_json::{from_str, to_string, to_string_pretty, Value};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn expand(s: &str) -> String {
    expand_json(s).unwrap()
}

#[wasm_bindgen]
pub fn collapse(s: &str) -> String {
    collapse_json(s).unwrap()
}

fn expand_json(s: &str) -> Result<String> {
    Ok(to_string_pretty(&from_str::<Value>(s)?)?)
}

fn collapse_json(s: &str) -> Result<String> {
    Ok(to_string(&from_str::<Value>(s)?)?)
}

#[wasm_bindgen(start)]
pub fn start() {
    log("Wasm");
}

#[test]
fn test() -> Result<()> {
    const COLLAPSE: &str = r#"{"key":"value"}"#;
    const EXPAND: &str = r#"{
  "key": "value"
}"#;
    assert_eq!(expand_json(COLLAPSE)?, EXPAND);
    assert_eq!(collapse_json(EXPAND)?, COLLAPSE);
    Ok(())
}
