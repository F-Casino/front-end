pub mod app;
pub mod phantom;
pub mod component;
pub mod config;

use leptos::*;
use tracing_subscriber::prelude::*;

use app::App;

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    console_error_panic_hook::set_once();

    mount_to_body(App);
}
