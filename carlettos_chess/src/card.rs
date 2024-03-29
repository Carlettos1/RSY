use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

use crate::{board::Mana, Time};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Card {
    // Summon,
    Knight,
    Rook,
    Warlock,
    // Board State
    Ice,
    Fire,
    AttackDemonic,
    Invulnerability,
    Revive,
    MoreMana,
    // Utility
    AddMovement,
    Mana,
}

impl Card {
    pub fn get_cost(&self) -> Mana {
        Mana(match self {
            Card::Knight => 2,
            Card::Rook => 0,
            Card::Warlock => 5,
            Card::Ice => 3,
            Card::Fire => 3,
            Card::AttackDemonic => 3,
            Card::Invulnerability => 5,
            Card::Revive => 4,
            Card::AddMovement => 1,
            Card::MoreMana => 2,
            Card::Mana => 1,
        })
    }

    pub fn tick(&mut self, time: &Time, place: &CardPlace) {
        match (self, place) {
            (Card::MoreMana, CardPlace::OnBoard) if time.is_round() => {
                // TODO: this should give 1 more mana every turn
            }
            _ => (),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Cards(pub Vec<Card>);

impl Cards {
    pub fn add(&mut self, card: Card) {
        self.0.push(card);
    }

    pub fn remove(&mut self, card: Card) -> Option<Card> {
        Some(self.0.remove(self.0.iter().position(|c| c == &card)?))
    }

    pub fn take(&mut self) -> Option<Card> {
        self.0.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn shuffle(&mut self) {
        self.0.shuffle(&mut thread_rng());
    }

    pub fn tick(&mut self, time: &Time, place: CardPlace) {
        self.0.iter_mut().for_each(|card| card.tick(time, &place));
    }
}

pub enum CardPlace {
    DiscardPile,
    OnBoard,
    Hand,
    Deck,
}
