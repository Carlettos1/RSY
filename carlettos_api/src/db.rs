use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::engine::any::Any;
use surrealdb::sql::Object;
use surrealdb::sql::Value;
use surrealdb::Surreal;

use crate::prelude;
use crate::utils::macros::map;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub title: String,
    pub completed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
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
    pub ds: Arc<Surreal<Any>>,
}

impl DB {
    pub async fn add_task(&self, title: String) -> Result<Object, prelude::Error> {
        let query = "CREATE tasks SET title = $title, completed = false, created_at = time::now()";
        let result = self
            .ds
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

    pub async fn get_task(&self, id: String) -> Result<Object, prelude::Error> {
        let query = "SELECT * FROM $th";
        let result = self
            .ds
            .query(query)
            .bind(("$th", id))
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

    pub async fn get_all_tasks(&self) -> Result<Vec<Object>, prelude::Error> {
        let query = "SELECT * FROM tasks ORDER BY created_at ASC;";
        let result = self.ds.query(query).await?.take::<Value>(0)?;

        match result {
            Value::Array(array) => Ok(array.into_iter().map(|v| v.try_into().unwrap()).collect()),
            _ => Err(prelude::Error::XValueNotOfType("Value is not an array")),
        }
    }

    pub async fn toggle_task(&self, id: String) -> Result<AffectedRows, prelude::Error> {
        let query = "UPDATE $th SET completed = function() { return !this.completed; }";
        self.ds.query(query).bind(("$th", id)).await?;

        Ok(AffectedRows { rows_affected: 1 })
    }

    pub async fn delete_task(&self, id: String) -> Result<AffectedRows, prelude::Error> {
        let query = "Delete $th";
        self.ds.query(query).bind(("$th", id)).await?;

        Ok(AffectedRows { rows_affected: 1 })
    }
}
