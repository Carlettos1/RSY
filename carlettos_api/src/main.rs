#[macro_use]
extern crate rocket;

use chess_api::Board;
use cors::CORS;
use db::{AffectedRows, Task, DB};
use prelude::{LeaderboardEntry, Votes};
use rocket::{serde::json::Json, State};
use serde::Serialize;

use std::{
    io::{self, ErrorKind},
    sync::Arc,
};

pub mod error;
pub mod prelude;
pub mod utils {
    pub mod macros;
    pub mod try_froms;
}
pub mod cors;
pub mod db;

#[derive(Debug, Serialize)]
struct AuthParams<'a> {
    email: &'a str,
    password: &'a str,
}

#[post("/task/<title>")]
async fn add_task(title: String, db: &State<DB>) -> Result<Json<Task>, io::Error> {
    let task = db
        .add_task(title)
        .await
        .map_err(|_| io::Error::new(ErrorKind::Other, "Unable to create task"))?;
    Ok(Json(task.into()))
}

#[get("/task/<id>")]
async fn get_task(id: String, db: &State<DB>) -> Result<Json<Task>, io::Error> {
    let task = db
        .get_task(id)
        .await
        .map_err(|_| io::Error::new(ErrorKind::Other, "Unable to fetch task"))?;
    Ok(Json(task))
}

#[get("/tasks")]
async fn get_all_tasks(db: &State<DB>) -> Result<Json<Vec<Task>>, io::Error> {
    let tasks = db
        .get_all_tasks()
        .await
        .map_err(|_| io::Error::new(ErrorKind::Other, "Unable to fetch all task"))?;
    Ok(Json(tasks))
}

#[patch("/task/<id>")]
async fn toggle_task(id: String, db: &State<DB>) -> Result<Json<AffectedRows>, io::Error> {
    let affected_rows = db
        .toggle_task(id)
        .await
        .map_err(|e| io::Error::new(ErrorKind::Other, e.to_string()))?;
    Ok(Json(affected_rows))
}

#[delete("/task/<id>")]
async fn delete_task(id: String, db: &State<DB>) -> Result<Json<AffectedRows>, io::Error> {
    let affected_rows = db
        .delete_task(id)
        .await
        .map_err(|_| io::Error::new(ErrorKind::Other, "Unable to delete task"))?;
    Ok(Json(affected_rows))
}

#[get("/chess")]
async fn get_chess_game(db: &State<DB>) -> Result<Json<Board>, io::Error> {
    let board = db
        .get_chess_game()
        .await
        .map_err(|_| io::Error::new(ErrorKind::Other, "Unable to get chess game"))?;
    Ok(Json(board.board))
}

#[patch("/chess/<json>")]
async fn update_chess_game(json: String, db: &State<DB>) -> Result<Json<Board>, io::Error> {
    let board = db
        .update_chess_game(serde_json::from_str::<Board>(&json).unwrap().into())
        .await
        .map_err(|_| io::Error::new(ErrorKind::Other, "Unable to update chess game"))?;
    Ok(Json(board.board))
}

#[get("/votes/<id>")]
async fn get_votes(id: String, db: &State<DB>) -> Result<Json<Votes>, io::Error> {
    let votes = db.get_votes(id).await.map_err(io::Error::other)?;
    Ok(Json(votes.into()))
}

#[patch("/votes/add/<id>/<vote_id>")]
async fn add_vote(id: String, vote_id: usize, db: &State<DB>) -> Result<Json<Votes>, io::Error> {
    let votes = db.add_vote(id, vote_id).await.map_err(io::Error::other)?;
    Ok(Json(votes.into()))
}

#[patch("/votes/remove/<id>/<vote_id>")]
async fn remove_vote(id: String, vote_id: usize, db: &State<DB>) -> Result<Json<Votes>, io::Error> {
    let votes = db
        .remove_vote(id, vote_id)
        .await
        .map_err(io::Error::other)?;
    Ok(Json(votes.into()))
}

#[post("/c2048/highscores/<name>/<score>/<max_tile>/<min_energy>/<max_energy>")]
async fn add_highscore(
    name: String,
    score: usize,
    max_tile: usize,
    min_energy: isize,
    max_energy: isize,
    db: &State<DB>,
) -> Result<Json<LeaderboardEntry>, io::Error> {
    let highscore = db
        .add_highscore(name, score, max_tile, min_energy, max_energy)
        .await
        .map_err(io::Error::other)?;
    Ok(Json(highscore))
}

#[get("/c2048/highscores")]
async fn get_highscores(db: &State<DB>) -> Result<Json<Vec<LeaderboardEntry>>, io::Error> {
    let highscores = db.get_highscores().await.map_err(io::Error::other)?;
    Ok(Json(highscores))
}

async fn connect(db: &DB) -> Result<(), prelude::Error> {
    db.db.use_ns("root").await?;
    db.db.use_db("database").await?;
    db.root_signin().await?;
    Ok(())
}

#[launch]
async fn rocket() -> _ {
    let db = Arc::new(
        surrealdb::engine::any::connect(
            std::env::var("DB_IP").expect("DB_IP varenv need to be setted"),
        )
        .await
        .unwrap(),
    );

    let db = DB { db };
    connect(&db).await.unwrap();

    // this should create a game if not exist, if exist, will do nothing
    db.create_chess_game().await.unwrap();

    rocket::build()
        .mount(
            "/",
            routes![
                add_task,
                get_task,
                get_all_tasks,
                toggle_task,
                delete_task,
                get_chess_game,
                update_chess_game,
                get_votes,
                add_vote,
                remove_vote,
                get_highscores,
                add_highscore,
            ],
        )
        .attach(CORS)
        .manage(db)
}
