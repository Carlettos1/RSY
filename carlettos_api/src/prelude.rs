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
