use sycamore::prelude::*;

use super::cell::*;
use super::player::*;

#[derive(Clone, PartialEq, Eq)]
pub struct GameState {
    pub cells: Vec<CellValue>,
    pub active_player: Player,
    pub winners: Vec<usize>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            cells: vec![CellValue::Empty; 9],
            active_player: Player::default(),
            winners: vec![0, 1, 2],
        }
    }
    pub fn is_draw(&self) -> bool {
        !self.cells.contains(&CellValue::Empty)
    }
    pub fn is_won(&self) -> Option<Vec<usize>> {
        let lines = vec![
            vec![0, 1, 2],
            vec![3, 4, 5],
            vec![6, 7, 8],
            vec![0, 3, 6],
            vec![1, 4, 7],
            vec![2, 5, 8],
            vec![0, 4, 8],
            vec![2, 4, 6],
        ];
        for line in lines {
            let (a, b, c) = (line[0], line[1], line[2]);
            if self.cells[a] != CellValue::Empty
                && self.cells[a] == self.cells[b]
                && self.cells[b] == self.cells[c]
            {
                return Some(line);
            }
        }
        None
    }
}

#[component]
pub fn Game<G: Html>(cx: Scope) -> View<G> {
    let game_state = create_signal(cx, GameState::new());

    let cells = create_memo(cx, || {
        (*game_state.get())
            .cells
            .clone()
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, CellValue)>>()
    });

    let game_status = create_memo(cx, || {
        if let Some(line) = (*game_state.get()).is_won() {
            game_state.set({
                let mut gs = (*game_state.get()).clone();
                gs.winners = line;
                gs
            });
            match (*game_state.get()).active_player {
                Player::X => "X wins!".to_string(),
                Player::O => "O wins!".to_string(),
            }
        } else if (*game_state.get()).is_draw() {
            "Its a draw".to_owned()
        } else {
            format!(
                "Player {}'s turn",
                (*game_state.get()).active_player.next().to_string()
            )
        }
    });

    let restart = |_| {
        game_state.set(GameState::new());
    };
    let restart_btn = create_memo(cx, || {
        if (*game_state.get()).is_won().is_some() || (*game_state.get()).is_draw() {
            "restart_btn game_over"
        } else {
            "restart_btn"
        }
    });
    let theme = create_signal(cx, false);
    let theme_btn_icon = create_memo(cx, || if *theme.get() { "☼" } else { "◑" });
    let style = create_memo(cx, || {
        if *theme.get() {
            format!(
                "{}{}{}{}{}",
                "--bg: #ffffff;",
                "--mg: #efefef;",
                "--fg: #000000;",
                "--accent: #4e3bfb;",
                "--highlight: #3dfe8e",
            )
        } else {
            format!(
                "{}{}{}{}{}",
                "--bg: #232328;",
                "--mg: #343338;",
                "--fg: #ffffff;",
                "--accent: #4e3bfb;",
                "--highlight: #43a346",
            )
        }
    });
    let toggle_theme = |_| theme.set(!*theme.get());
    provide_context_ref(cx, game_state);

    view! {cx,
        div(class="body", style=style){
            main{
                div(class="board") {
                    Indexed (
                        iterable=cells,
                        view = |cx, x| view! { cx,
                            Cell (x.0)
                        }
                    )
                }

                div(class="game_status") { (*game_status.get()) }

                div (class="bottom_btns"){
                    button(on:click=restart, class=restart_btn) {"Restart"}
                    button(on:click=toggle_theme, class="theme_btn") {(*theme_btn_icon.get())}
                }

            }
        }
    }
}
