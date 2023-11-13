mod components;
mod todos;
mod state;
mod server_interactiorns;
use std::{fs, path::Path};

use components::app::App;


fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}