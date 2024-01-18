use std::ops::{Add, AddAssign, Sub};

use board::shape::Square;
use piece::Piece;
use serde::{Deserialize, Serialize};

pub mod ability;
pub mod board;
pub mod card;
pub mod pattern;
pub mod piece;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Pos {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl Pos {
    #[inline(always)]
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub fn north(&self) -> Self {
        self.shift(0, 1)
    }

    #[inline(always)]
    pub fn east(&self) -> Self {
        self.shift(1, 0)
    }

    #[inline(always)]
    pub fn south(&self) -> Option<Self> {
        self.checked_shift(0, -1)
    }

    #[inline(always)]
    pub fn west(&self) -> Option<Self> {
        self.checked_shift(-1, 0)
    }

    #[inline(always)]
    pub fn shift(&self, x: usize, y: usize) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }

    #[inline(always)]
    pub fn checked_shift(&self, x: isize, y: isize) -> Option<Self> {
        let dx = self.x.checked_add_signed(x);
        let dy = self.y.checked_add_signed(y);
        match (dx, dy) {
            (Some(dx), Some(dy)) => Some(Self::new(dx, dy)),
            _ => None,
        }
    }

    pub fn abs_diff(&self, Pos { x, y }: &Pos) -> Pos {
        Pos {
            x: self.x.abs_diff(*x),
            y: self.y.abs_diff(*y),
        }
    }
}

impl Add for Pos {
    type Output = Pos;
    fn add(self, Pos { x, y }: Self) -> Self::Output {
        Pos {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl Add for &Pos {
    type Output = Pos;
    fn add(self, Pos { x, y }: Self) -> Self::Output {
        Pos {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum SubDirection {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Axis {
    NS,
    EW,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Action {
    /// Indicates a moving piece
    Move { from: Pos, to: Pos },
    /// Indicates a piece taking
    Take { from: Pos, to: Pos },
    /// Indicates an attacking piece
    Attack { from: Pos, to: Pos },
    /// Indicates a piece using its ability
    Ability { from: Pos, info: Info },
}

impl Action {
    pub fn is_move(&self) -> bool {
        matches!(self, Self::Move { from: _, to: _ })
    }

    pub fn is_take(&self) -> bool {
        matches!(self, Self::Take { from: _, to: _ })
    }

    pub fn is_attack(&self) -> bool {
        matches!(self, Self::Attack { from: _, to: _ })
    }

    pub fn is_ability(&self) -> bool {
        matches!(self, Self::Ability { from: _, info: _ })
    }
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Time {
    pub round: usize,
    pub turn: usize,
    pub movement: usize,
}

impl Time {
    pub fn new(rounds: usize, turns: usize, movements: usize) -> Self {
        Self {
            round: rounds,
            turn: turns,
            movement: movements,
        }
    }

    pub fn rounds(rounds: usize) -> Self {
        Self {
            round: rounds,
            turn: 0,
            movement: 0,
        }
    }

    pub fn turns(turns: usize) -> Self {
        Self {
            round: 0,
            turn: turns,
            movement: 0,
        }
    }

    pub fn movements(movements: usize) -> Self {
        Self {
            round: 0,
            turn: 0,
            movement: movements,
        }
    }

    pub fn on_round(&mut self) {
        self.round = self.round.checked_sub(1).unwrap_or_default();
    }

    pub fn on_turn(&mut self) {
        self.turn = self.turn.checked_sub(1).unwrap_or_default();
    }

    pub fn on_movement(&mut self) {
        self.movement = self.movement.checked_sub(1).unwrap_or_default();
    }
}

impl Sub for Time {
    type Output = Time;
    fn sub(
        self,
        Time {
            round,
            turn,
            movement,
        }: Self,
    ) -> Self::Output {
        Time {
            round: self.round - round,
            turn: self.turn - turn,
            movement: self.movement - movement,
        }
    }
}

impl Sub for &Time {
    type Output = Time;
    fn sub(
        self,
        Time {
            round,
            turn,
            movement,
        }: Self,
    ) -> Self::Output {
        Time {
            round: self.round - round,
            turn: self.turn - turn,
            movement: self.movement - movement,
        }
    }
}

impl Add for Time {
    type Output = Time;
    fn add(
        self,
        Time {
            round,
            turn,
            movement,
        }: Self,
    ) -> Self::Output {
        Time {
            round: self.round + round,
            turn: self.turn + turn,
            movement: self.movement + movement,
        }
    }
}

impl Add for &Time {
    type Output = Time;
    fn add(
        self,
        Time {
            round,
            turn,
            movement,
        }: Self,
    ) -> Self::Output {
        Time {
            round: self.round + round,
            turn: self.turn + turn,
            movement: self.movement + movement,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    #[default]
    White,
    Black,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Info {
    Piece(Piece),
}

#[cfg(test)]
mod test {
    use crate::Pos;

    #[test]
    fn pos_add() {
        let pos = Pos::new(1, 2);
        let dpos = Pos::new(3, 4);

        assert_eq!(pos + dpos, Pos::new(4, 6));
    }

    #[test]
    fn pos_add_assign() {
        let mut pos = Pos::new(1, 2);
        let dpos = Pos::new(3, 4);

        pos += dpos;
        assert_eq!(pos, Pos::new(4, 6));
    }

    #[test]
    fn pos_shift() {
        let pos = Pos::new(1, 2);

        assert_eq!(pos.shift(2, 4), Pos::new(3, 6));
        assert_eq!(pos.shift(1, 5), Pos::new(2, 7));
        assert_eq!(pos.shift(2, 1), Pos::new(3, 3));
        assert_eq!(pos.shift(3, 10), Pos::new(4, 12));
    }

    #[test]
    fn pos_checked_shift() {
        let pos = Pos::new(1, 2);

        assert_eq!(pos.checked_shift(4, 5), Some(Pos::new(5, 7)));
        assert_eq!(pos.checked_shift(1, 1), Some(Pos::new(2, 3)));
        assert_eq!(pos.checked_shift(8, 1), Some(Pos::new(9, 3)));
        assert_eq!(pos.checked_shift(-1, 2), Some(Pos::new(0, 4)));
        assert_eq!(pos.checked_shift(0, -3), None);
        assert_eq!(pos.checked_shift(-2, -1), None);
    }

    #[test]
    fn pos_edges() {
        let pos0: Pos = (0, 0).into();
        let posmax = Pos::new(usize::MAX, usize::MAX);

        assert_eq!(posmax.abs_diff(&pos0), posmax);
        assert_eq!(pos0.abs_diff(&posmax), posmax);
        assert_eq!(pos0.abs_diff(&posmax), posmax.abs_diff(&pos0));
        assert_eq!(posmax.north().east(), pos0);

        assert_eq!(pos0.checked_shift(0, 1), Some(Pos::new(0, 1)));
        assert_eq!(pos0.checked_shift(1, 0), Some(Pos::new(1, 0)));
        assert_eq!(pos0.checked_shift(0, -1), None);
        assert_eq!(pos0.checked_shift(-1, 0), None);

        assert_eq!(posmax.checked_shift(0, 1), None);
        assert_eq!(posmax.checked_shift(1, 0), None);
        assert_eq!(
            posmax.checked_shift(0, -1),
            Some(Pos::new(usize::MAX, usize::MAX - 1))
        );
        assert_eq!(
            posmax.checked_shift(-1, 0),
            Some(Pos::new(usize::MAX - 1, usize::MAX))
        );
    }
}
