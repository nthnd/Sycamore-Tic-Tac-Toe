use sycamore::prelude::*;

mod components;
use components::game::Game;

fn main() {
    sycamore::render(|cx| {
        view! {cx,
            Game {}
        }
    })
}
