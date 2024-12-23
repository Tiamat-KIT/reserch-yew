use gloo::utils::document;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use yew::platform::spawn_local;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    // yew::Renderer::<Roots>::new().render();
    // spawn_local(run());
}

