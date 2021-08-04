use wasm_bindgen::prelude::*;

mod dependencies;
mod provider;

#[wasm_bindgen]
pub fn add_two_ints(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub async fn get_account_data(address_str: String) -> Result<JsValue, JsValue> {
    let provider = dependencies::provider(dependencies::algod());

    let address = address_str.parse()?;

    let res = provider.get_infos(&address).await;

    match res {
        Ok(view_data) => {
            Ok(JsValue::from_serde(&serde_json::to_value(&view_data).unwrap()).unwrap())
        }
        Err(e) => Err(JsValue::from(format!("{}", e))),
    }
}
