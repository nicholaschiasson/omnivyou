#![recursion_limit = "1024"]

mod app;
mod components;
mod routes;

use wasm_bindgen::prelude::*;

use app::App;

fn main() -> Result<(), JsValue> {
	wasm_logger::init(wasm_logger::Config::default());
	yew::start_app::<App>();
	Ok(())
}
