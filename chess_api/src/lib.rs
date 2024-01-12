use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Board {
    pub pieces: Vec<Option<Piece>>,
    pub piece_selected: Option<(usize, usize)>,
    pub en_passant_square: Option<(usize, usize)>,
    pub take_squares: Vec<(usize, usize)>,
    pub move_squares: Vec<(usize, usize)>,
    pub turn: Color,
}

pub fn starting_pieces() -> Vec<Option<Piece>> {
    let mut pieces = vec![None; 64];
    pieces[point_to_index((0, 0))] = Some(Piece::Rook(Rook::black()));
    pieces[point_to_index((1, 0))] = Some(Piece::Knight(Knight::black()));
    pieces[point_to_index((2, 0))] = Some(Piece::Bishop(Bishop::black()));
    pieces[point_to_index((3, 0))] = Some(Piece::King(King::black()));
    pieces[point_to_index((4, 0))] = Some(Piece::Queen(Queen::black()));
    pieces[point_to_index((5, 0))] = Some(Piece::Bishop(Bishop::black()));
    pieces[point_to_index((6, 0))] = Some(Piece::Knight(Knight::black()));
    pieces[point_to_index((7, 0))] = Some(Piece::Rook(Rook::black()));

    pieces[point_to_index((0, 1))] = Some(Piece::Pawn(Pawn::black()));
    pieces[point_to_index((1, 1))] = Some(Piece::Pawn(Pawn::black()));
    pieces[point_to_index((2, 1))] = Some(Piece::Pawn(Pawn::black()));
    pieces[point_to_index((3, 1))] = Some(Piece::Pawn(Pawn::black()));
    pieces[point_to_index((4, 1))] = Some(Piece::Pawn(Pawn::black()));
    pieces[point_to_index((5, 1))] = Some(Piece::Pawn(Pawn::black()));
    pieces[point_to_index((6, 1))] = Some(Piece::Pawn(Pawn::black()));
    pieces[point_to_index((7, 1))] = Some(Piece::Pawn(Pawn::black()));

    pieces[point_to_index((0, 7))] = Some(Piece::Rook(Rook::white()));
    pieces[point_to_index((1, 7))] = Some(Piece::Knight(Knight::white()));
    pieces[point_to_index((2, 7))] = Some(Piece::Bishop(Bishop::white()));
    pieces[point_to_index((3, 7))] = Some(Piece::King(King::white()));
    pieces[point_to_index((4, 7))] = Some(Piece::Queen(Queen::white()));
    pieces[point_to_index((5, 7))] = Some(Piece::Bishop(Bishop::white()));
    pieces[point_to_index((6, 7))] = Some(Piece::Knight(Knight::white()));
    pieces[point_to_index((7, 7))] = Some(Piece::Rook(Rook::white()));

    pieces[point_to_index((0, 6))] = Some(Piece::Pawn(Pawn::white()));
    pieces[point_to_index((1, 6))] = Some(Piece::Pawn(Pawn::white()));
    pieces[point_to_index((2, 6))] = Some(Piece::Pawn(Pawn::white()));
    pieces[point_to_index((3, 6))] = Some(Piece::Pawn(Pawn::white()));
    pieces[point_to_index((4, 6))] = Some(Piece::Pawn(Pawn::white()));
    pieces[point_to_index((5, 6))] = Some(Piece::Pawn(Pawn::white()));
    pieces[point_to_index((6, 6))] = Some(Piece::Pawn(Pawn::white()));
    pieces[point_to_index((7, 6))] = Some(Piece::Pawn(Pawn::white()));
    pieces
}

impl Board {
    pub fn get(&self, pos: &(usize, usize)) -> &Option<Piece> {
        &self.pieces[point_to_index(*pos)]
    }

    pub fn start(&mut self) {
        self.pieces = starting_pieces();
    }

    fn get_king_pos(&self, color: &Color) -> Option<(usize, usize)> {
        self.pieces.iter().enumerate().find_map(|(i, p)| match p {
            Some(Piece::King(k)) if &k.color == color => Some(index_to_point(i)),
            _ => None,
        })
    }

    pub fn is_check_mate(&self, color: &Color) -> bool {
        if self.is_check(color) {
            let king_pos = self.get_king_pos(color).unwrap();
            let piece = self.get(&king_pos).as_ref().unwrap();
            let mut possible_moves = Vec::with_capacity(8);

            for pos in (0..64).map(index_to_point) {
                if piece.can_move(self, &king_pos, &pos) || piece.can_take(self, &king_pos, &pos) {
                    possible_moves.push(pos);
                }
            }

            for possible_move in possible_moves {
                let mut cloned = self.clone();
                cloned.turn = color.clone();
                cloned.on_click(king_pos);
                cloned.on_click(possible_move);
                if !cloned.inner_is_check(color) {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    pub fn is_check(&self, color: &Color) -> bool {
        self.clone().inner_is_check(color)
    }

    /// This set the current turn to the oposite color to allow the can_take method.
    fn inner_is_check(&mut self, color: &Color) -> bool {
        self.turn = color.other();
        let king_pos = match self.get_king_pos(color) {
            Some(pos) => pos,
            None => return false,
        };

        for (from, piece) in self.pieces.iter().enumerate().flat_map(|(x, p)| match p {
            Some(piece) if piece.color() != color => Some((index_to_point(x), piece)),
            _ => None,
        }) {
            if piece.can_take(self, &from, &king_pos) {
                return true;
            }
        }
        false
    }

    pub fn on_click(&mut self, from: (usize, usize)) -> bool {
        let mut cloned = self.clone();
        if cloned.inner_on_click(from) {
            log::info!("check: {}", cloned.inner_is_check(&self.turn));
            if cloned.inner_is_check(&self.turn) {
                return false;
            }
        }
        self.inner_on_click(from)
    }

    /// Returns true if a piece moved
    fn inner_on_click(&mut self, from: (usize, usize)) -> bool {
        let piece = &self.pieces[point_to_index(from)];
        if self.move_squares.contains(&from) || self.take_squares.contains(&from) {
            let piece_selected = self.piece_selected.unwrap();
            // handle en passant take
            if let Some(en_passant) = self.en_passant_square {
                if let Some(Piece::Pawn(p)) = &self.pieces[point_to_index(piece_selected)] {
                    if from == en_passant && self.take_squares.contains(&from) {
                        match p.color {
                            Color::Black => self.pieces[point_to_index((from.0, 4))] = None,
                            Color::White => self.pieces[point_to_index((from.0, 3))] = None,
                        }
                    }
                }
            }
            self.pieces[point_to_index(piece_selected)]
                .as_mut()
                .unwrap()
                .on_moved(&mut self.en_passant_square, &piece_selected, &from);
            self.pieces[point_to_index(from)] = None;
            self.pieces
                .swap(point_to_index(piece_selected), point_to_index(from));
            self.move_squares.clear();
            self.take_squares.clear();
            self.piece_selected = None;
            self.turn = self.turn.other();
            true
        } else {
            self.move_squares.clear();
            self.take_squares.clear();
            if let Some(piece) = piece {
                if piece.color() != &self.turn {
                    return false;
                }
            }
            self.piece_selected = Some(from);
            if let Some(piece) = piece {
                for to in (0..64).map(index_to_point) {
                    if piece.can_move(self, &from, &to) {
                        self.move_squares.push(to);
                    }
                    if piece.can_take(self, &from, &to) {
                        self.take_squares.push(to);
                    }
                }
            }
            false
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            pieces: starting_pieces(),
            piece_selected: None,
            take_squares: Vec::new(),
            move_squares: Vec::new(),
            en_passant_square: None,
            turn: Color::White,
        }
    }
}

pub fn index_to_point(index: usize) -> (usize, usize) {
    (index % 8, index / 8)
}
pub fn point_to_index((x, y): (usize, usize)) -> usize {
    x + y * 8
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Piece {
    Pawn(Pawn),
    Knight(Knight),
    Bishop(Bishop),
    Rook(Rook),
    Queen(Queen),
    King(King),
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum Color {
    Black,
    #[default]
    White,
}

impl Color {
    pub fn name(&self) -> String {
        match self {
            Self::Black => "black",
            Self::White => "white",
        }
        .to_string()
    }

    pub fn other(&self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}

impl Piece {
    pub fn class_name(&self) -> String {
        match self {
            Piece::Pawn(p) => format!("{}_pawn", p.color.name()),
            Piece::Knight(n) => format!("{}_knight", n.color.name()),
            Piece::Bishop(b) => format!("{}_bishop", b.color.name()),
            Piece::Rook(r) => format!("{}_rook", r.color.name()),
            Piece::Queen(q) => format!("{}_queen", q.color.name()),
            Piece::King(k) => format!("{}_king", k.color.name()),
        }
    }

    pub fn color(&self) -> &Color {
        match self {
            Piece::Pawn(p) => &p.color,
            Piece::Knight(n) => &n.color,
            Piece::Bishop(b) => &b.color,
            Piece::Rook(r) => &r.color,
            Piece::Queen(q) => &q.color,
            Piece::King(k) => &k.color,
        }
    }

    pub fn can_take(&self, board: &Board, from: &(usize, usize), to: &(usize, usize)) -> bool {
        if let (Some(piece1), Some(piece2)) = (board.get(from), board.get(to)) {
            if piece1.color() == piece2.color() {
                return false;
            }
            if &board.turn != piece1.color() {
                return false;
            }
        } else {
            // handle en passant case
            if let Piece::Pawn(p) = self {
                if let Some(pos) = board.en_passant_square {
                    if pos == *to {
                        match p.color {
                            Color::Black if to.1 != 5 => return false,
                            Color::White if to.1 != 2 => return false,
                            _ => return p.can_take(board, from, to),
                        }
                    }
                }
            }
            return false;
        }
        match self {
            Piece::Pawn(p) => p.can_take(board, from, to),
            Piece::Knight(n) => n.can_take(board, from, to),
            Piece::Bishop(b) => b.can_take(board, from, to),
            Piece::Rook(r) => r.can_take(board, from, to),
            Piece::Queen(q) => q.can_take(board, from, to),
            Piece::King(k) => k.can_take(board, from, to),
        }
    }

    pub fn can_move(&self, board: &Board, from: &(usize, usize), to: &(usize, usize)) -> bool {
        if let (Some(piece1), None) = (board.get(from), board.get(to)) {
            if &board.turn != piece1.color() {
                return false;
            }
        } else {
            return false;
        }
        match self {
            Piece::Pawn(p) => p.can_move(board, from, to),
            Piece::Knight(n) => n.can_move(board, from, to),
            Piece::Bishop(b) => b.can_move(board, from, to),
            Piece::Rook(r) => r.can_move(board, from, to),
            Piece::Queen(q) => q.can_move(board, from, to),
            Piece::King(k) => k.can_move(board, from, to),
        }
    }

    pub fn on_moved(
        &mut self,
        en_passant_square: &mut Option<(usize, usize)>,
        from: &(usize, usize),
        to: &(usize, usize),
    ) {
        // en passant is a immediate reaction.
        *en_passant_square = None;
        match self {
            Piece::King(k) => k.has_moved = true,
            Piece::Rook(r) => r.has_moved = true,
            Piece::Pawn(p) => {
                // mark en passant tile
                match p.color {
                    Color::Black => {
                        if from.1 == 1 && to.1 == 3 {
                            *en_passant_square = Some((from.0, 2));
                        }
                    }
                    Color::White => {
                        if from.1 == 6 && to.1 == 4 {
                            *en_passant_square = Some((from.0, 5));
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

pub trait Moves {
    fn can_take(&self, board: &Board, from: &(usize, usize), to: &(usize, usize)) -> bool;
    fn can_move(&self, board: &Board, from: &(usize, usize), to: &(usize, usize)) -> bool;
}

pub trait Same {
    fn can(&self, board: &Board, from: &(usize, usize), to: &(usize, usize)) -> bool;
}

impl<T: Same> Moves for T {
    fn can_move(&self, board: &Board, from: &(usize, usize), to: &(usize, usize)) -> bool {
        self.can(board, from, to)
    }

    fn can_take(&self, board: &Board, from: &(usize, usize), to: &(usize, usize)) -> bool {
        self.can(board, from, to)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct Pawn {
    color: Color,
}

impl Moves for Pawn {
    fn can_move(&self, _board: &Board, from: &(usize, usize), to: &(usize, usize)) -> bool {
        match self.color {
            Color::White => {
                (to.1 + 1 == from.1 && to.0 == from.0)
                    || (from.1 == 6 && (to.1 + 2 == from.1 && to.0 == from.0))
            }
            Color::Black => {
                (to.1 == from.1 + 1 && to.0 == from.0)
                    || (from.1 == 1 && (to.1 == from.1 + 2 && to.0 == from.0))
            }
        }
    }

    fn can_take(&self, _board: &Board, from: &(usize, usize), to: &(usize, usize)) -> bool {
        match self.color {
            Color::White => (to.1 + 1 == from.1) && (to.0 + 1 == from.0 || to.0 == from.0 + 1),
            Color::Black => (to.1 == from.1 + 1) && (to.0 + 1 == from.0 || to.0 == from.0 + 1),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct Knight {
    color: Color,
}

impl Same for Knight {
    fn can(&self, _board: &Board, from: &(usize, usize), to: &(usize, usize)) -> bool {
        ((from.0 == to.0 + 2 || from.0 + 2 == to.0) && (from.1 == to.1 + 1 || from.1 + 1 == to.1))
            || ((from.1 == to.1 + 2 || from.1 + 2 == to.1)
                && (from.0 == to.0 + 1 || from.0 + 1 == to.0))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct Bishop {
    color: Color,
}

impl Same for Bishop {
    fn can(&self, board: &Board, from: &(usize, usize), to: &(usize, usize)) -> bool {
        if from == to {
            return false;
        }
        let dx = to.0 as isize - from.0 as isize;
        let dy = to.1 as isize - from.1 as isize;
        if dx.abs() != dy.abs() {
            return false;
        }

        for i in 1..dx.abs() {
            if board
                .get(&(
                    (from.0 as isize + i * dx.signum()) as usize,
                    (from.1 as isize + i * dy.signum()) as usize,
                ))
                .is_some()
            {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct Rook {
    color: Color,
    has_moved: bool,
}

impl Same for Rook {
    fn can(&self, board: &Board, from: &(usize, usize), to: &(usize, usize)) -> bool {
        if from == to {
            return false;
        }
        let mov_x = to.0 != from.0;
        let mov_y = to.1 != from.1;
        if mov_x && mov_y {
            return false;
        }
        let dir_x = (to.0 as isize - from.0 as isize).signum();
        let dir_y = (to.1 as isize - from.1 as isize).signum();

        if mov_x {
            // se mueve en x
            for p in 1..(to.0 as isize - from.0 as isize).abs() {
                if board
                    .get(&((from.0 as isize + p * dir_x) as usize, from.1))
                    .is_some()
                {
                    return false;
                }
            }
        } else if mov_y {
            // se mueve en y
            for p in 1..(to.1 as isize - from.1 as isize).abs() {
                if board
                    .get(&(from.0, (from.1 as isize + p * dir_y) as usize))
                    .is_some()
                {
                    return false;
                }
            }
        }
        true
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct Queen {
    color: Color,
}

impl Same for Queen {
    fn can(&self, board: &Board, from: &(usize, usize), to: &(usize, usize)) -> bool {
        Bishop::can(&Default::default(), board, from, to)
            || Rook::can(&Default::default(), board, from, to)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct King {
    color: Color,
    has_moved: bool,
}

impl Same for King {
    fn can(&self, _board: &Board, from: &(usize, usize), to: &(usize, usize)) -> bool {
        if from == to {
            return false;
        }
        let dx = from.0 as isize - to.0 as isize;
        let dy = from.1 as isize - to.1 as isize;
        let dx = dx.abs();
        let dy = dy.abs();
        dx < 2 && dy < 2
    }
}

colored! { Pawn }
colored! { Knight }
colored! { Bishop }
colored! { Rook }
colored! { King }
colored! { Queen }

#[macro_export]
macro_rules! colored {
    ($t:ty) => {
        #[allow(clippy::needless_update)]
        impl $t {
            pub fn black() -> Self {
                Self {
                    color: Color::Black,
                    ..Default::default()
                }
            }

            pub fn white() -> Self {
                Self {
                    color: Color::White,
                    ..Default::default()
                }
            }
        }
    };
}
