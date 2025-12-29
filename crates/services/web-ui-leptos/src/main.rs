//! WASM entry point for Mouchak Mail Leptos frontend.

use web_ui_leptos::App;

fn main() {
    // Set up better panic messages for WASM debugging
    console_error_panic_hook::set_once();

    // Mount the Leptos app to the document body
    leptos::mount::mount_to_body(App);
}
