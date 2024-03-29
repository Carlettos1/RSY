use crate::{board::Board, Color, Pos, SubDirection};

pub fn pawn_move(board: &Board, color: &Color, from: &Pos, to: &Pos) -> bool {
    let (next, next2) = match color {
        Color::White => (from.shift(0, 1), from.shift(0, 2)),
        Color::Black => (from.shift(0, -1), from.shift(0, -2)),
    };
    if to == &next {
        true
    } else {
        to == &next2 && next.map(|n| board.is_empty(&n)).unwrap_or_default()
    }
}

pub fn pawn_take(_board: &Board, color: &Color, from: &Pos, to: &Pos) -> bool {
    let (left, right) = match color {
        Color::White => (from.shift(-1, 1), from.shift(1, 1)),
        Color::Black => (from.shift(-1, -1), from.shift(1, -1)),
    };
    to == &left || to == &right
}

pub fn knight(from: &Pos, to: &Pos) -> bool {
    let Pos { x, y } = from.abs_diff(to);
    (x == 2 && y == 1) || (x == 1 && y == 2)
}

pub fn king(from: &Pos, to: &Pos) -> bool {
    let Pos { x, y } = from.abs_diff(to);
    x < 2 && y < 2
}

pub fn bishop(board: &Board, from: &Pos, to: &Pos) -> bool {
    let Pos { x, y } = from.abs_diff(to);
    if x != y {
        return false;
    }

    let signx = if to.x > from.x { 1isize } else { -1 };
    let signy = if to.y > from.y { 1isize } else { -1 };
    board
        .ray_cast_empty(from, None, &(signx, signy))
        .contains(to)
}

pub fn rook(board: &Board, from: &Pos, to: &Pos) -> bool {
    let Pos { x, y } = from.abs_diff(to);
    if x != 0 && y != 0 {
        return false;
    }

    let signx = to.x.cmp(&from.x) as isize;
    let signy = to.y.cmp(&from.y) as isize;
    board
        .ray_cast_empty(from, None, &(signx, signy))
        .contains(to)
}

pub fn queen(board: &Board, from: &Pos, to: &Pos) -> bool {
    bishop(board, from, to) || rook(board, from, to)
}

pub fn square(from: &Pos, to: &Pos, range: usize) -> bool {
    let Pos { x, y } = from.abs_diff(to);
    x <= range && y <= range
}

pub fn cross(from: &Pos, to: &Pos, range: usize) -> bool {
    let Pos { x, y } = from.abs_diff(to);
    (x == 0 || y == 0) && (x + y <= range)
}

pub fn blockeable_cross(
    board: &Board,
    from: &Pos,
    to: &Pos,
    color: &Color,
    range: usize,
    strength: usize,
) -> bool {
    let Pos { x, y } = from.abs_diff(to);
    if x != 0 && y != 0 {
        return false;
    }
    if x + y > range {
        return false;
    }
    let signx = to.x.cmp(&from.x) as isize;
    let signy = to.y.cmp(&from.y) as isize;
    board
        .ray_cast(from, Some(range), &(signx, signy), |t| {
            t.piece.is_impenetrable(&strength) && !t.is_controlled_by(color)
        })
        .contains(to)
}

pub fn archer_move(from: &Pos, to: &Pos) -> bool {
    magician_move(from, to) || king(from, to)
}

pub fn magician_move(from: &Pos, to: &Pos) -> bool {
    let Pos { x, y } = from.abs_diff(to);
    x == y && x <= 2
}

pub fn structure_move(from: &Pos, to: &Pos) -> bool {
    let Pos { x, y } = from.abs_diff(to);
    (x == 0 && y == 1) || (x == 1 && y == 0)
}

pub fn crazy_pawn(board: &Board, from: &Pos, to: &Pos) -> bool {
    let subdirection = match (board.rng.movement() * 8.0).floor() as usize {
        0 => SubDirection::N,
        1 => SubDirection::NE,
        2 => SubDirection::E,
        3 => SubDirection::SE,
        4 => SubDirection::S,
        5 => SubDirection::SW,
        6 => SubDirection::W,
        7 => SubDirection::NW,
        _ => panic!("Non 0..8 random number range in crazy pawn movement"),
    };
    to == &from.subdirection_shift(&subdirection)
        || to
            == &from
                .subdirection_shift(&subdirection)
                .and_then(|pos| pos.subdirection_shift(&subdirection))
}

pub fn super_pawn_move(board: &Board, color: &Color, from: &Pos, to: &Pos) -> bool {
    for i in [-1, 0, 1] {
        let (next, next2) = match color {
            Color::White => (from.shift(i, 1), from.shift(i, 2)),
            Color::Black => (from.shift(i, -1), from.shift(i, -2)),
        };
        if to == &next || to == &next2 && next.map(|n| board.is_empty(&n)).unwrap_or_default() {
            return true;
        }
    }
    false
}

pub fn super_pawn_take(_board: &Board, color: &Color, from: &Pos, to: &Pos) -> bool {
    let (left, right, front) = match color {
        Color::White => (from.shift(-1, 1), from.shift(1, 1), from.shift(0, 1)),
        Color::Black => (from.shift(-1, -1), from.shift(1, -1), from.shift(0, -1)),
    };
    to == &left || to == &right || to == &front
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board::Board;
    use crate::piece::Piece;
    use crate::Action;
    use crate::Color;
    use crate::Pos;

    fn pawn_move(color: Color) {
        let sign = match color {
            Color::Black => -1,
            Color::White => 1,
        };

        for (x, y) in (0..8).flat_map(|x| (0..8).map(move |y| (x, y))) {
            let mut board = Board::default();
            let pawn_pos = Pos::new(x, y);
            let pos1 = pawn_pos.shift(0, sign);
            let pos2 = pawn_pos.shift(0, sign * 2);
            board
                .get_mut(&pawn_pos)
                .unwrap()
                .replace(Piece::pawn(color.clone()));
            for to in board.shape().points_iter() {
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
            let pos1 = pawn_pos.shift(-1, sign);
            let pos2 = pawn_pos.shift(1, sign);
            board
                .get_mut(&pawn_pos)
                .unwrap()
                .replace(Piece::pawn(color.clone()));
            for to in board.shape().points_iter() {
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

    #[test]
    fn test_pawn_move() {
        let board = Board::default();
        let color = Color::White;
        let from = Pos::new(1, 1);
        let to1 = Pos::new(1, 2);
        let to2 = Pos::new(1, 3);
        let to3 = Pos::new(2, 2);

        assert!(super::pawn_move(&board, &color, &from, &to1));
        assert!(super::pawn_move(&board, &color, &from, &to2));
        assert!(!super::pawn_move(&board, &color, &from, &to3));
    }

    #[test]
    fn test_pawn_take() {
        let board = Board::default();
        let color = Color::White;
        let from = Pos::new(1, 1);
        let to1 = Pos::new(0, 2);
        let to2 = Pos::new(2, 2);
        let to3 = Pos::new(1, 3);

        assert!(!super::pawn_move(&board, &color, &from, &to1));
        assert!(!super::pawn_move(&board, &color, &from, &to2));
        assert!(super::pawn_move(&board, &color, &from, &to3));
    }

    #[test]
    fn test_king() {
        let from = Pos::new(1, 1);
        let to1 = Pos::new(2, 2);
        let to2 = Pos::new(1, 2);
        let to3 = Pos::new(3, 3);

        assert!(king(&from, &to1));
        assert!(king(&from, &to2));
        assert!(!king(&from, &to3));
    }

    #[test]
    fn test_bishop() {
        let board = Board::default();
        let from = Pos::new(1, 1);
        let to1 = Pos::new(2, 2);
        let to2 = Pos::new(3, 3);
        let to3 = Pos::new(2, 3);

        assert!(bishop(&board, &from, &to1));
        assert!(bishop(&board, &from, &to2));
        assert!(!bishop(&board, &from, &to3));
    }

    #[test]
    fn test_knight() {
        let from = Pos::new(1, 1);
        let to1 = Pos::new(3, 2);
        let to2 = Pos::new(2, 3);
        let to3 = Pos::new(2, 2);

        assert!(knight(&from, &to1));
        assert!(knight(&from, &to2));
        assert!(!knight(&from, &to3));
    }

    #[test]
    fn rook_test() {
        let board = Board::default();
        let from = Pos::new(1, 1);
        let to1 = Pos::new(1, 2);
        let to2 = Pos::new(1, 3);
        let to3 = Pos::new(2, 2);

        assert!(rook(&board, &from, &to1));
        assert!(rook(&board, &from, &to2));
        assert!(!rook(&board, &from, &to3));
    }

    #[test]
    fn queen_test() {
        let board = Board::default();
        let from = Pos::new(1, 1);
        let to1 = Pos::new(1, 2);
        let to2 = Pos::new(1, 3);
        let to3 = Pos::new(2, 2);
        let to4 = Pos::new(2, 3);

        assert!(queen(&board, &from, &to1));
        assert!(queen(&board, &from, &to2));
        assert!(queen(&board, &from, &to3));
        assert!(!queen(&board, &from, &to4));
    }
}
