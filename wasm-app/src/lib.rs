// wasm-client/src/lib.rs
use wasm_bindgen::prelude::*;
use web_sys::console;
use rand::Rng;

// main function of WASM code
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console::log_1(&"WASM Module initialized!".into());
    Ok(())
}

#[wasm_bindgen]
pub fn increment_counter() {
    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("no document on window");
    
    let count_element = document
        .get_element_by_id("count")
        .expect("should have #count on the page");
    
    let current_count: i32 = count_element
        .text_content()
        .expect("should have text content")
        .parse()
        .unwrap_or(0);
    
    let new_count = current_count + 1;
    
    // update the text
    count_element.set_text_content(Some(&new_count.to_string()));
}

#[wasm_bindgen]
pub fn change_background() {
    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("no document on window");
    let body = document.body().expect("no body exists");

    let colors: [&str; 5] = ["red", "green", "blue", "yellow", "white"];
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..colors.len());
    //println!("Integer: {}", num);
    
    // update the background-color
    body.style()
        .set_property("background-color", colors[num])
        .expect("failed to set background color");
}
