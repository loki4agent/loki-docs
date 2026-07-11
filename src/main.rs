use leptos::prelude::*;
use loki_docs::App;

fn main() {
    // Set panic hook to output Rust errors to browser console log
    console_error_panic_hook::set_once();

    // Mount Leptos Root App into standard SPA DOM body container
    mount_to_body(App);
}
