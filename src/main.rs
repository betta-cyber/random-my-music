mod app;
mod components;
mod api;
mod pages;
mod store;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
