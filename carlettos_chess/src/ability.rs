use core::panic;

use crate::{
    board::{Board, Mana, Tile},
    pattern,
    piece::Piece,
    Color, Direction, Info, Pos, Time,
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
