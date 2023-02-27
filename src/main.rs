mod app;
mod components;
mod api;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
