use chess_api::Board;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

pub use crate::error::Error;
pub struct W<T>(pub T);

#[derive(Serialize, Deserialize, Debug)]
pub struct IdBoard {
    pub id: Thing,
    pub board: Board,
}

impl From<Board> for IdBoard {
    fn from(value: Board) -> Self {
        IdBoard {
            id: Thing {
                tb: "chess".to_string(),
                id: surrealdb::sql::Id::Number(0),
            },
            board: value,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vote {
    pub id: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThingVotes {
    pub id: Thing, // caso tal, poner Thing como id
    pub votes: Vec<Vote>,
}

impl ThingVotes {
    pub fn remove(&mut self, vote_id: usize) {
        let index = self.votes.iter().position(|vote| vote.id == vote_id);
        if let Some(index) = index {
            self.votes.remove(index);
        }
    }

    pub fn add(&mut self, vote_id: usize) {
        self.votes.push(Vote { id: vote_id });
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Votes {
    pub id: String,
    pub votes: Vec<Vote>,
}

impl From<ThingVotes> for Votes {
    fn from(value: ThingVotes) -> Self {
        Votes {
            id: value.id.id.to_raw(),
            votes: value.votes,
        }
    }
}
