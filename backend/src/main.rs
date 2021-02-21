#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize)]
struct Status {
    pub clans: Vec<String>,
    pub prize_pool: u64,
}

#[get("/")]
fn index() -> Json<Status> {
    Json(Status {
        clans: vec![
            "FaZe Clan".to_string(),
            "Sølvguttene".to_string(),
            "VUKUKARSK".to_string(),
        ],
        prize_pool: 2_000,
    })
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
