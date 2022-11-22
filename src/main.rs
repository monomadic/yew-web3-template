pub(crate) mod components;
pub(crate) mod config;
mod layout;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<layout::App>::new().render();
}
