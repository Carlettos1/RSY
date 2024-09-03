use std::rc::Rc;

use gloo::utils::document;
use log::info;
use serde::{Deserialize, Serialize};
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;

use crate::{controllers::C2048LeaderboardController, state::C2048LeaderboardState};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    pub name: String,
    pub score: usize,
    pub max_tile: usize,
    pub min_energy: isize,
    pub max_energy: isize,
}

impl Entry {
    pub fn new(
        name: String,
        score: usize,
        max_tile: usize,
        min_energy: isize,
        max_energy: isize,
    ) -> Self {
        Entry {
            name,
            score,
            max_tile,
            min_energy,
            max_energy,
        }
    }

    pub fn to_table_row(&self) -> Html {
        html! {
            <tr>
                <td>{&self.name}</td>
                <td>{self.score}</td>
                <td>{self.max_tile}</td>
                <td>{self.min_energy}</td>
                <td>{self.max_energy}</td>
            </tr>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct C2048Prop {
    pub show_leaderboard: bool,
    pub score: usize,
    pub max_tile: usize,
    pub min_energy: isize,
    pub max_energy: isize,
}

#[function_component(C2048Leaderboard)]
pub fn c2048_leaderboard(
    C2048Prop {
        show_leaderboard,
        score,
        max_tile,
        min_energy,
        max_energy,
    }: &C2048Prop,
) -> Html {
    let state = use_reducer(C2048LeaderboardState::default);
    let controller = Rc::new(C2048LeaderboardController::new(state.clone()));

    {
        let controller = controller.clone();
        use_effect_with((), move |_| {
            controller.get_highscores();
            || ()
        });
    }

    let add_hs = {
        let controller = controller.clone();
        Callback::from(move |entry: Entry| {
            controller.add_highscore(entry);
        })
    };

    let onclick = {
        info!(
            "{:?}",
            document()
                .get_element_by_id("c2048_highscore_input")
                .map(|e| e.dyn_into::<HtmlInputElement>())
        );
        let entry = Entry::new(String::new(), *score, *max_tile, *min_energy, *max_energy);
        let add_hs = add_hs.clone();
        Callback::from(move |_| {
            let input = document()
                .get_element_by_id("c2048_highscore_input")
                .unwrap()
                .dyn_into::<HtmlInputElement>();

            if let Ok(input) = input {
                add_hs.emit(Entry {
                    name: input.value(),
                    ..entry
                });
                input.set_value("");
            }
        })
    };

    let on_enter = {
        let entry = Entry::new(String::new(), *score, *max_tile, *min_energy, *max_energy);

        Callback::from(move |kbe: KeyboardEvent| {
            let input = document()
                .get_element_by_id("c2048_highscore_input")
                .unwrap()
                .dyn_into::<HtmlInputElement>();

            if kbe.key() == *"Enter" {
                if let Ok(input) = input {
                    add_hs.emit(Entry {
                        name: input.value(),
                        ..entry
                    });
                    input.set_value("");
                }
            }
        })
    };

    html! {
        <section class="c2048-leaderboard">
            <h1 class="is-size-3">
                { "Leaderboard" }
            </h1>
            <table class="table is-fullwidth">
                <thead>
                    <tr>
                        <th> { "Nombre" } </th>
                        <th> { "Score" } </th>
                        <th> { "Max Tile" } </th>
                        <th> { "Min Energy" } </th>
                        <th> { "Max Energy" } </th>
                    </tr>
                </thead>
                {
                    if *show_leaderboard {
                        html!{
                            <thead>
                                <tr>
                                    <th> <input id="c2048_highscore_input" class="input c2048_highscore_input" type="text" placeholder="Put your name" onkeydown={on_enter} />
                                    <button class="button" type="submit" style="height: 24px;" onclick={onclick}> { "Enter" } </button> </th>
                                    <th> { score } </th>
                                    <th> { max_tile } </th>
                                    <th> { min_energy } </th>
                                    <th> { max_energy } </th>
                                </tr>
                            </thead>
                        }
                    } else {
                        html!{}
                    }
                }
                <tfoot>
                    <tr>
                        <th> { "Nombre" } </th>
                        <th> { "Score" } </th>
                        <th> { "Max Tile" } </th>
                        <th> { "Min Energy" } </th>
                        <th> { "Max Energy" } </th>
                    </tr>
                </tfoot>
                <tbody>
                { for state.entries.iter().map(Entry::to_table_row) }
                </tbody>
            </table>
        </section>
    }
}
