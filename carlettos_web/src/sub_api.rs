use chess_api::Board;
use reqwasm::{http::Request, Error};

use crate::models::{AffectedRows, Task};

const BASE_URL: &str = "http://127.0.0.1:8080";

pub async fn get_chess_game() -> Result<Board, Error> {
    Request::get(&format!("{BASE_URL}/chess"))
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn update_chess_game(board: Board) -> Result<Board, Error> {
    Request::patch(&format!(
        "{BASE_URL}/chess/{}",
        serde_json::to_string(&board).unwrap()
    ))
    .send()
    .await
    .unwrap()
    .json()
    .await
}

pub async fn fetch_tasks() -> Result<Vec<Task>, Error> {
    Request::get(&format!("{BASE_URL}/tasks"))
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn create_task(title: &str) -> Result<Task, Error> {
    Request::post(&format!("{BASE_URL}/task/{title}"))
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn toggle_task(id: String) -> Result<AffectedRows, Error> {
    Request::patch(&format!("{BASE_URL}/task/{id}"))
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn delete_task(id: String) -> Result<AffectedRows, Error> {
    Request::delete(&format!("{BASE_URL}/task/{id}"))
        .send()
        .await
        .unwrap()
        .json()
        .await
}
