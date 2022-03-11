use anyhow::Result;
use serde_json::{from_str, to_string_pretty, Value};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn format(s: &str) -> JsValue {
    JsValue::from(pretty(s).unwrap())
}

fn pretty(s: &str) -> Result<String> {
    Ok(to_string_pretty(&from_str::<Value>(s)?)?)
}

#[wasm_bindgen(start)]
pub fn start() {
    log("Wasm");
}

#[test]
fn test() -> Result<()> {
    assert_eq!(pretty(r#"{"key":"value"}"#)?, r#"{
  "key": "value"
}"#);
    Ok(())
}
