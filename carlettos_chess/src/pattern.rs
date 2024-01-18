use crate::{board::Board, Color, Pos};

pub fn pawn_move(board: &Board, color: &Color, from: &Pos, to: &Pos) -> bool {
    let (next, next2) = match color {
        Color::White => (from.checked_shift(0, 1), from.checked_shift(0, 2)),
        Color::Black => (from.checked_shift(0, -1), from.checked_shift(0, -2)),
    };
    if to == &next {
        true
    } else {
        to == &next2 && board.is_empty(to)
    }
}

pub fn pawn_take(_board: &Board, color: &Color, from: &Pos, to: &Pos) -> bool {
    let (left, right) = match color {
        Color::White => (from.checked_shift(-1, 1), from.checked_shift(1, 1)),
        Color::Black => (from.checked_shift(-1, -1), from.checked_shift(1, -1)),
    };
    Some(to) == left.as_ref() || Some(to) == right.as_ref()
}

pub fn knight(from: &Pos, to: &Pos) -> bool {
    let Pos { x, y } = from.abs_diff(to);
    (x == 2 && y == 1) || (x == 1 && y == 2)
}

pub fn king(from: &Pos, to: &Pos) -> bool {
    let Pos { x, y } = from.abs_diff(to);
    x < 2 && y < 2
}

#[cfg(test)]
mod test {
    use crate::{board::Board, piece::Piece, Action, Color, Pos};

    fn pawn_move(color: Color) {
        let sign = match color {
            Color::Black => -1,
            Color::White => 1,
        };

        for (x, y) in (0..8).flat_map(|x| (0..8).map(move |y| (x, y))) {
            let mut board = Board::default();
            let pawn_pos = Pos::new(x, y);
            let pos1 = pawn_pos.checked_shift(0, sign);
            let pos2 = pawn_pos.checked_shift(0, sign * 2);
            board
                .get_mut(&pawn_pos)
                .unwrap()
                .replace(Piece::pawn(color.clone()));
            for to in (0..8).flat_map(|i| (0..8).map(move |j| Pos::new(i, j))) {
                let action = Action::Move {
                    from: pawn_pos.clone(),
                    to: to.clone(),
                };
                assert_eq!(
                    board.get(&pawn_pos).unwrap().piece.can_do(&board, action),
                    to == pos1 || to == pos2
                );
            }
        }
    }

    fn pawn_take(color: Color) {
        let sign = match color {
            Color::Black => -1,
            Color::White => 1,
        };

        for (x, y) in (0..8).flat_map(|x| (0..8).map(move |y| (x, y))) {
            let mut board = Board::default();
            let pawn_pos = Pos::new(x, y);
            let pos1 = pawn_pos.checked_shift(-1, sign);
            let pos2 = pawn_pos.checked_shift(1, sign);
            board
                .get_mut(&pawn_pos)
                .unwrap()
                .replace(Piece::pawn(color.clone()));
            for to in (0..8).flat_map(|i| (0..8).map(move |j| Pos::new(i, j))) {
                let action = Action::Take {
                    from: pawn_pos.clone(),
                    to: to.clone(),
                };
                assert_eq!(
                    board.get(&pawn_pos).unwrap().piece.can_do(&board, action),
                    to == pos1 || to == pos2
                );
            }
        }
    }

    #[test]
    fn pawn_move_white() {
        pawn_move(Color::White);
    }

    #[test]
    fn pawn_move_black() {
        pawn_move(Color::Black);
    }

    #[test]
    fn pawn_take_white() {
        pawn_take(Color::White);
    }

    #[test]
    fn pawn_take_black() {
        pawn_take(Color::Black);
    }
}
