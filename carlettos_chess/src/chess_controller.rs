//!
//! This module contains the main struct that will be used to control the game.
use serde::{Deserialize, Serialize};

use crate::{
    board::{
        shape::{Shape, Square},
        Board, Tile,
    },
    piece::Piece,
    Action, Color, Pos,
};

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CChess {
    pub board: Board,
    pub selected: Option<Pos>,
    pub moves: Vec<Pos>,
    pub takes: Vec<Pos>,
    pub attacks: Vec<Pos>,
    pub abilities: Vec<Pos>,
}

impl CChess {
    ///
    /// This is a function to safely click on the board.
    /// if the position is not valid, it will make nothing.
    /// And it will use inner methods to handle the click.
    /// Return true or false if the state had tried to change.
    pub fn click(&mut self, click_pos: Pos) -> bool {
        if !self.board.contains(&click_pos) {
            return false;
        }

        let click_tile = match self.board.get(&click_pos) {
            None => return false,
            Some(tile) => tile,
        };

        match &self.selected {
            None => {
                let piece = &click_tile.piece;
                for other_pos in self.board.shape().points_iter() {
                    if other_pos == click_pos {
                        continue;
                    }
                    let move_action = Action::r#move(&click_pos, &other_pos);
                    let take_action = Action::take(&click_pos, &other_pos);
                    let attack_action = Action::attack(&click_pos, &other_pos);
                    // TODO: add abilities
                    if piece.can_do(&self.board, move_action) && self.board.is_empty(&other_pos) {
                        self.moves.push(other_pos.clone());
                    }
                    if piece.can_do(&self.board, take_action)
                        && self.board.has_piece(&other_pos)
                        && !self.board.same_color(&click_pos, &other_pos)
                    {
                        self.takes.push(other_pos.clone());
                    }
                    if piece.can_do(&self.board, attack_action)
                        && self.board.has_piece(&other_pos)
                        && !self.board.same_color(&click_pos, &other_pos)
                    {
                        self.attacks.push(other_pos.clone());
                    }
                }
                self.selected = Some(click_pos);
            }
            Some(selected_pos) => {
                if self.attacks.contains(&click_pos) {
                    self.board.make(Action::attack(selected_pos, &click_pos));
                } else if self.takes.contains(&click_pos) {
                    self.board.make(Action::take(selected_pos, &click_pos));
                } else if self.moves.contains(&click_pos) {
                    self.board.make(Action::r#move(selected_pos, &click_pos));
                }
                //TODO: handle ability
                self.clear();
            }
        }
        true
    }

    fn clear(&mut self) {
        self.moves.clear();
        self.takes.clear();
        self.attacks.clear();
        self.abilities.clear();
        self.selected = None;
    }

    pub fn actions_at(&self, pos: &Pos) -> Vec<Action> {
        let mut actions = Vec::with_capacity(4);
        match (self.board.get(pos), &self.selected) {
            (Some(tile), Some(selected)) => {
                if self.moves.contains(tile.pos()) {
                    actions.push(Action::r#move(selected, tile.pos()));
                }
                if self.takes.contains(tile.pos()) {
                    actions.push(Action::take(selected, tile.pos()));
                }
                if self.attacks.contains(tile.pos()) {
                    actions.push(Action::attack(selected, tile.pos()));
                }
                actions
                // TODO: handle ability
            }
            _ => actions,
        }
    }

    pub fn default_display() -> Self {
        let mut board = Board::with_shape(Shape::new(vec![Square {
            anchor: Pos::new(0, 0),
            height: 2,
            width: 30,
        }]));
        board
            .get_mut(&Pos::new(0, 0))
            .unwrap()
            .replace(Piece::pawn(Color::White));
        board
            .get_mut(&Pos::new(0, 1))
            .unwrap()
            .replace(Piece::pawn(Color::Black));
        board
            .get_mut(&Pos::new(1, 0))
            .unwrap()
            .replace(Piece::knight(Color::White));
        board
            .get_mut(&Pos::new(1, 1))
            .unwrap()
            .replace(Piece::knight(Color::Black));
        board
            .get_mut(&Pos::new(2, 0))
            .unwrap()
            .replace(Piece::bishop(Color::White));
        board
            .get_mut(&Pos::new(2, 1))
            .unwrap()
            .replace(Piece::bishop(Color::Black));
        board
            .get_mut(&Pos::new(3, 0))
            .unwrap()
            .replace(Piece::rook(Color::White));
        board
            .get_mut(&Pos::new(3, 1))
            .unwrap()
            .replace(Piece::rook(Color::Black));
        board
            .get_mut(&Pos::new(4, 0))
            .unwrap()
            .replace(Piece::queen(Color::White));
        board
            .get_mut(&Pos::new(4, 1))
            .unwrap()
            .replace(Piece::queen(Color::Black));
        board
            .get_mut(&Pos::new(5, 0))
            .unwrap()
            .replace(Piece::king(Color::White));
        board
            .get_mut(&Pos::new(5, 1))
            .unwrap()
            .replace(Piece::king(Color::Black));
        board
            .get_mut(&Pos::new(6, 0))
            .unwrap()
            .replace(Piece::archer(Color::White));
        board
            .get_mut(&Pos::new(6, 1))
            .unwrap()
            .replace(Piece::archer(Color::Black));
        board
            .get_mut(&Pos::new(7, 0))
            .unwrap()
            .replace(Piece::ballista(Color::White));
        board
            .get_mut(&Pos::new(7, 1))
            .unwrap()
            .replace(Piece::ballista(Color::Black));

        let piece = Piece::None;

        // This is just so the compiler forces me to add the new pieces to the display.
        #[allow(unused_variables)]
        match piece {
            Piece::None => {}
            Piece::Pawn(data) => {}
            Piece::Knight(data) => {}
            Piece::Bishop(data) => {}
            Piece::Rook(data) => {}
            Piece::Queen(data) => {}
            Piece::King(data) => {}
            Piece::Archer(data) => {}
            Piece::Ballista(data) => {}
        }
        Self {
            board,
            ..Default::default()
        }
    }

    pub fn default_chessboard() -> Self {
        Self {
            board: Board::default_chessboard(),
            ..Default::default()
        }
    }

    pub fn height(&self) -> usize {
        self.board.height()
    }

    pub fn row_iter(&self, row: usize) -> impl Iterator<Item = &Tile> {
        self.board.row_iter(row)
    }

    pub fn has_move(&self, pos: &Pos) -> bool {
        self.moves.contains(pos)
    }

    pub fn has_take(&self, pos: &Pos) -> bool {
        self.takes.contains(pos)
    }

    pub fn has_attack(&self, pos: &Pos) -> bool {
        self.attacks.contains(pos)
    }
}
