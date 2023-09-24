#![warn(missing_docs)]

//! The aim of this crate is to generate 4-voiced SATB-sheets from predefined accords.
//! Especially, the original python logic was translated to rust and is used to create a web application.

mod app;
use app::App;
/// Contains the underlying logic, note calculation, chorsatz generation, etc.
mod logic;

fn main() {
    leptos::mount_to_body(|| leptos::view! {<App/>})
}
