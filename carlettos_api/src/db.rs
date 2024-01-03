use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::engine::any::Any;
use surrealdb::opt::PatchOp;
use surrealdb::sql::Object;
use surrealdb::sql::Thing;
use surrealdb::sql::Value;
use surrealdb::Surreal;

use crate::prelude;
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
    pub async fn add_task(&self, title: String) -> Result<Object, prelude::Error> {
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

    pub async fn get_task(&self, id: String) -> Result<Task, prelude::Error> {
        if let Some(task) = self.db.select(("tasks", &id)).await? {
            Ok(task)
        } else {
            Err(prelude::Error::ValueNotFound(id))
        }
    }

    pub async fn get_all_tasks(&self) -> Result<Vec<Task>, prelude::Error> {
        let mut tasks: Vec<Task> = self.db.select("tasks").await?;
        tasks.sort_by_key(|task| task.created_at.unwrap());
        Ok(tasks)
    }

    pub async fn toggle_task(&self, id: String) -> Result<AffectedRows, prelude::Error> {
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
        let _: Option<Task> = self.db.delete(("tasks", id)).await?;

        Ok(AffectedRows { rows_affected: 1 })
    }
}
