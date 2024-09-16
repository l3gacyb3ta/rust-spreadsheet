#![feature(trait_upcasting)]
mod app;
mod graph;
mod sheet;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
