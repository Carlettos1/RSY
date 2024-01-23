use serde::{Deserialize, Serialize};

use crate::{
    ability::{self, Ability},
    board::Board,
    pattern::{self},
    Action, Color, Info, Pos, Time,
};

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct PieceData {
    pub moved: bool,
    pub cooldown: Time,
    pub color: Color,
    pub types: Types,
    pub effects: Effects,
    pub properties: Properties,
}

impl PieceData {
    pub fn new(color: Color, types: Vec<Type>) -> Self {
        Self {
            color,
            types: Types(types),
            ..Default::default()
        }
    }

    pub fn with_props(color: Color, types: Vec<Type>, properties: Vec<Property>) -> Self {
        Self {
            color,
            types: Types(types),
            properties: Properties(properties),
            ..Default::default()
        }
    }

    pub fn can_do(&self, action: &Action) -> bool {
        self.types.can_do(action) && self.effects.can_do(action)
    }

    pub fn on_do(&self, action: &Action) {
        self.types.on_do(action);
        self.effects.on_do(action);
    }

    pub fn can_be(&self, action: &Action) -> bool {
        self.types.can_be(action) && self.effects.can_be(action)
    }

    pub fn on_be(&self, action: &Action) {
        self.types.on_be(action);
        self.effects.on_be(action);
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum Piece {
    #[default]
    None,
    Pawn(PieceData),
    Knight(PieceData),
    Bishop(PieceData),
    Rook(PieceData),
    Queen(PieceData),
    King(PieceData),
}

impl Piece {
    pub fn color(&self) -> Option<&Color> {
        match self {
            Piece::None => None,
            Piece::Pawn(data) => Some(&data.color),
            Piece::Knight(data) => Some(&data.color),
            Piece::Bishop(data) => Some(&data.color),
            Piece::Rook(data) => Some(&data.color),
            Piece::Queen(data) => Some(&data.color),
            Piece::King(data) => Some(&data.color),
        }
    }

    pub fn data(&self) -> Option<&PieceData> {
        match self {
            Piece::None => None,
            Piece::Pawn(data) => Some(data),
            Piece::Knight(data) => Some(data),
            Piece::Bishop(data) => Some(data),
            Piece::Rook(data) => Some(data),
            Piece::Queen(data) => Some(data),
            Piece::King(data) => Some(data),
        }
    }

    pub fn can_do(&self, board: &Board, action: Action) -> bool {
        match self.data() {
            None => false,
            Some(data) => {
                data.can_do(&action)
                    && match (self, action) {
                        (Piece::None, _) => false,
                        (Piece::Pawn(data), Action::Move { from, to }) => {
                            pattern::pawn_move(board, &data.color, &from, &to)
                        }
                        (Piece::Pawn(data), Action::Take { from, to }) => {
                            pattern::pawn_take(board, &data.color, &from, &to)
                        }
                        (Piece::Pawn(_), Action::Attack { from: _, to: _ }) => false,
                        (Piece::Pawn(_), Action::Ability { from, info }) => {
                            ability::Pawn::can_use(board, &from, &info)
                        }
                        (Piece::Knight(_), Action::Move { from, to }) => {
                            pattern::knight(&from, &to)
                        }
                        (Piece::Knight(_), Action::Take { from, to }) => {
                            pattern::knight(&from, &to)
                        }
                        (Piece::Knight(_), Action::Attack { from: _, to: _ }) => false,
                        (Piece::Knight(_), Action::Ability { from, info }) => {
                            ability::Knight::can_use(board, &from, &info)
                        }
                        (Piece::Bishop(_), Action::Move { from, to }) => {
                            pattern::bishop(board, &from, &to)
                        }
                        (Piece::Bishop(_), Action::Take { from, to }) => {
                            pattern::bishop(board, &from, &to)
                        }
                        (Piece::Bishop(_), Action::Attack { from: _, to: _ }) => false,
                        (Piece::Bishop(_), Action::Ability { from, info }) => {
                            ability::Bishop::can_use(board, &from, &info)
                        }
                        (Piece::Rook(_), Action::Move { from, to }) => {
                            pattern::rook(board, &from, &to)
                        }
                        (Piece::Rook(_), Action::Take { from, to }) => {
                            pattern::rook(board, &from, &to)
                        }
                        (Piece::Rook(_), Action::Attack { from: _, to: _ }) => false,
                        (Piece::Rook(_), Action::Ability { from, info }) => {
                            ability::Rook::can_use(board, &from, &info)
                        }
                        (Piece::Queen(_), Action::Move { from, to }) => {
                            pattern::queen(board, &from, &to)
                        }
                        (Piece::Queen(_), Action::Take { from, to }) => {
                            pattern::queen(board, &from, &to)
                        }
                        (Piece::Queen(_), Action::Attack { from: _, to: _ }) => false,
                        (Piece::Queen(_), Action::Ability { from, info }) => {
                            ability::Queen::can_use(board, &from, &info)
                        }
                        (Piece::King(_), Action::Move { from, to }) => pattern::king(&from, &to),
                        (Piece::King(_), Action::Take { from, to }) => pattern::king(&from, &to),
                        (Piece::King(_), Action::Attack { from: _, to: _ }) => false,
                        (Piece::King(_), Action::Ability { from, info }) => {
                            ability::King::can_use(board, &from, &info)
                        }
                    }
            }
        }
    }

    pub fn ability(board: &mut Board, from: Pos, info: Info) {
        let piece = board.get(&from).unwrap().piece.clone();
        match piece {
            Piece::None => (),
            Piece::Pawn(data) => {
                ability::Pawn::r#use(board, &from, info.clone());
                data.on_do(&Action::Ability { from, info });
            }
            Piece::Knight(data) => {
                ability::Knight::r#use(board, &from, info.clone());
                data.on_do(&Action::Ability { from, info });
            }
            Piece::Bishop(data) => {
                ability::Bishop::r#use(board, &from, info.clone());
                data.on_do(&Action::Ability { from, info });
            }
            Piece::Rook(data) => {
                ability::Rook::r#use(board, &from, info.clone());
                data.on_do(&Action::Ability { from, info });
            }
            Piece::Queen(data) => {
                ability::Queen::r#use(board, &from, info.clone());
                data.on_do(&Action::Ability { from, info });
            }
            Piece::King(data) => {
                ability::King::r#use(board, &from, info.clone());
                data.on_do(&Action::Ability { from, info });
            }
        }
    }

    pub fn pawn(color: Color) -> Self {
        Self::Pawn(PieceData::new(
            color,
            vec![Type::Biologic, Type::Transportable(2)],
        ))
    }

    pub fn knight(color: Color) -> Self {
        Self::Knight(PieceData::new(
            color,
            vec![Type::Biologic, Type::Transportable(4)],
        ))
    }

    pub fn bishop(color: Color) -> Self {
        Self::Bishop(PieceData::new(
            color,
            vec![Type::Biologic, Type::Transportable(3)],
        ))
    }

    pub fn rook(color: Color) -> Self {
        Self::Rook(PieceData::new(color, vec![Type::Structure]))
    }

    pub fn queen(color: Color) -> Self {
        Self::Queen(PieceData::new(color, vec![Type::Biologic, Type::Heroic]))
    }

    pub fn king(color: Color) -> Self {
        Self::King(PieceData::with_props(
            color,
            vec![Type::Biologic, Type::Heroic, Type::Immune],
            vec![Property::AbilityUsed(false)],
        ))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum Type {
    Biologic,
    Structure,
    /// This pieces can be transported, it has a weight
    Transportable(usize),
    /// Abilities cannot go through this piece, it has a strength
    Impenetrable(usize),
    /// Immune from habilities
    Immune,
    /// Immune from attacks
    Heroic,
    /// They give back mana when killed
    Demonic,
    /// They need to be killed multiple times, works like health
    Tough(usize),
    /// Dead pieces are in control of a necromancer
    Dead,
}

impl Type {
    pub fn can_do(&self, _action: &Action) -> bool {
        match self {
            Type::Biologic => true,
            Type::Structure => true,
            Type::Transportable(_) => true,
            Type::Impenetrable(_) => true,
            Type::Immune => true,
            Type::Heroic => true,
            Type::Demonic => true,
            Type::Tough(_) => true,
            Type::Dead => true,
        }
    }

    pub fn on_do(&self, _action: &Action) {
        match self {
            Type::Biologic => (),
            Type::Structure => (),
            Type::Transportable(_) => (),
            Type::Impenetrable(_) => (),
            Type::Immune => (),
            Type::Heroic => (),
            Type::Demonic => (),
            Type::Tough(_) => (),
            Type::Dead => (),
        }
    }

    pub fn can_be(&self, action: &Action) -> bool {
        match self {
            Type::Biologic => true,
            Type::Structure => true,
            Type::Transportable(_) => true,
            Type::Impenetrable(_) => true,
            Type::Immune if action.is_ability() => false,
            Type::Immune => true,
            Type::Heroic if action.is_attack() => false,
            Type::Heroic => true,
            Type::Demonic => true,
            Type::Tough(_) => true,
            Type::Dead => true,
        }
    }

    pub fn on_be(&self, _action: &Action) {
        match self {
            Type::Biologic => (),
            Type::Structure => (),
            Type::Transportable(_) => (),
            Type::Impenetrable(_) => (),
            Type::Immune => (),
            Type::Heroic => (),
            Type::Demonic => todo!("Add mana to player on dead"),
            Type::Tough(_) => todo!("implement toughness"),
            Type::Dead => (),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum Effect {
    Deactivate(Time),
    Fire(Time),
    Ice(Time),
    Invulnerability(Time),
}

impl Effect {
    pub fn deactivate() -> Self {
        Effect::Deactivate(Time::rounds(6))
    }

    pub fn fire() -> Self {
        Effect::Fire(Time::rounds(5))
    }

    pub fn ice() -> Self {
        Effect::Ice(Time::rounds(3))
    }

    pub fn invulnerability() -> Self {
        Effect::Invulnerability(Time::rounds(3))
    }

    pub fn can_do(&self, _action: &Action) -> bool {
        match self {
            Effect::Deactivate(_) => false,
            Effect::Fire(_) => true,
            Effect::Ice(_) => false,
            Effect::Invulnerability(_) => true,
        }
    }

    pub fn on_do(&self, _action: &Action) {
        match self {
            Effect::Deactivate(_) => (),
            Effect::Fire(_) => (),
            Effect::Ice(_) => (),
            Effect::Invulnerability(_) => (),
        }
    }

    pub fn can_be(&self, _action: &Action) -> bool {
        match self {
            Effect::Deactivate(_) => true,
            Effect::Fire(_) => true,
            Effect::Ice(_) => true,
            Effect::Invulnerability(_) => false,
        }
    }

    pub fn on_be(&self, _action: &Action) {
        match self {
            Effect::Deactivate(_) => (),
            Effect::Fire(_) => (),
            Effect::Ice(_) => (),
            Effect::Invulnerability(_) => (),
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum Property {
    #[default]
    None,
    AbilityUsed(bool),
    Taken(usize),
    Pieces(Vec<Piece>),
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Types(pub Vec<Type>);

impl Types {
    pub fn can_do(&self, action: &Action) -> bool {
        self.0.iter().all(|t| t.can_do(action))
    }

    pub fn on_do(&self, action: &Action) {
        self.0.iter().for_each(|t| t.on_do(action))
    }

    pub fn can_be(&self, action: &Action) -> bool {
        self.0.iter().all(|t| t.can_be(action))
    }

    pub fn on_be(&self, action: &Action) {
        self.0.iter().for_each(|t| t.on_do(action))
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Effects(pub Vec<Effect>);

impl Effects {
    pub fn can_do(&self, action: &Action) -> bool {
        self.0.iter().all(|e| e.can_do(action))
    }

    pub fn on_do(&self, action: &Action) {
        self.0.iter().for_each(|e| e.on_do(action))
    }

    pub fn can_be(&self, action: &Action) -> bool {
        self.0.iter().all(|e| e.can_be(action))
    }

    pub fn on_be(&self, action: &Action) {
        self.0.iter().for_each(|e| e.on_do(action))
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Properties(pub Vec<Property>);
