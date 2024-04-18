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

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Vote {
    pub id: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct Votes {
    //#[serde(deserialize_with = "deserialize_id")]
    pub id: String,
    pub votes: Vec<Vote>,
}

impl Votes {
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

#[allow(dead_code)]
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Check {
    Certain,
    Checking,
    #[default]
    None,
}

impl Check {
    pub fn to_class(&self) -> String {
        match self {
            Check::None => "",
            Check::Certain => "certain",
            Check::Checking => "checking",
        }
        .to_string()
    }

    pub fn update_from_votes(votes: &[Vote]) -> Vec<Self> {
        let mut checks = vec![Check::None; 6];
        for vote in votes.iter() {
            checks[vote.id] = Check::Certain;
        }
        checks
    }
}
