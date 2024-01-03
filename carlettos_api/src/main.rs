#[macro_use]
extern crate rocket;

use cors::CORS;
use db::{AffectedRows, Task, DB};
use rocket::{serde::json::Json, State};

use std::{
    io::{self, ErrorKind},
    sync::Arc,
};
use surrealdb::opt::auth::Root;

pub mod error;
pub mod prelude;
pub mod utils {
    pub mod macros;
    pub mod try_froms;
}
pub mod cors;
pub mod db;

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

async fn connect(db: &DB) -> Result<(), prelude::Error> {
    db.db.use_ns("root").await?;
    db.db.use_db("database").await?;

    db.db
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await
        .map(|_| ())
        .map_err(|e| e.into())
}

#[launch]
async fn rocket() -> _ {
    let db = Arc::new(
        surrealdb::engine::any::connect("ws://0.0.0.0:8000")
            .await
            .unwrap(),
    );

    let db = DB { db };
    connect(&db).await.unwrap();

    rocket::build()
        .mount(
            "/",
            routes![add_task, get_task, get_all_tasks, toggle_task, delete_task],
        )
        .attach(CORS)
        .manage(db)
}
