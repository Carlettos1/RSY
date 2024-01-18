use crate::{board::Board, piece::Piece, Color, Info, Pos};

pub trait Ability {
    fn ply(board: &mut Board, from: &Pos, info: Info);
    fn can_ply(board: &Board, from: &Pos, info: &Info) -> bool;
}

pub struct Pawn;

impl Ability for Pawn {
    fn ply(board: &mut Board, from: &Pos, info: Info) {
        match info {
            Info::Piece(piece) => drop(board.get_mut(from).unwrap().replace(piece)),
            _ => panic!("Non piece info"),
        }
    }

    fn can_ply(board: &Board, from: &Pos, _info: &Info) -> bool {
        let tile = board.get(from).unwrap();
        match tile.get_color().unwrap() {
            Color::White => board.get(&Pos::new(from.x, from.y + 1)).is_none(),
            Color::Black => from.y == 0,
        }
    }
}

pub struct Knight;

impl Ability for Knight {
    fn ply(board: &mut Board, from: &Pos, _info: Info) {
        let color = board.get(from).unwrap().get_color().unwrap().clone();
        board
            .get_mut(&from.east())
            .unwrap()
            .replace(Piece::pawn(color.clone()));
        board
            .get_mut(&from.west().unwrap())
            .unwrap()
            .replace(Piece::pawn(color));
    }

    fn can_ply(board: &Board, from: &Pos, _info: &Info) -> bool {
        let e = from.east();
        let w = from.west();
        let w = if let Some(pos) = w { pos } else { return false };
        match (board.get(&e), board.get(&w)) {
            (Some(te), Some(tw)) if te.is_empty() && tw.is_empty() => true,
            _ => false,
        }
    }
}
