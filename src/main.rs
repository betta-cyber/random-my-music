mod app;
mod components;
mod api;
mod pages;
mod store;
mod router;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
