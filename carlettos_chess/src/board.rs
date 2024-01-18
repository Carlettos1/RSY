use std::{
    mem,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use serde::{Deserialize, Serialize};

use crate::{card::Cards, piece::Piece, Color, Pos};

use self::shape::Shape;

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone)]
pub struct Mana(pub usize);

impl Add for Mana {
    type Output = Mana;
    fn add(self, rhs: Self) -> Self::Output {
        Mana(self.0 + rhs.0)
    }
}

impl Add for &Mana {
    type Output = Mana;
    fn add(self, rhs: Self) -> Self::Output {
        Mana(self.0 + rhs.0)
    }
}

impl AddAssign for Mana {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Mana {
    type Output = Mana;
    fn sub(self, rhs: Self) -> Self::Output {
        Mana(self.0.checked_sub(rhs.0).unwrap_or_default())
    }
}

impl Sub for &Mana {
    type Output = Mana;
    fn sub(self, rhs: Self) -> Self::Output {
        Mana(self.0.checked_sub(rhs.0).unwrap_or_default())
    }
}

impl SubAssign for Mana {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0.min(self.0);
    }
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone)]
pub struct Movements(pub usize);

impl Add for Movements {
    type Output = Movements;
    fn add(self, rhs: Self) -> Self::Output {
        Movements(self.0 + rhs.0)
    }
}

impl Add for &Movements {
    type Output = Movements;
    fn add(self, rhs: Self) -> Self::Output {
        Movements(self.0 + rhs.0)
    }
}

impl AddAssign for Movements {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Movements {
    type Output = Movements;
    fn sub(self, rhs: Self) -> Self::Output {
        Movements(self.0.checked_sub(rhs.0).unwrap_or_default())
    }
}

impl Sub for &Movements {
    type Output = Movements;
    fn sub(self, rhs: Self) -> Self::Output {
        Movements(self.0.checked_sub(rhs.0).unwrap_or_default())
    }
}

impl SubAssign for Movements {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0.min(self.0);
    }
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone)]
pub struct Player {
    pub movements: Movements,
    pub mana: Mana,
    pub hand: Cards,
    deck: Cards,
    discard_pile: Cards,
    id: usize,
    color: Color,
}

impl Player {
    pub fn new(color: Color, id: usize, deck: Cards) -> Self {
        Player {
            movements: Movements(1),
            mana: Mana(0),
            id,
            color,
            deck,
            ..Default::default()
        }
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn id(&self) -> &usize {
        &self.id
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Tile {
    pub magic: bool,
    pub buildable: bool,
    pub piece: Piece,
    pos: Pos,
}

impl Tile {
    pub fn new(pos: Pos) -> Self {
        Tile {
            magic: false,
            buildable: true,
            piece: Piece::None,
            pos,
        }
    }

    pub fn get_color(&self) -> Option<&Color> {
        self.piece.color()
    }

    pub fn is_controlled_by(&self, color: &Color) -> bool {
        self.get_color() == Some(color)
    }

    pub fn is_empty(&self) -> bool {
        matches!(self.piece, Piece::None)
    }

    pub fn has_piece(&self) -> bool {
        !self.is_empty()
    }

    pub fn replace(&mut self, piece: Piece) -> Piece {
        mem::replace(&mut self.piece, piece)
    }

    pub fn remove(&mut self) -> Piece {
        self.replace(Piece::None)
    }
}

pub mod shape {
    use serde::{Deserialize, Serialize};

    use crate::Pos;

    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct Square {
        // This is the south west point
        pub anchor: Pos,
        pub width: usize,
        pub height: usize,
    }

    impl Square {
        pub fn north(&self) -> usize {
            self.anchor.y + self.height
        }
        pub fn east(&self) -> usize {
            self.anchor.x + self.width
        }
        pub fn south(&self) -> usize {
            self.anchor.y
        }
        pub fn west(&self) -> usize {
            self.anchor.x
        }

        pub fn ne_point(&self) -> Pos {
            &self.anchor + &Pos::new(self.width, self.height)
        }

        pub fn se_point(&self) -> Pos {
            &self.anchor + &Pos::new(self.width, 0)
        }

        pub fn sw_point(&self) -> Pos {
            self.anchor.clone()
        }

        pub fn nw_point(&self) -> Pos {
            &self.anchor + &Pos::new(0, self.height)
        }

        pub fn contains(&self, pos: &Pos) -> bool {
            pos.x >= self.west()
                && pos.x < self.east()
                && pos.y >= self.south()
                && pos.y < self.north()
        }

        pub fn points_iter(&self) -> impl Iterator<Item = Pos> + '_ {
            (self.west()..self.west() + self.width).flat_map(|x| {
                (self.south()..self.south() + self.height).map(move |y| Pos::new(x, y))
            })
        }
    }

    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct Shape {
        pub squares: Vec<Square>,
    }

    impl Shape {
        pub fn new(squares: Vec<Square>) -> Self {
            Self { squares }
        }

        pub fn default_chessboard() -> Self {
            let square = Square {
                anchor: Pos::new(0, 0),
                width: 8,
                height: 8,
            };
            Shape::new(vec![square])
        }

        pub fn cross_shape() -> Self {
            let squares = vec![
                Square {
                    anchor: Pos::new(2, 0),
                    width: 4,
                    height: 2,
                },
                Square {
                    anchor: Pos::new(0, 2),
                    width: 2,
                    height: 4,
                },
                Square {
                    anchor: Pos::new(2, 2),
                    width: 4,
                    height: 4,
                },
                Square {
                    anchor: Pos::new(2, 6),
                    width: 4,
                    height: 2,
                },
                Square {
                    anchor: Pos::new(6, 2),
                    width: 2,
                    height: 4,
                },
            ];
            Shape::new(squares)
        }

        pub fn contains(&self, pos: &Pos) -> bool {
            self.squares.iter().any(|square| square.contains(pos))
        }

        pub fn points_iter(&self) -> impl Iterator<Item = Pos> + '_ {
            self.squares.iter().flat_map(|s| s.points_iter())
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Board {
    pub tiles: Vec<Tile>,
    players: Vec<Player>,
    cards: Cards,
    dead_pieces: Vec<Piece>,
    shape: Shape,
}

impl Board {
    pub fn with_shape(shape: Shape) -> Self {
        Self {
            tiles: shape.points_iter().map(Tile::new).collect(),
            dead_pieces: Vec::new(),
            shape,
            players: vec![
                Player::new(Color::White, 0, Cards::default()),
                Player::new(Color::Black, 1, Cards::default()),
            ],
            cards: Cards::default(),
        }
    }

    pub fn with_default_players(tiles: Vec<Tile>, shape: Shape) -> Self {
        Self {
            tiles,
            dead_pieces: Vec::new(),
            shape,
            players: vec![
                Player::new(Color::White, 0, Cards::default()),
                Player::new(Color::Black, 1, Cards::default()),
            ],
            cards: Cards::default(),
        }
    }

    pub fn with_empty_tiles(shape: Shape, players: Vec<Player>) -> Self {
        Self {
            tiles: shape.points_iter().map(Tile::new).collect(),
            dead_pieces: Vec::new(),
            shape,
            players,
            cards: Cards::default(),
        }
    }

    pub fn new(tiles: Vec<Tile>, shape: Shape, players: Vec<Player>) -> Self {
        Self {
            tiles,
            dead_pieces: Vec::new(),
            shape,
            players,
            cards: Cards::default(),
        }
    }

    pub fn get_last_dead(&self) -> Option<&Piece> {
        self.dead_pieces.last()
    }

    pub fn remove_last_dead(&mut self) -> Piece {
        self.dead_pieces.pop().unwrap_or_default()
    }

    pub fn get_last_dead_with_color(&self, color: &Color) -> Option<&Piece> {
        self.dead_pieces
            .iter()
            .rev()
            .find(|p| p.color() == Some(color))
    }

    pub fn remove_last_dead_with_color(&mut self, color: &Color) -> Piece {
        let last = self
            .dead_pieces
            .iter()
            .rev()
            .position(|x| x.color() == Some(color));
        match last {
            None => Piece::None,
            Some(i) => self.dead_pieces.remove(i),
        }
    }

    pub fn contains(&self, pos: &Pos) -> bool {
        self.shape.contains(pos)
    }

    pub fn get(&self, pos: &Pos) -> Option<&Tile> {
        self.tiles.iter().find(|t| &t.pos == pos)
    }

    pub fn get_mut(&mut self, pos: &Pos) -> Option<&mut Tile> {
        self.tiles.iter_mut().find(|t| &t.pos == pos)
    }

    pub fn is_empty(&self, pos: &Pos) -> bool {
        match self.get(pos) {
            None => false,
            Some(tile) => tile.is_empty(),
        }
    }

    pub fn has_piece(&self, pos: &Pos) -> bool {
        match self.get(pos) {
            None => false,
            Some(tile) => tile.has_piece(),
        }
    }

    pub fn shape(&self) -> &Shape {
        &self.shape
    }
}

impl Default for Board {
    fn default() -> Self {
        let shape = Shape::default_chessboard();
        Self {
            tiles: shape.points_iter().map(Tile::new).collect(),
            dead_pieces: Vec::new(),
            shape,
            players: vec![
                Player::new(Color::White, 0, Cards::default()),
                Player::new(Color::Black, 1, Cards::default()),
            ],
            cards: Cards::default(),
        }
    }
}
