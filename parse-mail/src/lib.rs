use std::collections::HashMap;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub fn parse(data: String) -> Option<HashMap<String, String>> {
    let mut result: HashMap<String, String> = HashMap::new();

    let mut current_field_name: Option<&str> = None;

    for line in data.lines() {
        let line = line.trim();

        if line.trim().ends_with(':') {
            current_field_name = Some(line.get(..line.len() - 1)?);
            result.insert(current_field_name?.to_string(), "".to_string());
        } else if let Some(field_name) = current_field_name {
            if let Some(field) = result.get_mut(field_name) {
                *field = format!("{}\n{}", field, line).trim().to_string();
            }
        }
    }

    Some(result)
}

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(target_arch = "wasm32")]
use js_sys::Array;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn parse_mail() -> i32 {
    // vec![String::from("ok"), String::from("ok23")]
    809
}

#[cfg(target_arch = "wasm32")]
mod utils;
