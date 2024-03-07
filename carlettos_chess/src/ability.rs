use core::panic;

use crate::{
    board::{Board, Event, EventFunction, FilterFunction, Mana, Tile},
    card::Card,
    pattern,
    piece::{Effect, Piece, Type},
    Color, Direction, Info, PaladinAbilityType, Pos, Time,
};

pub struct AbilityData {
    pub cooldown: Time,
    pub cost: Mana,
}

pub trait Ability {
    fn data(&self) -> AbilityData;
    fn r#use(board: &mut Board, from: &Pos, info: Info);
    fn can_use(board: &Board, from: &Pos, info: &Info) -> bool;
}

pub struct Pawn;

impl Ability for Pawn {
    fn data(&self) -> AbilityData {
        AbilityData {
            cooldown: Time::default(),
            cost: Mana::default(),
        }
    }

    fn r#use(board: &mut Board, from: &Pos, info: Info) {
        match info {
            Info::Piece(piece) => drop(board.get_mut(from).unwrap().replace(piece)),
            _ => panic!("Non piece info"),
        }
    }

    fn can_use(board: &Board, from: &Pos, _info: &Info) -> bool {
        let tile = board.get(from).unwrap();
        match tile.get_color().unwrap() {
            Color::White => board.get(&Pos::new(from.x, from.y + 1)).is_none(),
            Color::Black => from.y == 0,
        }
    }
}

pub struct Knight;

impl Ability for Knight {
    fn data(&self) -> AbilityData {
        AbilityData {
            cooldown: Time::rounds(10),
            cost: Mana(1),
        }
    }

    fn r#use(board: &mut Board, from: &Pos, _info: Info) {
        let color = board.get(from).unwrap().get_color().unwrap().clone();
        board
            .get_mut(&from.east().unwrap())
            .unwrap()
            .replace(Piece::pawn(color.clone()));
        board
            .get_mut(&from.west().unwrap())
            .unwrap()
            .replace(Piece::pawn(color));
    }

    fn can_use(board: &Board, from: &Pos, _info: &Info) -> bool {
        let e = from.east();
        let w = from.west();
        let e = if let Some(pos) = e { pos } else { return false };
        let w = if let Some(pos) = w { pos } else { return false };
        matches!((board.get(&e), board.get(&w)), (Some(te), Some(tw)) if te.is_empty() && tw.is_empty())
    }
}

pub struct Bishop;

impl Ability for Bishop {
    fn data(&self) -> AbilityData {
        AbilityData {
            cooldown: Time::rounds(2),
            cost: Mana(0),
        }
    }

    fn r#use(board: &mut Board, from: &Pos, info: Info) {
        if let Info::Direction(direction) = info {
            let piece = board.get_mut(from).unwrap().remove();
            board
                .get_mut(&from.direction_shift(&direction).unwrap())
                .unwrap()
                .replace(piece);
        }
    }

    fn can_use(board: &Board, from: &Pos, info: &Info) -> bool {
        match info {
            Info::Direction(direction) => match from.direction_shift(direction) {
                None => false,
                Some(to) => match board.get(&to) {
                    None => false,
                    Some(tile) => tile.is_empty(),
                },
            },
            _ => false,
        }
    }
}

pub struct Rook;

impl Ability for Rook {
    fn data(&self) -> AbilityData {
        AbilityData {
            cooldown: Time::rounds(10),
            cost: Mana(0),
        }
    }

    fn r#use(board: &mut Board, from: &Pos, info: Info) {
        // The rook's ability is to "throw" all nearby rooks in one direction.
        // By throwing, we mean that the rook is moved in that direction until it hits a piece.
        // If the piece is an enemy, it is captured. If it is an ally, it is not captured.
        // If it is the edge of the board, the rook is leaved there.
        // first, we get the direction of the throw
        if let Info::Direction(direction) = info {
            // then, we get all the nearby rooks.
            let mut rooks: Vec<&Tile> = board
                .get_nearby_tiles(from)
                .into_iter()
                .filter(|t| t.has_rook())
                .collect();
            let mut prev_len = rooks.len();
            let mut next_len = 0;
            while prev_len != next_len {
                let mut new_rooks = Vec::new();
                for nearby_rook in rooks
                    .iter()
                    .flat_map(|t| board.get_nearby_tiles(t.pos()))
                    .filter(|t| t.has_rook())
                {
                    if !rooks.contains(&nearby_rook) {
                        new_rooks.push(nearby_rook);
                    }
                }
                rooks.extend(new_rooks);
                prev_len = next_len;
                next_len = rooks.len();
            }
            // rooks now contains all the rooks that will be thrown.
            // then, we need to sort them so that the first one to be
            // thrown is the one closest to the edge of the board in the direction of the throw.
            rooks.sort_by(|t1, t2| match &direction {
                Direction::N => t1.pos().y.cmp(&t2.pos().y).reverse(),
                Direction::E => t1.pos().x.cmp(&t2.pos().x).reverse(),
                Direction::S => t1.pos().y.cmp(&t2.pos().y),
                Direction::W => t1.pos().x.cmp(&t2.pos().x),
            });
            // now, we can throw the rooks.
            let pos_vec = rooks
                .into_iter()
                .map(|t| t.pos().clone())
                .collect::<Vec<_>>();
            for rook_pos in pos_vec {
                let rc = board.ray_cast_empty(&rook_pos, None, &(&direction).into());
                if let Some(last) = rc.last() {
                    let piece = board.get_mut(&rook_pos).unwrap().remove();
                    board.get_mut(last).unwrap().replace(piece);
                }
            }
        } else {
            panic!("Non direction info")
        }
    }

    fn can_use(_board: &Board, _from: &Pos, info: &Info) -> bool {
        matches!(info, Info::Direction(_))
    }
}

pub struct Queen;

impl Ability for Queen {
    fn data(&self) -> AbilityData {
        AbilityData {
            cooldown: Time::rounds(5),
            cost: Mana(0),
        }
    }
    fn r#use(board: &mut Board, from: &Pos, info: Info) {
        if let Info::Pos(pos) = info {
            let piece = board.get_mut(from).unwrap().remove();
            board.get_mut(&pos).unwrap().replace(piece);
        } else {
            panic!("Non pos info")
        }
    }

    fn can_use(board: &Board, from: &Pos, info: &Info) -> bool {
        matches!(info, Info::Pos(to) if pattern::knight(from, to) && board.contains(to) && board.get(to).unwrap().is_empty())
    }
}

pub struct King;

impl Ability for King {
    fn data(&self) -> AbilityData {
        AbilityData {
            cooldown: Time::default(),
            cost: Mana(2),
        }
    }

    fn r#use(board: &mut Board, from: &Pos, info: Info) {
        if let Info::Pos(to) = info {
            let piece = board.get_mut(from).unwrap().remove();
            board.get_mut(&to).unwrap().replace(piece);
        } else {
            panic!("Non pos info")
        }
    }

    fn can_use(board: &Board, from: &Pos, info: &Info) -> bool {
        matches!(info, Info::Pos(to) if pattern::square(from, to, 5) && board.contains(to) && board.get(to).unwrap().is_empty())
    }
}

pub struct Builder;

impl Ability for Builder {
    fn data(&self) -> AbilityData {
        AbilityData {
            cooldown: Time::rounds(10),
            cost: Mana(0),
        }
    }

    fn can_use(_board: &Board, _from: &Pos, info: &Info) -> bool {
        matches!(info, Info::Direction(_))
    }

    fn r#use(board: &mut Board, from: &Pos, info: Info) {
        if let Info::Direction(dir) = info {
            let color = board.get(from).unwrap().get_color().unwrap().clone();
            for subdir in dir.related_subdirections() {
                if let Some(pos) = from.subdirection_shift(&subdir) {
                    if let Some(tile) = board.get_mut(&pos) {
                        if tile.is_empty() {
                            tile.replace(Piece::wall(color.clone()));
                        }
                    }
                }
            }
        } else {
            panic!("Non direction info")
        }
    }
}

pub struct Catapult;

impl Ability for Catapult {
    fn data(&self) -> AbilityData {
        AbilityData {
            cooldown: Time::rounds(2),
            cost: Mana(0),
        }
    }

    fn can_use(board: &Board, from: &Pos, info: &Info) -> bool {
        match info {
            Info::Trio(dir, subdir, squares) => {
                match (dir.as_ref(), subdir.as_ref(), squares.as_ref()) {
                    (Info::Direction(dir), Info::SubDirection(subdir), Info::Integer(squares)) => {
                        if let Some(piece_pos) = from.subdirection_shift(subdir) {
                            if let Some(tile) = board.get(&piece_pos) {
                                if tile.piece.is_transportable(&5) {
                                    // TODO: add an attribute to make this dynamic
                                    let (x, y): (isize, isize) = (dir).into();
                                    let to =
                                        from.shift(x * *squares as isize, y * *squares as isize);
                                    if let Some(to) = to {
                                        if board
                                            .get(&to)
                                            .map(|tile| tile.is_empty())
                                            .unwrap_or_default()
                                        {
                                            return true;
                                        }
                                    }
                                }
                            }
                        }
                        false
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn r#use(board: &mut Board, from: &Pos, info: Info) {
        match info {
            Info::Trio(dir, subdir, squares) => match (*dir, *subdir, *squares) {
                (Info::Direction(dir), Info::SubDirection(subdir), Info::Integer(squares)) => {
                    let piece_pos = from.subdirection_shift(&subdir).unwrap();
                    let piece = board.get_mut(&piece_pos).unwrap().remove();

                    let (x, y): (isize, isize) = (&dir).into();
                    let to = from
                        .shift(x * squares as isize, y * squares as isize)
                        .unwrap();
                    board.get_mut(&to).unwrap().replace(piece);
                }
                _ => panic!("non (dir, subdir) info for catapult ability"),
            },
            _ => panic!("non pair info for catapult ability"),
        }
    }
}

pub struct CrazyPawn;

impl Ability for CrazyPawn {
    fn data(&self) -> AbilityData {
        AbilityData {
            cooldown: Time::default(),
            cost: Mana(0),
        }
    }

    fn can_use(_board: &Board, _from: &Pos, _info: &Info) -> bool {
        true
    }

    fn r#use(board: &mut Board, _from: &Pos, _info: Info) {
        let player_id = *board.current_player().id();
        board.add_event(Event::new(
            "Crazy Pawn Cards!".to_string(),
            vec![
                EventFunction::TakeCard(player_id),
                EventFunction::TakeCard(player_id),
                EventFunction::ShuffleDeck(player_id),
            ],
        ))
    }
}

pub struct Magician;

impl Ability for Magician {
    fn data(&self) -> AbilityData {
        AbilityData {
            cooldown: Time::rounds(6),
            cost: Mana(2),
        }
    }

    fn can_use(board: &Board, _from: &Pos, _info: &Info) -> bool {
        board.has_any_card_on_board(vec![Card::Ice, Card::Fire])
    }

    fn r#use(board: &mut Board, from: &Pos, _info: Info) {
        let has_ice = board.has_card_on_board(Card::Ice);
        let has_fire = board.has_card_on_board(Card::Fire);
        board
            .iter_mut_from_pattern(from, |from, to| pattern::square(from, to, 4))
            .for_each(|tile| {
                if let Some(data) = tile.piece.mut_data() {
                    if has_fire {
                        data.add_effect(Effect::fire());
                    }
                    if has_ice {
                        data.add_effect(Effect::ice());
                    }
                }
            })
    }
}

pub struct Paladin;

impl Ability for Paladin {
    fn data(&self) -> AbilityData {
        AbilityData {
            cooldown: Time::rounds(8),
            cost: Mana(2),
        }
    }

    fn can_use(board: &Board, from: &Pos, info: &Info) -> bool {
        match info {
            Info::PaladinAbilityInfo(ability_type) => match ability_type {
                PaladinAbilityType::Attack(to) => {
                    board.has_card_on_board(Card::AttackDemonic)
                        && !board.is_empty(to)
                        && !board.same_color(from, to)
                }
                PaladinAbilityType::Invulnerability(to) => {
                    board.has_card_on_board(Card::Invulnerability)
                        && !board.is_empty(to)
                        && board.same_color(from, to)
                }
                PaladinAbilityType::Revive(to) => {
                    board.has_card_on_board(Card::Revive) && board.is_empty(to)
                }
            },
            _ => false,
        }
    }

    fn r#use(board: &mut Board, from: &Pos, info: Info) {
        if let Info::PaladinAbilityInfo(ability_type) = info {
            match ability_type {
                PaladinAbilityType::Attack(to) => board.attack_piece(from, &to),
                PaladinAbilityType::Invulnerability(to) => board
                    .get_mut_data(&to)
                    .unwrap()
                    .add_effect(Effect::Invulnerability(Time::rounds(5))),
                PaladinAbilityType::Revive(to) => {
                    let self_color = board.get_data(from).unwrap().color.clone();
                    let revived_piece = board.remove_last_dead_with_color(&self_color);
                    board.get_mut(&to).unwrap().replace(revived_piece);
                }
            }
        } else {
            panic!("Non paladin ability info in paladin ability")
        }
    }
}

pub struct Ram;

impl Ability for Ram {
    fn data(&self) -> AbilityData {
        AbilityData {
            cooldown: Time::turns(4),
            cost: Mana(0),
        }
    }

    fn can_use(_board: &Board, _from: &Pos, info: &Info) -> bool {
        matches!(info, Info::Direction(_))
    }

    fn r#use(board: &mut Board, from: &Pos, info: Info) {
        if let Info::Direction(direction) = info {
            let strength = &board.get_data(from).unwrap().get_strength();
            let raycast = board.ray_cast(from, None, &(&direction).into(), |tile| {
                tile.has_piece() || tile.piece.is_impenetrable(strength)
            });

            if let Some(ref collision) = raycast.collision {
                let charge = raycast.len() / 5 + 1;
                // if collision is impenetrable, just stay in front of that piece
                // either, kill that piece and continue until charge is 0 or the next piece is impenetrable
                if board
                    .get(collision)
                    .unwrap()
                    .piece
                    .is_impenetrable(strength)
                {
                    let ram = board.get_mut(from).unwrap().remove();
                    board.get_mut(raycast.last().unwrap()).unwrap().replace(ram);
                } else {
                    let ram = board.get_mut(from).unwrap().remove();
                    let mut to = collision.clone();
                    board.attack_piece(from, collision);
                    for i in 0..charge {
                        let prev = to.clone();
                        to = to.direction_shift(&direction).unwrap();
                        if board.get(&to).unwrap().piece.is_impenetrable(strength) {
                            board.get_mut(&prev).unwrap().replace(ram);
                            break;
                        }
                        board.attack_piece(from, &to);
                        if i == charge - 1 {
                            board.get_mut(&to).unwrap().replace(ram);
                            break;
                        }
                    }
                }
            } else {
                // no colission, put the ram at the last position (the edge of the board)
                let ram = board.get_mut(from).unwrap().remove();
                board.get_mut(raycast.last().unwrap()).unwrap().replace(ram);
            }
        } else {
            panic!("Non direction info for ram")
        }
    }
}

/// The ability of the Shield Bearer is to give nearby allies the impeneatrable Type.
pub struct ShieldBearer;

impl Ability for ShieldBearer {
    fn data(&self) -> AbilityData {
        AbilityData {
            cooldown: Time::rounds(15),
            cost: Mana(0),
        }
    }

    fn r#use(board: &mut Board, from: &Pos, _info: Info) {
        board
            .tiles
            .iter_mut()
            .filter(|tile| pattern::king(from, tile.pos()))
            .for_each(|tile| {
                tile.piece.add_type(Type::Impenetrable(1)); // TODO: add type or add to type
            });
    }

    fn can_use(_board: &Board, _from: &Pos, _info: &Info) -> bool {
        true
    }
}

pub struct Ship;

impl Ability for Ship {
    fn data(&self) -> AbilityData {
        AbilityData {
            cooldown: Time::rounds(12),
            cost: Mana(0),
        }
    }

    fn can_use(_board: &Board, _from: &Pos, _info: &Info) -> bool {
        true
    }

    fn r#use(board: &mut Board, from: &Pos, _info: Info) {
        for subdir in vec![
            Direction::E
                .related_subdirections()
                .into_iter()
                .collect::<Vec<_>>(),
            Direction::W
                .related_subdirections()
                .into_iter()
                .collect::<Vec<_>>(),
        ]
        .into_iter()
        .flatten()
        {
            if let Some(attack_point) = from.subdirection_shift(&subdir) {
                board.attack_piece(from, &attack_point);
            }
        }
    }
}

pub struct SuperPawn;

impl Ability for SuperPawn {
    fn data(&self) -> AbilityData {
        AbilityData {
            cooldown: Time::rounds(10),
            cost: Mana(0),
        }
    }

    fn can_use(board: &Board, from: &Pos, _info: &Info) -> bool {
        let piece = &board.get(from).unwrap().piece;
        !piece.is_immune() && !piece.is_impenetrable(&10)
    }

    fn r#use(board: &mut Board, from: &Pos, _info: Info) {
        let piece = &mut board.get_mut(from).unwrap().piece;
        piece.add_type(Type::Immune);
        piece.add_type(Type::Impenetrable(10))
    }
}

pub struct TeslaTower;

impl Ability for TeslaTower {
    fn data(&self) -> AbilityData {
        AbilityData {
            cooldown: Time::rounds(10),
            cost: Mana(1),
        }
    }

    fn can_use(_board: &Board, _from: &Pos, _info: &Info) -> bool {
        true
    }

    fn r#use(board: &mut Board, from: &Pos, _info: Info) {
        let color = &board.get_data(from).unwrap().color;
        board.add_event(Event::full(
            "Tesla Tower Ability".to_string(),
            Time::turns(2),
            from.clone(),
            vec![EventFunction::ApplyEffect(
                Effect::Deactivate(Time::rounds(6)),
                from.clone(),
                FilterFunction::trio(
                    FilterFunction::Square(3),
                    FilterFunction::IsType(Type::Structure),
                    FilterFunction::IsNotColor(color.clone()),
                ),
            )],
        ))
    }
}

pub struct Warlock;

impl Ability for Warlock {
    fn data(&self) -> AbilityData {
        AbilityData {
            cooldown: Time::rounds(5),
            cost: Mana(3),
        }
    }

    fn can_use(board: &Board, from: &Pos, _info: &Info) -> bool {
        board
            .iter_from_pattern(from, pattern::king)
            .any(|tile| tile.is_empty() && tile.buildable && tile.magic)
    }

    fn r#use(board: &mut Board, from: &Pos, _info: Info) {
        let color = board.get(from).unwrap().piece.color().unwrap().clone();
        for tile in board.iter_mut_from_pattern(from, pattern::king) {
            if tile.is_empty() && tile.buildable && tile.magic {
                tile.replace(Piece::portal(color.clone()));
            }
        }
    }
}

pub struct Portal;

impl Ability for Portal {
    fn data(&self) -> AbilityData {
        AbilityData {
            cooldown: Time::turns(1),
            cost: Mana(0),
        }
    }

    fn can_use(_board: &Board, _from: &Pos, _info: &Info) -> bool {
        true
    }

    fn r#use(_board: &mut Board, _from: &Pos, _info: Info) {
        unimplemented!("portal::use not implemented yet")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use() {
        // Create a new Board object
        let mut board = Board::default();

        // Add some rooks to the board at specific positions
        // (replace `Rook` and `Pos` with the actual types and constructors)
        board
            .get_mut(&Pos::new(1, 1))
            .unwrap()
            .replace(Piece::rook(Color::White));
        board
            .get_mut(&Pos::new(1, 2))
            .unwrap()
            .replace(Piece::rook(Color::White));
        board
            .get_mut(&Pos::new(2, 1))
            .unwrap()
            .replace(Piece::rook(Color::White));

        // Use the rook's ability
        Rook::r#use(&mut board, &Pos::new(1, 1), Info::Direction(Direction::N));

        // Check the state of the board to ensure the rooks have been moved correctly
        // (replace `get_rook` with the actual method to get a rook at a position)
        assert!(!board.get(&Pos::new(1, 1)).unwrap().has_rook());
        assert!(!board.get(&Pos::new(1, 2)).unwrap().has_rook());
        assert!(!board.get(&Pos::new(2, 1)).unwrap().has_rook());
        assert!(board.get(&Pos::new(1, 7)).unwrap().has_rook());
        assert!(board.get(&Pos::new(1, 6)).unwrap().has_rook());
        assert!(board.get(&Pos::new(2, 7)).unwrap().has_rook());
    }
}
