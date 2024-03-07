use std::{
    mem,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::{
    card::{Card, Cards},
    pattern,
    piece::{Effect, Piece, PieceData, Type},
    Action, Color, Pos, Time,
};

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

    pub fn take_from_deck(&mut self) -> Result<(), EventFunctionError> {
        match self.deck.take() {
            Some(card) => {
                self.hand.add(card);
                Ok(())
            }
            None => Err(EventFunctionError::EmptyDeck),
        }
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

    pub fn pos(&self) -> &Pos {
        &self.pos
    }

    pub fn has_pawn(&self) -> bool {
        matches!(self.piece, Piece::Pawn(_))
    }

    pub fn has_knight(&self) -> bool {
        matches!(self.piece, Piece::Knight(_))
    }

    pub fn has_bishop(&self) -> bool {
        matches!(self.piece, Piece::Bishop(_))
    }

    pub fn has_rook(&self) -> bool {
        matches!(self.piece, Piece::Rook(_))
    }

    pub fn has_queen(&self) -> bool {
        matches!(self.piece, Piece::Queen(_))
    }

    pub fn has_king(&self) -> bool {
        matches!(self.piece, Piece::King(_))
    }

    pub fn has_archer(&self) -> bool {
        matches!(self.piece, Piece::Archer(_))
    }

    pub fn has_ballista(&self) -> bool {
        matches!(self.piece, Piece::Ballista(_))
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

        pub fn height(&self) -> usize {
            self.squares
                .iter()
                .map(|s| s.north())
                .max()
                .unwrap_or_default()
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Board {
    pub tiles: Vec<Tile>,
    pub rng: BoardRng,
    pub time: Time,
    players: Vec<Player>,
    cards: Cards,
    dead_pieces: Vec<Piece>,
    shape: Shape,
    events: Events,
}

impl Board {
    pub fn default_chessboard() -> Self {
        let mut default = Self::default();
        default
            .get_mut(&Pos::new(0, 0))
            .unwrap()
            .replace(Piece::rook(Color::White));
        default
            .get_mut(&Pos::new(1, 0))
            .unwrap()
            .replace(Piece::knight(Color::White));
        default
            .get_mut(&Pos::new(2, 0))
            .unwrap()
            .replace(Piece::bishop(Color::White));
        default
            .get_mut(&Pos::new(3, 0))
            .unwrap()
            .replace(Piece::queen(Color::White));
        default
            .get_mut(&Pos::new(4, 0))
            .unwrap()
            .replace(Piece::king(Color::White));
        default
            .get_mut(&Pos::new(5, 0))
            .unwrap()
            .replace(Piece::bishop(Color::White));
        default
            .get_mut(&Pos::new(6, 0))
            .unwrap()
            .replace(Piece::knight(Color::White));
        default
            .get_mut(&Pos::new(7, 0))
            .unwrap()
            .replace(Piece::rook(Color::White));
        default
            .get_mut(&Pos::new(0, 1))
            .unwrap()
            .replace(Piece::pawn(Color::White));
        default
            .get_mut(&Pos::new(1, 1))
            .unwrap()
            .replace(Piece::pawn(Color::White));
        default
            .get_mut(&Pos::new(2, 1))
            .unwrap()
            .replace(Piece::pawn(Color::White));
        default
            .get_mut(&Pos::new(3, 1))
            .unwrap()
            .replace(Piece::pawn(Color::White));
        default
            .get_mut(&Pos::new(4, 1))
            .unwrap()
            .replace(Piece::pawn(Color::White));
        default
            .get_mut(&Pos::new(5, 1))
            .unwrap()
            .replace(Piece::pawn(Color::White));
        default
            .get_mut(&Pos::new(6, 1))
            .unwrap()
            .replace(Piece::pawn(Color::White));
        default
            .get_mut(&Pos::new(7, 1))
            .unwrap()
            .replace(Piece::pawn(Color::White));
        default
            .get_mut(&Pos::new(0, 6))
            .unwrap()
            .replace(Piece::pawn(Color::Black));
        default
            .get_mut(&Pos::new(1, 6))
            .unwrap()
            .replace(Piece::pawn(Color::Black));
        default
            .get_mut(&Pos::new(2, 6))
            .unwrap()
            .replace(Piece::pawn(Color::Black));
        default
            .get_mut(&Pos::new(3, 6))
            .unwrap()
            .replace(Piece::pawn(Color::Black));
        default
            .get_mut(&Pos::new(4, 6))
            .unwrap()
            .replace(Piece::pawn(Color::Black));
        default
            .get_mut(&Pos::new(5, 6))
            .unwrap()
            .replace(Piece::pawn(Color::Black));
        default
            .get_mut(&Pos::new(6, 6))
            .unwrap()
            .replace(Piece::pawn(Color::Black));
        default
            .get_mut(&Pos::new(7, 6))
            .unwrap()
            .replace(Piece::pawn(Color::Black));
        default
            .get_mut(&Pos::new(0, 7))
            .unwrap()
            .replace(Piece::rook(Color::Black));
        default
            .get_mut(&Pos::new(1, 7))
            .unwrap()
            .replace(Piece::knight(Color::Black));
        default
            .get_mut(&Pos::new(2, 7))
            .unwrap()
            .replace(Piece::bishop(Color::Black));
        default
            .get_mut(&Pos::new(3, 7))
            .unwrap()
            .replace(Piece::queen(Color::Black));
        default
            .get_mut(&Pos::new(4, 7))
            .unwrap()
            .replace(Piece::king(Color::Black));
        default
            .get_mut(&Pos::new(5, 7))
            .unwrap()
            .replace(Piece::bishop(Color::Black));
        default
            .get_mut(&Pos::new(6, 7))
            .unwrap()
            .replace(Piece::knight(Color::Black));
        default
            .get_mut(&Pos::new(7, 7))
            .unwrap()
            .replace(Piece::rook(Color::Black));
        default
    }

    pub fn with_shape(shape: Shape) -> Self {
        Self {
            tiles: shape.points_iter().map(Tile::new).collect(),
            dead_pieces: Vec::new(),
            shape,
            ..Default::default()
        }
    }

    pub fn with_default_players(tiles: Vec<Tile>, shape: Shape) -> Self {
        Self {
            tiles,
            shape,
            ..Default::default()
        }
    }

    pub fn with_empty_tiles(shape: Shape) -> Self {
        Self {
            tiles: shape.points_iter().map(Tile::new).collect(),
            shape,
            ..Default::default()
        }
    }

    pub fn new(tiles: Vec<Tile>, shape: Shape, players: Vec<Player>) -> Self {
        Self {
            tiles,
            shape,
            players,
            ..Default::default()
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

    pub fn get_nearby_tiles(&self, pos: &Pos) -> Vec<&Tile> {
        vec![pos.north(), pos.east(), pos.south(), pos.west()]
            .into_iter()
            .flatten()
            .filter_map(|p| self.get(&p))
            .collect()
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

    pub fn same_color(&self, pos1: &Pos, pos2: &Pos) -> bool {
        match (self.get(pos1), self.get(pos2)) {
            (Some(tile1), Some(tile2)) => tile1.get_color() == tile2.get_color(),
            _ => false,
        }
    }

    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    pub fn ray_cast<F: Fn(&Tile) -> bool>(
        &self,
        from: &Pos,
        len: Option<usize>,
        shift: &(isize, isize),
        stop_at: F,
    ) -> RayCastInfo {
        if !self.contains(from) {
            return RayCastInfo::empty();
        }
        let next = from.shift(shift.0, shift.1);
        let mut next = match next {
            None => return RayCastInfo::start(from.clone()),
            Some(pos) => pos,
        };
        let mut mid = Vec::with_capacity(len.unwrap_or(10));
        let mut collision = None;
        loop {
            // If len is achieved, collision is none.
            if let Some(len) = len {
                if mid.len() == len {
                    break;
                }
            }
            // if the next position is not in the board, collision is none.
            if !self.contains(&next) {
                break;
            }
            // if the next position is stop, collision is the next position.
            if stop_at(self.get(&next).unwrap()) {
                collision = Some(next.clone());
                break;
            }
            mid.push(next.clone());
            next = match next.shift(shift.0, shift.1) {
                None => break,
                Some(pos) => pos,
            };
        }
        match collision {
            None => RayCastInfo::mid(from.clone(), mid),
            Some(collision) => RayCastInfo::collision(from.clone(), mid, collision),
        }
    }

    pub fn ray_cast_empty(
        &self,
        from: &Pos,
        len: Option<usize>,
        shift: &(isize, isize),
    ) -> RayCastInfo {
        self.ray_cast(from, len, shift, |t| t.has_piece())
    }

    ///
    /// Returns the tiles in the same row as the given position.
    pub fn row_iter(&self, row: usize) -> impl Iterator<Item = &Tile> {
        self.tiles.iter().filter(move |t| t.pos.y == row)
    }

    pub fn height(&self) -> usize {
        self.shape.height()
    }

    pub fn move_piece(&mut self, from: &Pos, to: &Pos) {
        let piece = self.get_mut(from).unwrap().remove();
        self.get_mut(to).unwrap().replace(piece);
    }

    pub fn take_piece(&mut self, from: &Pos, to: &Pos) {
        let piece = self.get_mut(from).unwrap().remove();
        let dead = self.get_mut(to).unwrap().replace(piece);
        self.dead_pieces.push(dead);
    }

    pub fn attack_piece(&mut self, _from: &Pos, to: &Pos) {
        let dead = self.get_mut(to).unwrap().remove();
        self.dead_pieces.push(dead);
    }

    pub fn make(&mut self, action: Action) {
        match action {
            Action::Move { from, to } => self.move_piece(&from, &to),
            Action::Take { from, to } => self.take_piece(&from, &to),
            Action::Attack { from, to } => self.attack_piece(&from, &to),
            Action::Ability { from, info } => Piece::ability(self, from, info),
        }
    }

    pub fn tick(&mut self) {
        // add movement, tick everything on movement
        // if movement == current_player.max_movements, add turn, tick turn
        // if turn == players.len(), add round, tick round
        // TODO: TICK
        // tick pieces
        // tick rng
        // tick effects
        // tick types
        // tick cards
    }

    pub fn player_from_id(&self, player_id: usize) -> Option<&Player> {
        self.players.iter().find(|player| player.id == player_id)
    }

    pub fn player_from_color(&self, color: &Color) -> Option<&Player> {
        self.players.iter().find(|player| &player.color == color)
    }

    pub fn mut_player_from_id(&mut self, player_id: usize) -> Option<&mut Player> {
        self.players
            .iter_mut()
            .find(|player| player.id == player_id)
    }

    pub fn mut_player_from_color(&mut self, color: &Color) -> Option<&mut Player> {
        self.players
            .iter_mut()
            .find(|player| &player.color == color)
    }

    pub fn current_player(&self) -> &Player {
        &self.players[self.time.turn]
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.events.push(event);
    }

    pub fn has_card_on_board(&self, card: Card) -> bool {
        self.cards.0.contains(&card)
    }

    pub fn has_any_card_on_board(&self, cards: Vec<Card>) -> bool {
        self.cards.0.iter().any(|card| cards.contains(card))
    }

    pub fn iter(&self) -> impl Iterator<Item = &Tile> {
        self.tiles.iter()
    }

    pub fn iter_from_pattern<'a, F: Fn(&Pos, &Pos) -> bool + 'a>(
        &'a self,
        from: &'a Pos,
        f: F,
    ) -> impl Iterator<Item = &Tile> + '_ {
        self.iter()
            .filter(move |tile| tile.pos() != from && f(from, tile.pos()))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Tile> {
        self.tiles.iter_mut()
    }

    pub fn iter_mut_from_pattern<'a, F: Fn(&Pos, &Pos) -> bool + 'a>(
        &'a mut self,
        from: &'a Pos,
        f: F,
    ) -> impl Iterator<Item = &mut Tile> + '_ {
        self.iter_mut()
            .filter(move |tile| tile.pos() != from && f(from, tile.pos()))
    }

    pub fn pos_vec(&self) -> Vec<Pos> {
        self.shape.points_iter().collect()
    }

    pub fn pos_vec_from_pattern<'a, F: Fn(&Pos, &Pos) -> bool + 'a>(
        &'a self,
        from: &'a Pos,
        f: F,
    ) -> Vec<Pos> {
        self.iter_from_pattern(from, f)
            .map(|tile| tile.pos.clone())
            .collect()
    }

    pub fn get_data(&self, pos: &Pos) -> Option<&PieceData> {
        self.get(pos).and_then(|tile| tile.piece.data())
    }

    pub fn get_mut_data(&mut self, pos: &Pos) -> Option<&mut PieceData> {
        self.get_mut(pos).and_then(|tile| tile.piece.mut_data())
    }

    pub fn get_piece(&self, pos: &Pos) -> Option<&Piece> {
        self.get(pos).map(|tile| &tile.piece)
    }

    pub fn get_mut_piece(&mut self, pos: &Pos) -> Option<&mut Piece> {
        self.get_mut(pos).map(|tile| &mut tile.piece)
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
            rng: BoardRng::default(),
            events: Events::default(),
            time: Time::default(),
        }
    }
}

/// A ray cast struct.
/// Start is the starting position.
/// Mid are the positions of the ray cast.
/// Collision is the position of the first collision.
pub struct RayCastInfo {
    pub start: Option<Pos>,
    pub mid: Option<Vec<Pos>>,
    pub collision: Option<Pos>,
}

impl RayCastInfo {
    pub fn empty() -> Self {
        RayCastInfo {
            start: None,
            mid: None,
            collision: None,
        }
    }

    pub fn start(start: Pos) -> Self {
        RayCastInfo {
            start: Some(start),
            mid: None,
            collision: None,
        }
    }

    pub fn mid(start: Pos, mid: Vec<Pos>) -> Self {
        RayCastInfo {
            start: Some(start),
            mid: Some(mid),
            collision: None,
        }
    }

    pub fn collision(start: Pos, mid: Vec<Pos>, end: Pos) -> Self {
        RayCastInfo {
            start: Some(start),
            mid: Some(mid),
            collision: Some(end),
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(
            self,
            RayCastInfo {
                start: None,
                mid: None,
                collision: None
            }
        )
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        if self.start.is_some() {
            len += 1;
        }
        if let Some(mid) = &self.mid {
            len += mid.len();
        }
        if self.collision.is_some() {
            len += 1;
        }
        len
    }

    pub fn contains(&self, pos: &Pos) -> bool {
        if let Some(start) = &self.start {
            if start == pos {
                return true;
            }
        }
        if let Some(mid) = &self.mid {
            if mid.contains(pos) {
                return true;
            }
        }
        if let Some(collision) = &self.collision {
            if collision == pos {
                return true;
            }
        }
        false
    }

    pub fn contains_mid(&self, pos: &Pos) -> bool {
        if let Some(mid) = &self.mid {
            if mid.contains(pos) {
                return true;
            }
        }
        false
    }

    pub fn first(&self) -> Option<&Pos> {
        match &self.mid {
            None => None,
            Some(mid) => mid.first(),
        }
    }

    pub fn last(&self) -> Option<&Pos> {
        match &self.mid {
            None => None,
            Some(mid) => mid.last(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct BoardRng {
    movement_rng: RandomNumberGenerator,
    turn_rng: RandomNumberGenerator,
    round_rng: RandomNumberGenerator,
}

impl Default for BoardRng {
    fn default() -> Self {
        Self {
            movement_rng: RandomNumberGenerator::with_seed(thread_rng().gen()),
            turn_rng: RandomNumberGenerator::with_seed(thread_rng().gen()),
            round_rng: RandomNumberGenerator::with_seed(thread_rng().gen()),
        }
    }
}

impl BoardRng {
    pub fn movement(&self) -> f64 {
        self.movement_rng.get_f64()
    }

    pub fn turn(&self) -> f64 {
        self.turn_rng.get_f64()
    }

    pub fn round(&self) -> f64 {
        self.round_rng.get_f64()
    }

    pub fn next_movement(&mut self) {
        self.movement_rng.next();
    }

    pub fn next_turn(&mut self) {
        self.turn_rng.next();
    }

    pub fn next_round(&mut self) {
        self.round_rng.next();
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct RandomNumberGenerator {
    pub seed: u64,
    pub a: u64,
    pub c: u64,
    pub m: u64,
}

impl Default for RandomNumberGenerator {
    fn default() -> Self {
        Self {
            seed: 1,
            a: 1_103_515_245,
            c: 12345,
            m: 32768,
        }
    }
}

impl RandomNumberGenerator {
    pub fn with_seed(seed: u64) -> Self {
        Self {
            seed: seed % 32768,
            ..Default::default()
        }
    }

    pub fn next(&mut self) {
        self.seed = (self.a * self.seed + self.c) % self.m;
    }

    pub fn get_u64(&self) -> u64 {
        self.seed
    }

    pub fn get_f64(&self) -> f64 {
        self.seed as f64 / self.m as f64
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct Events {
    events: Vec<Event>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Event {
    name: String,
    time: Time,
    pos: Option<Pos>,
    functions: Vec<EventFunction>,
}

impl Event {
    pub fn new(name: String, functions: Vec<EventFunction>) -> Self {
        Self {
            name,
            time: Time::turns(1),
            pos: None,
            functions,
        }
    }

    pub fn with_time(name: String, time: Time, functions: Vec<EventFunction>) -> Self {
        Self {
            name,
            time,
            pos: None,
            functions,
        }
    }

    pub fn with_pos(name: String, pos: Pos, functions: Vec<EventFunction>) -> Self {
        Self {
            name,
            time: Time::turns(1),
            pos: Some(pos),
            functions,
        }
    }

    pub fn full(name: String, time: Time, pos: Pos, functions: Vec<EventFunction>) -> Self {
        Self {
            name,
            time,
            pos: Some(pos),
            functions,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum EventFunction {
    Nothing,
    TakeCard(usize),
    ShuffleDeck(usize),
    ApplyEffect(Effect, Pos, FilterFunction),
}

impl EventFunction {
    pub fn act(self, board: &mut Board) -> Result<(), EventFunctionError> {
        match self {
            EventFunction::Nothing => Ok(()),
            EventFunction::TakeCard(player_id) => {
                let player = board.mut_player_from_id(player_id);
                match player {
                    Some(player) => player.take_from_deck(),
                    None => Err(EventFunctionError::PlayerNotFound),
                }
            }
            EventFunction::ShuffleDeck(player_id) => {
                let player = board.mut_player_from_id(player_id);
                match player {
                    Some(player) => {
                        player.deck.shuffle();
                        Ok(())
                    }
                    None => Err(EventFunctionError::PlayerNotFound),
                }
            }
            Self::ApplyEffect(effect, from, filter) => {
                for pos in
                    board.pos_vec_from_pattern(&from, |from, to| filter.filter(board, from, to))
                {
                    if let Some(tile) = board.get_mut(&pos) {
                        if let Some(data) = tile.piece.mut_data() {
                            data.add_effect(effect.clone())
                        }
                    }
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum EventFunctionError {
    PlayerNotFound,
    EmptyDeck,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum FilterFunction {
    Square(usize),
    Cross(usize),
    IsType(Type),
    IsNotType(Type),
    IsColor(Color),
    IsNotColor(Color),
    HasEffect(Effect),
    Pair(Box<FilterFunction>, Box<FilterFunction>),
    Trio(
        Box<FilterFunction>,
        Box<FilterFunction>,
        Box<FilterFunction>,
    ),
}

impl FilterFunction {
    pub fn pair(ff1: FilterFunction, ff2: FilterFunction) -> FilterFunction {
        FilterFunction::Pair(Box::new(ff1), Box::new(ff2))
    }
    pub fn trio(ff1: FilterFunction, ff2: FilterFunction, ff3: FilterFunction) -> FilterFunction {
        FilterFunction::Trio(Box::new(ff1), Box::new(ff2), Box::new(ff3))
    }

    pub fn filter(&self, board: &Board, from: &Pos, to: &Pos) -> bool {
        match self {
            FilterFunction::Cross(range) => pattern::cross(from, to, *range),
            FilterFunction::Square(range) => pattern::square(from, to, *range),
            FilterFunction::IsType(t) => board
                .get_piece(to)
                .map(|piece| piece.is_type(t))
                .unwrap_or_default(),
            FilterFunction::IsNotType(t) => board
                .get_piece(to)
                .map(|piece| !piece.is_type(t))
                .unwrap_or_default(),
            FilterFunction::IsColor(color) => board
                .get_data(to)
                .map(|data| &data.color == color)
                .unwrap_or_default(),
            FilterFunction::IsNotColor(color) => board
                .get_data(to)
                .map(|data| &data.color != color)
                .unwrap_or_default(),
            FilterFunction::HasEffect(effect) => board
                .get_data(to)
                .map(|data| data.has_effect(effect))
                .unwrap_or_default(),
            FilterFunction::Pair(ff1, ff2) => {
                ff1.filter(board, from, to) && ff2.filter(board, from, to)
            }
            FilterFunction::Trio(ff1, ff2, ff3) => {
                ff1.filter(board, from, to)
                    && ff2.filter(board, from, to)
                    && ff3.filter(board, from, to)
            }
        }
    }
}
