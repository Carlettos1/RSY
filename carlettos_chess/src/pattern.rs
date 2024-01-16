use crate::{board::Board, Color, Pos};

pub fn pawn_move(board: &Board, color: &Color, Pos { x, y }: &Pos, to: &Pos) -> bool {
    let (next, next2) = match color {
        Color::White => (Pos::new(*x, y + 1), Pos::new(*x, y + 2)),
        Color::Black => (Pos::new(*x, y - 1), Pos::new(*x, y - 2)),
    };
    if to == &next {
        true
    } else if to == &next2 && board.is_empty(&next) {
        true
    } else {
        false
    }
}

pub fn pawn_take(_board: &Board, color: &Color, Pos { x, y }: &Pos, to: &Pos) -> bool {
    let (left, right) = match color {
        Color::White => (Pos::new(*x - 1, y + 1), Pos::new(*x + 1, y + 1)),
        Color::Black => (Pos::new(*x - 1, y - 1), Pos::new(*x + 1, y - 1)),
    };
    to == &left || to == &right
}

pub fn knight(from: &Pos, to: &Pos) -> bool {
    let Pos { x, y } = from - to;
    (x == 2 && y == 1) || (x == 1 && y == 2)
}

pub fn king(from: &Pos, to: &Pos) -> bool {
    let Pos { x, y } = from - to;
    x < 2 && y < 2
}

#[cfg(test)]
mod test {}
