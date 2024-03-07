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

    pub fn get_strength(&self) -> usize {
        self.properties.strength()
    }

    pub fn add_effect(&mut self, effect: Effect) {
        self.effects.0.push(effect)
    }

    pub fn has_effect(&self, effect: &Effect) -> bool {
        self.effects.0.contains(effect)
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum Piece {
    #[default]
    None,

    // Default pieces
    Pawn(PieceData),
    Knight(PieceData),
    Bishop(PieceData),
    Rook(PieceData),
    Queen(PieceData),
    King(PieceData),

    // Custom starting pieces
    Archer(PieceData),
    Ballista(PieceData),
    Builder(PieceData),
    Cannon(PieceData),
    Catapult(PieceData),
    CrazyPawn(PieceData),
    Magician(PieceData),
    Paladin(PieceData),
    Ram(PieceData),
    ShieldBearer(PieceData),
    Ship(PieceData),
    SuperPawn(PieceData),
    TeslaTower(PieceData),
    Wall(PieceData),
    Warlock(PieceData),

    // Demonic pieces
    Portal(PieceData),
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
            Piece::Archer(data) => Some(&data.color),
            Piece::Ballista(data) => Some(&data.color),
            Piece::Builder(data) => Some(&data.color),
            Piece::Cannon(data) => Some(&data.color),
            Piece::Catapult(data) => Some(&data.color),
            Piece::CrazyPawn(data) => Some(&data.color),
            Piece::Magician(data) => Some(&data.color),
            Piece::Paladin(data) => Some(&data.color),
            Piece::Ram(data) => Some(&data.color),
            Piece::ShieldBearer(data) => Some(&data.color),
            Piece::Ship(data) => Some(&data.color),
            Piece::SuperPawn(data) => Some(&data.color),
            Piece::TeslaTower(data) => Some(&data.color),
            Piece::Wall(data) => Some(&data.color),
            Piece::Warlock(data) => Some(&data.color),
            Piece::Portal(data) => Some(&data.color),
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
            Piece::Archer(data) => Some(data),
            Piece::Ballista(data) => Some(data),
            Piece::Builder(data) => Some(data),
            Piece::Cannon(data) => Some(data),
            Piece::Catapult(data) => Some(data),
            Piece::CrazyPawn(data) => Some(data),
            Piece::Magician(data) => Some(data),
            Piece::Paladin(data) => Some(data),
            Piece::Ram(data) => Some(data),
            Piece::ShieldBearer(data) => Some(data),
            Piece::Ship(data) => Some(data),
            Piece::SuperPawn(data) => Some(data),
            Piece::TeslaTower(data) => Some(data),
            Piece::Wall(data) => Some(data),
            Piece::Warlock(data) => Some(data),
            Piece::Portal(data) => Some(data),
        }
    }

    pub fn mut_data(&mut self) -> Option<&mut PieceData> {
        match self {
            Piece::None => None,
            Piece::Pawn(data) => Some(data),
            Piece::Knight(data) => Some(data),
            Piece::Bishop(data) => Some(data),
            Piece::Rook(data) => Some(data),
            Piece::Queen(data) => Some(data),
            Piece::King(data) => Some(data),
            Piece::Archer(data) => Some(data),
            Piece::Ballista(data) => Some(data),
            Piece::Builder(data) => Some(data),
            Piece::Cannon(data) => Some(data),
            Piece::Catapult(data) => Some(data),
            Piece::CrazyPawn(data) => Some(data),
            Piece::Magician(data) => Some(data),
            Piece::Paladin(data) => Some(data),
            Piece::Ram(data) => Some(data),
            Piece::ShieldBearer(data) => Some(data),
            Piece::Ship(data) => Some(data),
            Piece::SuperPawn(data) => Some(data),
            Piece::TeslaTower(data) => Some(data),
            Piece::Wall(data) => Some(data),
            Piece::Warlock(data) => Some(data),
            Piece::Portal(data) => Some(data),
        }
    }

    pub fn add_type(&mut self, type_: Type) {
        if let Some(data) = self.mut_data() {
            data.types.0.push(type_);
        }
    }

    pub fn is_type(&self, type_: &Type) -> bool {
        match type_ {
            Type::Biologic => self.is_biologic(),
            Type::Dead => self.is_dead(),
            Type::Demonic => self.is_demonic(),
            Type::Heroic => self.is_heroic(),
            Type::Immune => self.is_immune(),
            Type::Impenetrable(s) => self.is_impenetrable(s),
            Type::Structure => self.is_structure(),
            Type::Tough(s) => self.is_tough(s),
            Type::Transportable(w) => self.is_transportable(w),
        }
    }

    pub fn is_biologic(&self) -> bool {
        match self.data() {
            None => false,
            Some(data) => data.types.0.iter().any(|t| matches!(t, Type::Biologic)),
        }
    }

    pub fn is_structure(&self) -> bool {
        match self.data() {
            None => false,
            Some(data) => data.types.0.iter().any(|t| matches!(t, Type::Structure)),
        }
    }

    pub fn is_transportable(&self, max_weight: &usize) -> bool {
        match self.data() {
            None => false,
            Some(data) => data
                .types
                .0
                .iter()
                .any(|t| matches!(t, Type::Transportable(weight) if weight <= max_weight)),
        }
    }

    pub fn is_impenetrable(&self, min_strength: &usize) -> bool {
        match self.data() {
            None => false,
            Some(data) => data
                .types
                .0
                .iter()
                .any(|t| matches!(t, Type::Impenetrable(strength) if strength >= min_strength)),
        }
    }

    pub fn is_immune(&self) -> bool {
        match self.data() {
            None => false,
            Some(data) => data.types.0.iter().any(|t| matches!(t, Type::Immune)),
        }
    }

    pub fn is_heroic(&self) -> bool {
        match self.data() {
            None => false,
            Some(data) => data.types.0.iter().any(|t| matches!(t, Type::Heroic)),
        }
    }

    pub fn is_demonic(&self) -> bool {
        match self.data() {
            None => false,
            Some(data) => data.types.0.iter().any(|t| matches!(t, Type::Demonic)),
        }
    }

    // CHECK: is this the correct way to implement toughness?
    pub fn is_tough(&self, max_life: &usize) -> bool {
        match self.data() {
            None => false,
            Some(data) => data
                .types
                .0
                .iter()
                .any(|t| matches!(t, Type::Tough(life) if life <= max_life)),
        }
    }

    pub fn is_dead(&self) -> bool {
        match self.data() {
            None => false,
            Some(data) => data.types.0.iter().any(|t| matches!(t, Type::Dead)),
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
                        (Piece::Archer(_), Action::Move { from, to }) => {
                            pattern::archer_move(&from, &to)
                        }
                        (Piece::Archer(_), Action::Take { from: _, to: _ }) => false,
                        (Piece::Archer(_), Action::Attack { from, to }) => {
                            pattern::square(&from, &to, 4)
                        }
                        (Piece::Archer(_), Action::Ability { from: _, info: _ }) => false,
                        (Piece::Ballista(_), Action::Move { from, to }) => {
                            pattern::structure_move(&from, &to)
                        }
                        (Piece::Ballista(_), Action::Take { from: _, to: _ }) => false,
                        (Piece::Ballista(data), Action::Attack { from, to }) => {
                            pattern::blockeable_cross(
                                board,
                                &from,
                                &to,
                                &data.color,
                                6,
                                data.get_strength(),
                            )
                        }
                        (Piece::Ballista(_), Action::Ability { from: _, info: _ }) => false,
                        (Piece::Builder(_), Action::Move { from, to }) => {
                            pattern::magician_move(&from, &to)
                        }
                        (Piece::Builder(_), Action::Take { from, to }) => {
                            pattern::cross(&from, &to, 1)
                        }
                        (Piece::Builder(_), Action::Attack { from: _, to: _ }) => false,
                        (Piece::Builder(_), Action::Ability { from, info }) => {
                            ability::Builder::can_use(board, &from, &info)
                        }
                        (Piece::Cannon(_), Action::Move { from, to }) => {
                            pattern::structure_move(&from, &to)
                        }
                        (Piece::Cannon(_), Action::Take { from: _, to: _ }) => false,
                        (Piece::Cannon(_), Action::Attack { from, to }) => {
                            pattern::square(&from, &to, 3)
                        }
                        (Piece::Cannon(_), Action::Ability { from: _, info: _ }) => false,
                        (Piece::Catapult(_), Action::Move { from, to }) => {
                            pattern::structure_move(&from, &to)
                        }
                        (Piece::Catapult(_), Action::Take { from: _, to: _ }) => false,
                        (Piece::Catapult(_), Action::Attack { from: _, to: _ }) => false,
                        (Piece::Catapult(_), Action::Ability { from, info }) => {
                            ability::Catapult::can_use(board, &from, &info)
                        }
                        (Piece::CrazyPawn(_), Action::Move { from, to }) => {
                            pattern::crazy_pawn(board, &from, &to)
                        }
                        (Piece::CrazyPawn(_), Action::Take { from, to }) => {
                            pattern::crazy_pawn(board, &from, &to)
                        }
                        (Piece::CrazyPawn(_), Action::Attack { from: _, to: _ }) => false,
                        (Piece::CrazyPawn(_), Action::Ability { from, info }) => {
                            ability::CrazyPawn::can_use(board, &from, &info)
                        }
                        (Piece::Magician(_), Action::Move { from, to }) => {
                            pattern::magician_move(&from, &to)
                        }
                        (Piece::Magician(_), Action::Take { from: _, to: _ }) => false,
                        (Piece::Magician(_), Action::Attack { from: _, to: _ }) => false,
                        (Piece::Magician(_), Action::Ability { from, info }) => {
                            ability::Magician::can_use(board, &from, &info)
                        }
                        (Piece::Paladin(_), Action::Move { from, to }) => {
                            pattern::queen(board, &from, &to)
                        }
                        (Piece::Paladin(_), Action::Take { from, to }) => {
                            pattern::queen(board, &from, &to)
                        }
                        (Piece::Paladin(_), Action::Attack { from: _, to: _ }) => false,
                        (Piece::Paladin(_), Action::Ability { from, info }) => {
                            ability::Paladin::can_use(board, &from, &info)
                        }
                        (Piece::Ram(_), Action::Move { from, to }) => {
                            pattern::structure_move(&from, &to)
                        }
                        (Piece::Ram(_), Action::Take { from: _, to: _ }) => false,
                        (Piece::Ram(_), Action::Attack { from: _, to: _ }) => false,
                        (Piece::Ram(_), Action::Ability { from, info }) => {
                            ability::Ram::can_use(board, &from, &info)
                        }
                        (Piece::ShieldBearer(data), Action::Move { from, to }) => {
                            pattern::pawn_move(board, &data.color, &from, &to)
                        }
                        (Piece::ShieldBearer(data), Action::Take { from, to }) => {
                            pattern::pawn_take(board, &data.color, &from, &to)
                        }
                        (Piece::ShieldBearer(_), Action::Attack { from: _, to: _ }) => false,
                        (Piece::ShieldBearer(_), Action::Ability { from, info }) => {
                            ability::ShieldBearer::can_use(board, &from, &info)
                        }
                        (Piece::Ship(_), Action::Move { from, to }) => {
                            pattern::magician_move(&from, &to)
                        }
                        (Piece::Ship(_), Action::Take { from, to }) => pattern::king(&from, &to),
                        (Piece::Ship(_), Action::Attack { from: _, to: _ }) => false,
                        (Piece::Ship(_), Action::Ability { from, info }) => {
                            ability::Ship::can_use(board, &from, &info)
                        }
                        (Piece::SuperPawn(_), Action::Move { from, to }) => {
                            pattern::super_pawn_move(board, &data.color, &from, &to)
                        }
                        (Piece::SuperPawn(_), Action::Take { from, to }) => {
                            pattern::super_pawn_take(board, &data.color, &from, &to)
                        }
                        (Piece::SuperPawn(_), Action::Attack { from: _, to: _ }) => false,
                        (Piece::SuperPawn(_), Action::Ability { from, info }) => {
                            ability::SuperPawn::can_use(board, &from, &info)
                        }
                        (Piece::TeslaTower(_), Action::Move { from, to }) => {
                            pattern::magician_move(&from, &to)
                        }
                        (Piece::TeslaTower(_), Action::Take { from, to }) => {
                            pattern::structure_move(&from, &to)
                        }
                        (Piece::TeslaTower(_), Action::Attack { from: _, to: _ }) => false,
                        (Piece::TeslaTower(_), Action::Ability { from, info }) => {
                            ability::TeslaTower::can_use(board, &from, &info)
                        }
                        (Piece::Wall(_), _) => false,
                        (Piece::Warlock(_), Action::Move { from, to }) => {
                            pattern::magician_move(&from, &to)
                        }
                        (Piece::Warlock(_), Action::Take { from: _, to: _ }) => false,
                        (Piece::Warlock(_), Action::Attack { from: _, to: _ }) => false,
                        (Piece::Warlock(_), Action::Ability { from, info }) => {
                            ability::Warlock::can_use(board, &from, &info)
                        }
                        (Piece::Portal(_), Action::Move { from: _, to: _ }) => false,
                        (Piece::Portal(_), Action::Take { from: _, to: _ }) => false,
                        (Piece::Portal(_), Action::Attack { from: _, to: _ }) => false,
                        (Piece::Portal(_), Action::Ability { from, info }) => {
                            ability::Portal::can_use(board, &from, &info)
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
            Piece::Archer(_) => (),
            Piece::Ballista(_) => (),
            Piece::Builder(data) => {
                ability::Builder::r#use(board, &from, info.clone());
                data.on_do(&Action::Ability { from, info });
            }
            Piece::Cannon(_) => (),
            Piece::Catapult(data) => {
                ability::Catapult::r#use(board, &from, info.clone());
                data.on_do(&Action::Ability { from, info });
            }
            Piece::CrazyPawn(data) => {
                ability::CrazyPawn::r#use(board, &from, info.clone());
                data.on_do(&Action::Ability { from, info });
            }
            Piece::Magician(data) => {
                ability::Magician::r#use(board, &from, info.clone());
                data.on_do(&Action::Ability { from, info });
            }
            Piece::Paladin(data) => {
                ability::Paladin::r#use(board, &from, info.clone());
                data.on_do(&Action::Ability { from, info });
            }
            Piece::Ram(data) => {
                ability::Ram::r#use(board, &from, info.clone());
                data.on_do(&Action::Ability { from, info });
            }
            Piece::ShieldBearer(data) => {
                ability::ShieldBearer::r#use(board, &from, info.clone());
                data.on_do(&Action::Ability { from, info });
            }
            Piece::Ship(data) => {
                ability::Ship::r#use(board, &from, info.clone());
                data.on_do(&Action::Ability { from, info });
            }
            Piece::SuperPawn(data) => {
                ability::SuperPawn::r#use(board, &from, info.clone());
                data.on_do(&Action::Ability { from, info });
            }
            Piece::TeslaTower(data) => {
                ability::TeslaTower::r#use(board, &from, info.clone());
                data.on_do(&Action::Ability { from, info });
            }
            Piece::Wall(_) => (),
            Piece::Warlock(data) => {
                ability::Warlock::r#use(board, &from, info.clone());
                data.on_do(&Action::Ability { from, info });
            }
            Piece::Portal(data) => {
                ability::Portal::r#use(board, &from, info.clone());
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

    pub fn archer(color: Color) -> Self {
        Self::Archer(PieceData::new(
            color,
            vec![Type::Biologic, Type::Transportable(3)],
        ))
    }

    pub fn ballista(color: Color) -> Self {
        Self::Ballista(PieceData::with_props(
            color,
            vec![Type::Structure],
            vec![Property::Strength(3)],
        ))
    }

    pub fn builder(color: Color) -> Self {
        Self::Builder(PieceData::new(
            color,
            vec![Type::Biologic, Type::Transportable(3)],
        ))
    }

    pub fn cannon(color: Color) -> Self {
        Self::Cannon(PieceData::new(color, vec![Type::Structure]))
    }

    pub fn catapult(color: Color) -> Self {
        Self::Catapult(PieceData::new(color, vec![Type::Structure]))
    }

    pub fn crazy_pawn(color: Color) -> Self {
        Self::CrazyPawn(PieceData::new(
            color,
            vec![Type::Biologic, Type::Transportable(2)],
        ))
    }

    pub fn magician(color: Color) -> Self {
        Self::Magician(PieceData::new(
            color,
            vec![
                Type::Biologic,
                Type::Transportable(4),
                Type::Heroic,
                Type::Immune,
            ],
        ))
    }

    pub fn paladin(color: Color) -> Self {
        Self::Paladin(PieceData::new(
            color,
            vec![
                Type::Biologic,
                Type::Transportable(4),
                Type::Heroic,
                Type::Immune,
            ],
        ))
    }

    pub fn ram(color: Color) -> Self {
        Self::Ram(PieceData::with_props(
            color,
            vec![Type::Structure],
            vec![Property::Strength(2)],
        ))
    }

    pub fn shield_bearer(color: Color) -> Self {
        Self::ShieldBearer(PieceData::new(
            color,
            vec![
                Type::Biologic,
                Type::Transportable(2),
                Type::Impenetrable(5),
            ],
        ))
    }

    pub fn ship(color: Color) -> Self {
        Self::Ship(PieceData::new(color, vec![Type::Structure]))
    }

    pub fn super_pawn(color: Color) -> Self {
        Self::SuperPawn(PieceData::new(
            color,
            vec![Type::Biologic, Type::Transportable(2)],
        ))
    }

    pub fn tesla_tower(color: Color) -> Self {
        Self::TeslaTower(PieceData::new(color, vec![Type::Structure]))
    }

    pub fn wall(color: Color) -> Self {
        Self::Wall(PieceData::new(
            color,
            vec![Type::Structure, Type::Impenetrable(2)],
        ))
    }

    pub fn warlock(color: Color) -> Self {
        Self::Warlock(PieceData::new(
            color,
            vec![Type::Transportable(5), Type::Demonic, Type::Immune],
        ))
    }

    pub fn portal(color: Color) -> Self {
        Self::Portal(PieceData::new(color, vec![Type::Structure]))
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
    // This property is for pieces that can only use one time its ability, like the king.
    AbilityUsed(bool),
    // This property is for pieces that have been taken multiple times. Needed to use Type::Tough.
    Taken(usize),
    // This property is for pieces that have multiple pieces, like the necromancer.
    Pieces(Vec<Piece>),
    // This property is for pieces that have a strength, like the ballista. Needed for pieces that interact with Type::Impenetrable.
    Strength(usize),
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

impl Properties {
    pub fn has_used_ability(&self) -> bool {
        self.0
            .iter()
            .any(|p| matches!(p, Property::AbilityUsed(true)))
    }

    // CHECK: optimal way to implement this, avoiding filter_map and using find?
    pub fn taken_times(&self) -> usize {
        self.0
            .iter()
            .filter_map(|p| match p {
                Property::Taken(times) => Some(*times),
                _ => None,
            })
            .sum()
    }

    pub fn contains_pawn(&self) -> bool {
        self.0.iter().any(|p| matches!(p, Property::Pieces(pieces) if pieces.iter().any(|p| matches!(p, Piece::Pawn(_)))))
    }

    pub fn contains_knight(&self) -> bool {
        self.0.iter().any(|p| matches!(p, Property::Pieces(pieces) if pieces.iter().any(|p| matches!(p, Piece::Knight(_)))))
    }

    pub fn contains_bishop(&self) -> bool {
        self.0.iter().any(|p| matches!(p, Property::Pieces(pieces) if pieces.iter().any(|p| matches!(p, Piece::Bishop(_)))))
    }

    pub fn contains_rook(&self) -> bool {
        self.0.iter().any(|p| matches!(p, Property::Pieces(pieces) if pieces.iter().any(|p| matches!(p, Piece::Rook(_)))))
    }

    pub fn contains_queen(&self) -> bool {
        self.0.iter().any(|p| matches!(p, Property::Pieces(pieces) if pieces.iter().any(|p| matches!(p, Piece::Queen(_)))))
    }

    pub fn contains_king(&self) -> bool {
        self.0.iter().any(|p| matches!(p, Property::Pieces(pieces) if pieces.iter().any(|p| matches!(p, Piece::King(_)))))
    }

    pub fn contains_archer(&self) -> bool {
        self.0.iter().any(|p| matches!(p, Property::Pieces(pieces) if pieces.iter().any(|p| matches!(p, Piece::Archer(_)))))
    }

    pub fn contains_ballista(&self) -> bool {
        self.0.iter().any(|p| matches!(p, Property::Pieces(pieces) if pieces.iter().any(|p| matches!(p, Piece::Ballista(_)))))
    }

    pub fn strength(&self) -> usize {
        self.0
            .iter()
            .filter_map(|p| match p {
                Property::Strength(strength) => Some(*strength),
                _ => None,
            })
            .sum()
    }
}
