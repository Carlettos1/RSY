use std::env;

use chess_api::Board;
use lazy_static::lazy_static;
use reqwasm::{http::Request, Error};

use crate::models::{AffectedRows, Task, Votes};

lazy_static! {
    pub static ref API_IP: String = env::var("API_IP").expect("API_IP varenv not found");
}

pub async fn get_chess_game() -> Result<Board, Error> {
    Request::get(&format!("{}/chess", *API_IP))
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn update_chess_game(board: Board) -> Result<Board, Error> {
    Request::patch(&format!(
        "{}/chess/{}",
        *API_IP,
        serde_json::to_string(&board).unwrap()
    ))
    .send()
    .await
    .unwrap()
    .json()
    .await
}

pub async fn fetch_tasks() -> Result<Vec<Task>, Error> {
    Request::get(&format!("{}/tasks", *API_IP))
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn create_task(title: &str) -> Result<Task, Error> {
    Request::post(&format!("{}/task/{title}", *API_IP))
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn toggle_task(id: String) -> Result<AffectedRows, Error> {
    Request::patch(&format!("{}/task/{id}", *API_IP))
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn delete_task(id: String) -> Result<AffectedRows, Error> {
    Request::delete(&format!("{}/task/{id}", *API_IP))
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn get_votes(id: String) -> Result<Votes, Error> {
    Request::get(&format!("{}/votes/{id}", *API_IP))
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn add_vote(id: String, vote_id: usize) -> Result<Votes, Error> {
    Request::patch(&format!("{}/votes/add/{id}/{vote_id}", *API_IP))
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn remove_vote(id: String, vote_id: usize) -> Result<Votes, Error> {
    Request::patch(&format!("{}/votes/remove/{id}/{vote_id}", *API_IP))
        .send()
        .await
        .unwrap()
        .json()
        .await
}
