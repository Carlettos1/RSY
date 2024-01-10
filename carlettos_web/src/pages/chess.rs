use std::rc::Rc;

use chess_api::*;
use gloo::timers::callback::Interval;
use yew::prelude::*;

use crate::{controllers::ChessController, state::ChessState};

#[derive(Properties, PartialEq)]
pub struct SquareProp {
    board: Board,
    piece: Option<Piece>,
    square: (usize, usize),
    on_click: Callback<(usize, usize)>,
}

#[function_component(ChessSquare)]
pub fn square(
    SquareProp {
        board,
        piece,
        square,
        on_click,
    }: &SquareProp,
) -> Html {
    let idx = point_to_index(*square);
    let status = if board.take_squares.contains(square) {
        "square-take"
    } else if board.move_squares.contains(square) {
        "square-move"
    } else {
        ""
    };

    let color = if (square.0 + square.1) % 2 == 0 {
        "square-black"
    } else {
        "square-white"
    };

    let piece_name = if let Some(piece) = piece {
        piece.class_name()
    } else {
        "".to_string()
    };

    let on_square_click = {
        let square = *square;
        let on_click = on_click.clone();
        move |_| on_click.emit(square)
    };

    html! {
        <div key={idx} class={classes!("chess-square", color)} onclick={on_square_click}>
            {
                if piece.is_some() {
                    html! { <img class={classes!("piece")} src={ format!("assets/{piece_name}.png") }/> } }
                else {
                    html! { }
                }
            }
            <div class={classes!("movement", status)}></div>
        </div>
    }
}

#[function_component(ChessBoard)]
pub fn chess() -> Html {
    let chess = use_reducer(ChessState::default);
    let chess_controller = Rc::new(ChessController::new(chess.clone()));

    {
        let chess_controller = chess_controller.clone();
        use_effect_with((), move |_| {
            chess_controller.get_chess();
            || ()
        });
    }

    let on_square_click = {
        let chess_controller = chess_controller.clone();
        Callback::from(move |square: (usize, usize)| {
            chess_controller.on_click(square);
        })
    };

    let on_start_click = {
        let chess_controller = chess_controller.clone();
        Callback::from(move |_: ()| {
            chess_controller.update_chess(Board::default());
        })
    };

    let on_button_click = {
        let on_start_click = on_start_click.clone();
        move |_| on_start_click.emit(())
    };

    {
        let update = {
            let chess_controller = chess_controller.clone();
            Callback::from(move |_: ()| {
                chess_controller.get_chess();
            })
        };
        use_effect(|| {
            let interval = Interval::new(200, move || update.emit(()));
            move || drop(interval)
        });
    }

    let rows = chess.board.pieces.chunks(8).enumerate().map(|(y, pieces)| {
        let offset = y * 8;

        let pieces: Html = pieces
            .iter()
            .enumerate()
            .map(|(x, piece)| html! {<ChessSquare board={chess.board.clone()} piece={piece.clone()} square={index_to_point(x + offset)} on_click={on_square_click.clone()} />}).collect();
        html! {
            <div key={y} class="chess-row">
                { pieces }
            </div>
        }
    });

    html! {
        <div>
            <section class="chess-container">
                <header>
                    <h1>{ "Chess" }</h1>
                </header>
                <section class="chess-board">
                    <div class="chess">
                        { for rows }
                    </div>
                    <div>
                        <button onclick={on_button_click}>{ "Re-start" }</button>
                    </div>
                </section>
            </section>
        </div>
    }
}
