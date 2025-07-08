use graph_generation_language::GGLEngine;
use wasm_bindgen::prelude::*;
use web_sys::window;

// Import the `console.log` function from the `console` module
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Define a macro to make console logging easier
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
pub fn main() {
    // Set up panic hook for better error reporting
    console_error_panic_hook::set_once();

    console_log!("🚀 Graph Generation Language WASM module loaded!");

    // Get the document
    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access the document");

    // Try to find the output element and update it
    if let Some(output_element) = document.get_element_by_id("output") {
        output_element.set_text_content(Some(
            "✅ WASM module loaded successfully! Ready to generate graphs.",
        ));
        output_element.set_class_name("output success");

        // Enable the generate button
        if let Some(button) = document.get_element_by_id("generate-btn") {
            let _ = button.remove_attribute("disabled");
        }
    }

    // Create a global GGL engine instance
    let engine = GGLEngine::new();

    // Store the engine in a global variable for JavaScript access
    let global = js_sys::global();
    let _ = js_sys::Reflect::set(&global, &"gglEngine".into(), &JsValue::from(engine));

    console_log!("🎯 GGL Engine created and ready for use!");
}
