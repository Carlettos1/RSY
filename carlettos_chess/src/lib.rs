use std::ops::{Add, AddAssign, Sub};

use piece::Piece;
use serde::{Deserialize, Serialize};

pub mod ability;
pub mod board;
pub mod card;
pub mod chess_controller;
pub mod pattern;
pub mod piece;
pub mod prelude;

/// Represents a position on a chessboard.
///
/// The `Pos` struct provides methods for manipulating and calculating positions on a chessboard.
/// Positions are represented by their x and y coordinates, where (0, 0) represents the bottom-left corner of the board.
///
/// # Examples
///
/// ```
/// use carlettos_chess::Pos;
///
/// let pos = Pos::new(3, 4);
/// assert_eq!(pos.x, 3);
/// assert_eq!(pos.y, 4);
///
/// let north_pos = pos.north();
/// assert_eq!(north_pos.unwrap().y, 5);
///
/// let east_pos = pos.east();
/// assert_eq!(east_pos.unwrap().x, 4);
///
/// let south_pos = pos.south();
/// assert_eq!(south_pos.unwrap().y, 3);
///
/// let west_pos = pos.west();
/// assert_eq!(west_pos.unwrap().x, 2);
/// ```
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

impl PartialEq<Option<&Pos>> for Pos {
    fn eq(&self, other: &Option<&Pos>) -> bool {
        match other {
            None => false,
            Some(other) => other == &self,
        }
    }
}

impl PartialEq<Option<Pos>> for Pos {
    fn eq(&self, other: &Option<Pos>) -> bool {
        match other {
            None => false,
            Some(other) => other == self,
        }
    }
}

impl Pos {
    /// Creates a new `Pos` with the specified x and y coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate of the position.
    /// * `y` - The y coordinate of the position.
    ///
    /// # Examples
    ///
    /// ```
    /// use carlettos_chess::Pos;
    ///
    /// let pos = Pos::new(3, 4);
    /// assert_eq!(pos.x, 3);
    /// assert_eq!(pos.y, 4);
    /// ```
    #[inline(always)]
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Returns the position to the north of the current position.
    ///
    /// If the resulting position is greater than usize::MAX, `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use carlettos_chess::Pos;
    ///
    /// let pos = Pos::new(3, 4);
    /// let north_pos = pos.north();
    /// assert_eq!(north_pos.unwrap().y, 5);
    /// ```
    #[inline(always)]
    pub fn north(&self) -> Option<Self> {
        self.shift(0, 1)
    }

    /// Returns the position to the east of the current position.
    ///
    /// If the resulting position is greater than usize::MAX, `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use carlettos_chess::Pos;
    ///
    /// let pos = Pos::new(3, 4);
    /// let east_pos = pos.east();
    /// assert_eq!(east_pos.unwrap().x, 4);
    /// ```
    #[inline(always)]
    pub fn east(&self) -> Option<Self> {
        self.shift(1, 0)
    }

    /// Returns the position to the south of the current position.
    ///
    /// If the resulting position is smaller than 0, `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use carlettos_chess::Pos;
    ///
    /// let pos = Pos::new(3, 4);
    /// let south_pos = pos.south();
    /// assert_eq!(south_pos.unwrap().y, 3);
    /// ```
    #[inline(always)]
    pub fn south(&self) -> Option<Self> {
        self.shift(0, -1)
    }

    /// Returns the position to the west of the current position.
    ///
    /// If the resulting position is smaller than 0, `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use carlettos_chess::Pos;
    ///
    /// let pos = Pos::new(3, 4);
    /// let west_pos = pos.west();
    /// assert_eq!(west_pos.unwrap().x, 2);
    /// ```
    #[inline(always)]
    pub fn west(&self) -> Option<Self> {
        self.shift(-1, 0)
    }

    /// Returns a new position that is shifted by the specified x and y offsets, if the resulting position is within `usize::MIN` and `usize::MAX`.
    ///
    /// # Arguments
    ///
    /// * `x` - The x offset to apply to the current position.
    /// * `y` - The y offset to apply to the current position.
    ///
    /// # Examples
    ///
    /// ```
    /// use carlettos_chess::Pos;
    ///
    /// let pos = Pos::new(3, 4);
    /// let shifted_pos = pos.shift(2, -1);
    /// assert_eq!(shifted_pos.as_ref().unwrap().x, 5);
    /// assert_eq!(shifted_pos.as_ref().unwrap().y, 3);
    /// ```
    #[inline(always)]
    pub fn shift(&self, x: isize, y: isize) -> Option<Self> {
        let dx = self.x.checked_add_signed(x);
        let dy = self.y.checked_add_signed(y);
        match (dx, dy) {
            (Some(dx), Some(dy)) => Some(Self::new(dx, dy)),
            _ => None,
        }
    }

    /// Returns the absolute difference between the current position and the specified position.
    ///
    /// # Arguments
    ///
    /// * `other` - The other position to calculate the absolute difference with.
    ///
    /// # Examples
    ///
    /// ```
    /// use carlettos_chess::Pos;
    ///
    /// let pos1 = Pos::new(3, 4);
    /// let pos2 = Pos::new(6, 2);
    /// let diff = pos1.abs_diff(&pos2);
    /// assert_eq!(diff.x, 3);
    /// assert_eq!(diff.y, 2);
    /// ```
    #[inline(always)]
    pub fn abs_diff(&self, Pos { x, y }: &Pos) -> Pos {
        Pos {
            x: self.x.abs_diff(*x),
            y: self.y.abs_diff(*y),
        }
    }

    /// Returns the position obtained by shifting the current position in the specified direction.
    ///
    /// # Arguments
    ///
    /// * `direction` - The direction in which to shift the position.
    ///
    /// # Examples
    ///
    /// ```
    /// use carlettos_chess::{Pos, Direction};
    ///
    /// let pos = Pos::new(3, 4);
    /// let north_pos = pos.direction_shift(&Direction::N);
    /// assert_eq!(north_pos.unwrap().y, 5);
    /// ```
    #[inline(always)]
    pub fn direction_shift(&self, direction: &Direction) -> Option<Self> {
        let (dx, dy) = direction.into();
        self.shift(dx, dy)
    }

    /// Returns the position obtained by shifting the current position in the specified subdirection.
    ///
    /// # Arguments
    ///
    /// * `subdirection` - The subdirection in which to shift the position.
    ///
    /// # Examples
    ///
    /// ```
    /// use carlettos_chess::{Pos, SubDirection};
    ///
    /// let pos = Pos::new(3, 4);
    /// let north_pos = pos.subdirection_shift(&SubDirection::N);
    /// assert_eq!(north_pos.unwrap().y, 5);
    /// ```
    #[inline(always)]
    pub fn subdirection_shift(&self, subdirection: &SubDirection) -> Option<Self> {
        let (dx, dy) = subdirection.into();
        self.shift(dx, dy)
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

impl From<&Direction> for (isize, isize) {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::N => (0, 1),
            Direction::E => (1, 0),
            Direction::S => (0, -1),
            Direction::W => (-1, 0),
        }
    }
}

impl Direction {
    pub fn is_axis(&self, axis: &Axis) -> bool {
        matches!(
            (self, axis),
            (Direction::N, Axis::NS)
                | (Direction::S, Axis::NS)
                | (Direction::E, Axis::EW)
                | (Direction::W, Axis::EW)
        )
    }

    pub fn into_subdirection(&self) -> SubDirection {
        match self {
            Direction::N => SubDirection::N,
            Direction::E => SubDirection::E,
            Direction::S => SubDirection::S,
            Direction::W => SubDirection::W,
        }
    }
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

impl From<&SubDirection> for (isize, isize) {
    fn from(value: &SubDirection) -> Self {
        match value {
            SubDirection::N => (0, 1),
            SubDirection::NE => (1, 1),
            SubDirection::E => (1, 0),
            SubDirection::SE => (1, -1),
            SubDirection::S => (0, -1),
            SubDirection::SW => (-1, -1),
            SubDirection::W => (-1, 0),
            SubDirection::NW => (-1, 1),
        }
    }
}

impl SubDirection {
    pub fn is_direction(&self, direction: &Direction) -> bool {
        matches!(
            (self, direction),
            (SubDirection::N, Direction::N)
                | (SubDirection::NE, Direction::N)
                | (SubDirection::NE, Direction::E)
                | (SubDirection::E, Direction::E)
                | (SubDirection::SE, Direction::E)
                | (SubDirection::SE, Direction::S)
                | (SubDirection::S, Direction::S)
                | (SubDirection::SW, Direction::S)
                | (SubDirection::SW, Direction::W)
                | (SubDirection::W, Direction::W)
                | (SubDirection::NW, Direction::W)
                | (SubDirection::NW, Direction::N)
        )
    }

    pub fn into_direction(&self) -> Option<Direction> {
        match self {
            SubDirection::N => Some(Direction::N),
            SubDirection::E => Some(Direction::E),
            SubDirection::S => Some(Direction::S),
            SubDirection::W => Some(Direction::W),
            _ => None,
        }
    }
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
    pub fn r#move(from: &Pos, to: &Pos) -> Self {
        Self::Move {
            from: from.clone(),
            to: to.clone(),
        }
    }

    pub fn take(from: &Pos, to: &Pos) -> Self {
        Self::Take {
            from: from.clone(),
            to: to.clone(),
        }
    }

    pub fn attack(from: &Pos, to: &Pos) -> Self {
        Self::Attack {
            from: from.clone(),
            to: to.clone(),
        }
    }

    pub fn ability(from: &Pos, info: Info) -> Self {
        Self::Ability {
            from: from.clone(),
            info,
        }
    }

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
    Direction(Direction),
    Pos(Pos),
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

        assert_eq!(pos.shift(4, 5), Some(Pos::new(5, 7)));
        assert_eq!(pos.shift(1, 1), Some(Pos::new(2, 3)));
        assert_eq!(pos.shift(8, 1), Some(Pos::new(9, 3)));
        assert_eq!(pos.shift(-1, 2), Some(Pos::new(0, 4)));
        assert_eq!(pos.shift(0, -3), None);
        assert_eq!(pos.shift(-2, -1), None);
    }

    #[test]
    fn pos_edges() {
        let pos0: Pos = (0, 0).into();
        let posmax = Pos::new(usize::MAX, usize::MAX);

        assert_eq!(posmax.abs_diff(&pos0), posmax);
        assert_eq!(pos0.abs_diff(&posmax), posmax);
        assert_eq!(pos0.abs_diff(&posmax), posmax.abs_diff(&pos0));
        assert_eq!(posmax.north(), None);

        assert_eq!(pos0.shift(0, 1), Some(Pos::new(0, 1)));
        assert_eq!(pos0.shift(1, 0), Some(Pos::new(1, 0)));
        assert_eq!(pos0.shift(0, -1), None);
        assert_eq!(pos0.shift(-1, 0), None);

        assert_eq!(posmax.shift(0, 1), None);
        assert_eq!(posmax.shift(1, 0), None);
        assert_eq!(
            posmax.shift(0, -1),
            Some(Pos::new(usize::MAX, usize::MAX - 1))
        );
        assert_eq!(
            posmax.shift(-1, 0),
            Some(Pos::new(usize::MAX - 1, usize::MAX))
        );
    }
}
