use std::rc::Rc;

use carlettos_chess::{chess_controller::CChess, prelude::*};
use yew::prelude::*;

use crate::{controllers::CarlettosChessController, state::CarlettosChessState};

#[derive(Properties, PartialEq)]
pub struct SquareProp {
    board: Board,
    piece: Piece,
    square: Pos,
    on_click: Callback<Pos>,
    is_move: bool,
    is_take: bool,
    is_attack: bool,
}

#[function_component(ChessTile)]
pub fn tile(
    SquareProp {
        board: _,
        piece,
        square,
        on_click,
        is_move,
        is_take,
        is_attack,
    }: &SquareProp,
) -> Html {
    let idx = (square.x as u128) << 64 | square.y as u128;

    let color = if (square.x + square.y) % 2 == 0 {
        "square-black"
    } else {
        "square-white"
    };

    let piece_name = match piece {
        Piece::None => "".to_string(),
        Piece::Pawn(data) => format!("{:?}_pawn", data.color).to_lowercase(),
        Piece::Knight(data) => format!("{:?}_knight", data.color).to_lowercase(),
        Piece::Bishop(data) => format!("{:?}_bishop", data.color).to_lowercase(),
        Piece::Rook(data) => format!("{:?}_rook", data.color).to_lowercase(),
        Piece::Queen(data) => format!("{:?}_queen", data.color).to_lowercase(),
        Piece::King(data) => format!("{:?}_king", data.color).to_lowercase(),
        Piece::Archer(data) => format!("{:?}_archer", data.color).to_lowercase(),
    };

    let on_square_click = {
        let square = square.clone();
        let on_click = on_click.clone();
        move |_| on_click.emit(square.clone())
    };

    let status = if *is_move {
        "move"
    } else if *is_take && *is_attack {
        "take-attack"
    } else if *is_take {
        "take"
    } else if *is_attack {
        "attack"
    } else {
        ""
    };

    let img_html = if matches!(piece, Piece::None) {
        html! {}
    } else {
        html! { <img draggable={ "false" } class={classes!("carlettos-piece")} src={ format!("assets/{piece_name}.png") }/> }
    };

    html! {
        <div key={idx} class={classes!("carlettos-chess-square", color)} onclick={on_square_click}>
            {
                img_html
            }
            <div class={classes!("carlettos-movement", status)}></div>
        </div>
    }
}

#[function_component(CarlettosChess)]
pub fn chess() -> Html {
    let chess = use_reducer(CarlettosChessState::default);
    let chess_controller = Rc::new(CarlettosChessController::new(chess.clone()));

    {
        let chess_controller = chess_controller.clone();
        use_effect_with((), move |_| {
            chess_controller.start();
            || ()
        });
    }

    let on_tile_click = {
        let chess_controller = chess_controller.clone();
        Callback::from(move |pos| chess_controller.on_click(pos))
    };

    let on_start_click = {
        let chess_controller = chess_controller.clone();
        Callback::from(move |_: ()| {
            chess_controller.start();
        })
    };

    let on_button_click = {
        let on_start_click = on_start_click.clone();
        move |_| on_start_click.emit(())
    };

    let on_display_click = {
        let chess_controller = chess_controller.clone();
        Callback::from(move |pos| chess_controller.on_display_click(pos))
    };

    let rows = (0..chess.board.height()).rev().map(|row| {
        html! {
            <div class={classes!("carlettos-chess-row")}>
                { for chess.board.row_iter(row).map(|tile| {
                    html! { <ChessTile board={chess.board.board.clone()} piece={tile.piece.clone()} square={tile.pos().clone()} on_click={on_tile_click.clone()} is_move={chess.board.has_move(tile.pos())} is_take={chess.board.has_take(tile.pos())} is_attack={chess.board.has_attack(tile.pos())} /> }
                }) }
            </div>
        }});

    html! {
        <section class={classes!("carlettos-chess-container")}>
            <header>
                <h1>{ "Carlettos Chess" }</h1>
                <button onclick={on_button_click}>{ "Start" }</button>
            </header>
            <ChessPiecesDisplay display={chess.display.clone()} on_click={on_display_click} />
            <section class={classes!("carlettos-chess-board")}>
                { for rows }
            </section>
            <footer>
                <div>{ format!("Selected: {:?}", chess.board.selected) }</div>
                <div>{ format!("Debug: {:?}", chess.board.selected.as_ref().map(|p| chess.board.board.get(p))) }</div>
            </footer>
        </section>
    }
}

#[derive(Properties, PartialEq)]
pub struct ChessPieceDisplayProp {
    display: CChess,
    on_click: Callback<Pos>,
}
/// This component is used to display pieces that can be placed on the board.
/// So I can test new pieces without having to change the board state.
/// The placement of the pieces is done by clicking on the piece and then on the board.
#[function_component(ChessPiecesDisplay)]
pub fn chess_pieces_display(
    ChessPieceDisplayProp { display, on_click }: &ChessPieceDisplayProp,
) -> Html {
    let rows = (0..display.height()).rev().map(|row| {
        html! {
            <div class={classes!("carlettos-chess-row")}>
                { for display.row_iter(row).map(|tile| {
                    let is_move = tile.pos() == &display.selected;
                    html! { <ChessTile board={display.board.clone()} piece={tile.piece.clone()} square={tile.pos().clone()} on_click={on_click.clone()} is_move={is_move} is_take={false} is_attack={false} /> }
                }) }
            </div>
        }});

    html! {
        <div>
            <div class={classes!("carlettos-chess-board", "cchess-display")}>
                { for rows }
            </div>
        </div>
    }
}
