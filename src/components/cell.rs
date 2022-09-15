use sycamore::prelude::*;

use super::game::*;
use super::player::*;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum CellValue {
    #[default]
    Empty,
    Player(Player),
}

impl CellValue {
    fn to_string(self) -> String {
        match self {
            CellValue::Empty => " ".to_string(),
            CellValue::Player(p) => p.to_string(),
        }
    }
    fn is_set(self) -> bool {
        self != CellValue::Empty
    }
}

#[component]
pub fn Cell<G: Html>(cx: Scope, ind: usize) -> View<G> {
    let ind = create_ref(cx, ind);
    let game_state = use_context::<Signal<GameState>>(cx);
    let current_cell = create_memo(cx, || (*game_state.get()).cells[*ind]);

    let click = |_| {
        if !(*current_cell.get()).is_set() && !(*game_state.get()).is_won() {
            game_state.set({
                let mut gs = (*game_state.get()).clone();
                gs.cells[*ind] = CellValue::Player(gs.active_player.next());
                gs.active_player = gs.active_player.next();
                gs
            });
        }
    };

    view! {cx,
        button(on:click=click) { ((*current_cell.get()).to_string()) }
    }
}
