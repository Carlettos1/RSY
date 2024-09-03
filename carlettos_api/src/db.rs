use std::sync::Arc;

use chess_api::Board;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Root;
use surrealdb::opt::PatchOp;
use surrealdb::sql::Object;
use surrealdb::sql::Thing;
use surrealdb::sql::Value;
use surrealdb::Surreal;

use crate::prelude;
use crate::prelude::IdBoard;
use crate::prelude::LeaderboardEntry;
use crate::prelude::ThingVotes;
use crate::prelude::Vote;
use crate::utils::macros::map;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub title: String,
    pub completed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn from_obj(obj: Object) -> Option<Self> {
        Some(Self {
            id: Some({
                if let Value::Thing(thing) = obj.get("id")? {
                    thing.clone()
                } else {
                    return None;
                }
            }),
            title: {
                if let Value::Strand(string) = obj.get("title")? {
                    string.0.clone()
                } else {
                    return None;
                }
            },
            completed: {
                if let Value::Bool(b) = obj.get("completed")? {
                    *b
                } else {
                    return None;
                }
            },
            created_at: Some({
                if let Value::Datetime(dt) = obj.get("created_at")? {
                    dt.0
                } else {
                    return None;
                }
            }),
        })
    }
}

impl From<Object> for Task {
    fn from(val: Object) -> Self {
        Task::from_obj(val).unwrap()
    }
}

impl From<Task> for Value {
    fn from(task: Task) -> Self {
        match task.id {
            Some(t) => map![
                "id".into() => t.into(),
                "title".into() => task.title.into(),
                "completed".into() => task.completed.into()
            ]
            .into(),
            None => map![
                "title".into() => task.title.into(),
                "completed".into() => task.completed.into()
            ]
            .into(),
        }
    }
}

pub trait Creatable: Into<Value> {}
impl Creatable for Task {}

#[derive(Debug, Serialize, Deserialize)]
pub struct RowId {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AffectedRows {
    pub rows_affected: u64,
}

pub struct DB {
    pub db: Arc<Surreal<Any>>,
}

impl DB {
    pub async fn connect(&self) -> Result<(), prelude::Error> {
        let a: Result<Vec<Task>, surrealdb::Error> = self.db.select("tasks").await;
        if a.is_err() {
            self.root_signin().await?;
        }
        Ok(())
    }

    pub async fn root_signin(&self) -> Result<(), prelude::Error> {
        self.db
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;
        Ok(())
    }

    pub async fn add_task(&self, title: String) -> Result<Object, prelude::Error> {
        self.connect().await?;
        let query = "CREATE tasks SET title = $title, completed = false, created_at = time::now();";
        let result = self
            .db
            .query(query)
            .bind(("title", title))
            .await?
            .take::<Value>(0)?;

        if !result.is_array() {
            return Err(prelude::Error::XValueNotOfType("Value is not an array"));
        }

        match result.first() {
            Value::Object(obj) => Ok(obj),
            _ => Err(prelude::Error::XValueNotOfType("Value is not an object")),
        }
    }

    pub async fn create_chess_game(&self) -> Result<IdBoard, prelude::Error> {
        if let Some(board) = self.db.select(("chess", 0)).await? {
            Ok(board)
        } else {
            let query = "CREATE chess SET board = $json, id = 0;";
            let result = self
                .db
                .query(query)
                .bind(("json", Board::default()))
                .await?
                .take::<Option<IdBoard>>(0)?;
            if let Some(board) = result {
                Ok(board)
            } else {
                Err(prelude::Error::ValueNotFound(
                    "Couldn't create chess games".to_string(),
                ))
            }
        }
    }

    pub async fn get_task(&self, id: String) -> Result<Task, prelude::Error> {
        self.connect().await?;
        if let Some(task) = self.db.select(("tasks", &id)).await? {
            Ok(task)
        } else {
            Err(prelude::Error::ValueNotFound(id))
        }
    }

    pub async fn get_chess_game(&self) -> Result<IdBoard, prelude::Error> {
        self.connect().await?;
        if let Some(board) = self.db.select(("chess", 0)).await? {
            Ok(board)
        } else {
            Err(prelude::Error::ValueNotFound(
                "Chess game not found".to_string(),
            ))
        }
    }

    pub async fn update_chess_game(&self, board: IdBoard) -> Result<IdBoard, prelude::Error> {
        self.connect().await?;
        if let Some(board) = self.db.update(("chess", 0)).content(board).await? {
            Ok(board)
        } else {
            Err(prelude::Error::ValueNotFound(
                "Chess game cannot be updated".to_string(),
            ))
        }
    }

    pub async fn get_all_tasks(&self) -> Result<Vec<Task>, prelude::Error> {
        self.connect().await?;
        let a: Result<Vec<Task>, surrealdb::Error> = self.db.select("tasks").await;
        println!("{:?}", a);
        let mut tasks: Vec<Task> = a?;
        tasks.sort_by_key(|task| task.created_at.unwrap());
        Ok(tasks)
    }

    pub async fn toggle_task(&self, id: String) -> Result<AffectedRows, prelude::Error> {
        self.connect().await?;
        let task = self.get_task(id.clone()).await?;
        if self
            .db
            .update::<Option<Task>>(("tasks", &id))
            .patch(PatchOp::replace("completed", !task.completed))
            .await?
            .is_some()
        {
            Ok(AffectedRows { rows_affected: 1 })
        } else {
            Err(prelude::Error::ValueNotFound(id))
        }
    }

    pub async fn delete_task(&self, id: String) -> Result<AffectedRows, prelude::Error> {
        self.connect().await?;
        let _: Option<Task> = self.db.delete(("tasks", id)).await?;

        Ok(AffectedRows { rows_affected: 1 })
    }

    pub async fn get_votes(&self, id: String) -> Result<ThingVotes, prelude::Error> {
        self.connect().await?;
        if let Some(votes) = self.db.select(("vote", id.clone())).await? {
            Ok(votes)
        } else {
            let query = "CREATE vote SET id = $id, votes = $votes;";
            let result = self
                .db
                .query(query)
                .bind(("id", id))
                .bind(("votes", Vec::<Vote>::new()))
                .await?
                .take::<Option<ThingVotes>>(0)?;
            if let Some(votes) = result {
                Ok(votes)
            } else {
                Err(prelude::Error::ValueNotFound(
                    "Couldn't create votes".to_string(),
                ))
            }
        }
    }

    pub async fn add_vote(&self, id: String, vote_id: usize) -> Result<ThingVotes, prelude::Error> {
        self.connect().await?;
        let mut votes = self.get_votes(id.clone()).await?;
        if votes.votes.len() < 3 {
            votes.add(vote_id);
            self.db
                .update::<Option<ThingVotes>>(("vote", &id))
                .patch(PatchOp::replace("votes", votes.votes.clone()))
                .await?;
        }
        Ok(votes)
    }

    pub async fn remove_vote(
        &self,
        id: String,
        vote_id: usize,
    ) -> Result<ThingVotes, prelude::Error> {
        self.connect().await?;
        let mut votes = self.get_votes(id.clone()).await?;
        votes.remove(vote_id);
        if self
            .db
            .update::<Option<ThingVotes>>(("vote", &id))
            .patch(PatchOp::replace("votes", votes.votes.clone()))
            .await?
            .is_some()
        {
            Ok(votes)
        } else {
            Err(prelude::Error::ValueNotFound(id))
        }
    }

    pub async fn get_highscores(&self) -> Result<Vec<LeaderboardEntry>, prelude::Error> {
        self.connect().await?;
        let highscores = self.db.select("c2048").await?;
        Ok(highscores)
    }

    pub async fn add_highscore(
        &self,
        name: String,
        score: usize,
        max_tile: usize,
        min_energy: isize,
        max_energy: isize,
    ) -> Result<LeaderboardEntry, prelude::Error> {
        self.connect().await?;
        let mut hs = self
            .db
            .create("c2048")
            .content(LeaderboardEntry {
                name,
                score,
                max_tile,
                min_energy,
                max_energy,
            })
            .await?;
        assert!(hs.len() == 1);
        Ok(hs.remove(0))
    }
}
