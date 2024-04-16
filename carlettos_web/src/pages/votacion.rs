use std::rc::Rc;

use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
    controllers::VotesController,
    models::{Check, Vote},
    state::{VoteAction, VotesState},
};

#[function_component(Votacion)]
pub fn currently_programming() -> Html {
    let all_votes: Vec<_> = vec![0, 1, 2, 3, 4, 5]
        .into_iter()
        .map(|id| Vote { id })
        .collect();

    let state = use_reducer(VotesState::default);
    let controller = Rc::new(VotesController::new(state.clone()));
    let input_node_ref = use_node_ref();

    let on_login = {
        let input_node_ref = input_node_ref.clone();
        let controller = controller.clone();
        let init = Callback::from(move |id: String| {
            controller.init_votes(id);
        });

        Callback::from(move |_| {
            let input = input_node_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                init.emit(input.value());
                input.set_value("");
            }
        })
    };

    let on_remove_vote = {
        let controller = controller.clone();
        Callback::from(move |vote_id: usize| {
            controller.remove_vote(vote_id);
        })
    };

    let on_add_vote = {
        let controller = controller.clone();
        Callback::from(move |vote_id: usize| {
            controller.add_vote(vote_id);
        })
    };

    let on_image_click = {
        let controller = controller.clone();
        let rm = on_remove_vote.clone();
        let add = on_add_vote.clone();
        Callback::from(move |image_id: usize| {
            let action = controller.click(image_id);
            match action {
                VoteAction::Set(_) => unreachable!("Click doesn't return set"),
                VoteAction::Remove(vote) => rm.emit(vote.id),
                VoteAction::Add(vote) => add.emit(vote.id),
            }
        })
    };

    let on_enter = {
        let input_node_ref = input_node_ref.clone();
        let controller = controller.clone();
        let init = Callback::from(move |id: String| {
            controller.init_votes(id);
        });

        Callback::from(move |kbe: KeyboardEvent| {
            if kbe.key() == *"Enter" {
                let input = input_node_ref.cast::<HtmlInputElement>();

                if let Some(input) = input {
                    init.emit(input.value());
                    input.set_value("");
                }
            }
        })
    };

    html! {
        <div class="votes">
            <label> { "Ingresar RUT sin puntos ni guíon" } </label>
            <div class="center">
                <input onkeydown={on_enter} ref={input_node_ref} id="login_text" type="text"/>
                <button onclick={on_login}> {"Ingresar"} </button>
            </div>
            <div>
                <VoteList
                    login={state.login}
                    all_votes={all_votes.clone()}
                    checks={state.checks.clone()}
                    on_click={on_image_click.clone()}
                />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct VoteProps {
    pub login: bool,
    pub all_votes: Vec<Vote>,
    pub checks: Vec<Check>,
    pub on_click: Callback<usize>,
}

#[function_component(VoteList)]
pub fn vote_list(
    VoteProps {
        login,
        all_votes,
        checks,
        on_click,
    }: &VoteProps,
) -> Html {
    if !login {
        return html! {};
    }

    let votes = all_votes.iter().zip(checks.iter()).enumerate().map(|(id, (vote, check))| {
        let on_click = {
            let a = on_click.clone();
            move |_| a.emit(id)
        };
        html! {
            <img draggable={ "false" } src={ format!("assets/fractal{}.png", vote.id) } class={ classes!("vote-item", check.to_class()) } onclick={on_click}/>
        }
    });

    html! {
        <div>
            <div>
                { "Lista" }
            </div>
            <div class="vote-list">
                { for votes }
            </div>
        </div>
    }
}
