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

pub fn pawn_take(_board: &Board, color: &Color, from: &Pos, to: &Pos) -> bool {
    let (left, right) = match color {
        Color::White => (from.checked_shift(-1, 1), from.checked_shift(1, 1)),
        Color::Black => (from.checked_shift(-1, -1), from.checked_shift(1, 1)),
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

    #[test]
    fn pawn_move_white() {
        for (x, y) in (0..8).flat_map(|x| (0..8).map(move |y| (x, y))) {
            let mut board = Board::default();
            let pawn_pos = Pos::new(x, y);
            let pos1 = Pos::new(x, y + 1);
            let pos2 = Pos::new(x, y + 2);
            board
                .get_mut(&pawn_pos)
                .unwrap()
                .replace(Piece::pawn(Color::White));
            for to in (0..8).flat_map(|i| (0..8).map(move |j| Pos::new(i, j))) {
                let action = Action::Move {
                    from: pawn_pos.clone(),
                    to: to.clone(),
                };
                if to == pos1 || to == pos2 {
                    assert_eq!(
                        board.get(&pawn_pos).unwrap().piece.can_do(&board, action),
                        true
                    );
                } else {
                    assert_eq!(
                        board.get(&pawn_pos).unwrap().piece.can_do(&board, action),
                        false
                    );
                }
            }
        }
    }
    #[test]

    fn pawn_take_white() {
        for (x, y) in (0..8).flat_map(|x| (0..8).map(move |y| (x, y))) {
            let mut board = Board::default();
            let pawn_pos = Pos::new(x, y);
            let pos1 = pawn_pos.checked_shift(-1, 1);
            let pos2 = pawn_pos.checked_shift(1, 1);
            if let Some(pos) = pos1.as_ref() {
                assert!(pos.x <= 8 && pos.y <= 8)
            }
            if let Some(pos) = pos2.as_ref() {
                assert!(pos.x <= 8 && pos.y <= 8)
            }
            board
                .get_mut(&pawn_pos)
                .unwrap()
                .replace(Piece::pawn(Color::White));
            for to in (0..8).flat_map(|i| (0..8).map(move |j| Pos::new(i, j))) {
                let action = Action::Take {
                    from: pawn_pos.clone(),
                    to: to.clone(),
                };
                if (pos1.is_some() && &to == pos1.as_ref().unwrap())
                    || (pos2.is_some() && &to == pos2.as_ref().unwrap())
                {
                    assert_eq!(
                        board.get(&pawn_pos).unwrap().piece.can_do(&board, action),
                        true
                    );
                } else {
                    assert_eq!(
                        board.get(&pawn_pos).unwrap().piece.can_do(&board, action),
                        false
                    );
                }
            }
        }
    }
}
