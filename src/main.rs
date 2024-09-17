#![feature(trait_upcasting)]
#![feature(anonymous_lifetime_in_impl_trait)]
mod app;
mod graph;
mod sheet;
mod sheetRender;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
