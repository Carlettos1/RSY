use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Task {
    #[serde(deserialize_with = "deserialize_id")]
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

// This is for parsing rocket::Thing and retrieving only the id
#[allow(dead_code, non_snake_case)]
fn deserialize_id<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    #[derive(Deserialize)]
    struct Id {
        tb: String,
        id: SubId,
    }

    #[derive(Deserialize)]
    struct SubId {
        String: String,
    }

    let id = Id::deserialize(deserializer)?;
    Ok(id.id.String)
}

#[derive(Deserialize)]
pub struct AffectedRows {
    pub rows_affected: u64,
}

#[derive(Deserialize)]
pub struct RowId {
    pub id: String,
}
