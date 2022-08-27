use rocket::{Route, State};

use crate::{
    model::{DataAccessObject, Entry, Save},
    AppConfig,
};
use rocket::serde::json::Json;

#[get("/<id>")]
fn get_one(id: &str, cfg: &State<AppConfig>) -> Json<Option<Entry>> {
    Json(cfg.dao.get_one(id.to_owned()).unwrap())
}

#[get("/")]
fn get_all(cfg: &State<AppConfig>) -> Json<Vec<Entry>> {
    Json(cfg.dao.get_all().unwrap())
}

#[post("/<id>")]
fn save(id: Option<&str>, cfg: &State<AppConfig>) -> Json<Save> {
    cfg.dao.save(id.to_owned()).unwrap();

    unimplemented!()
}

pub fn api_routes() -> Vec<Route> {
    routes![get_one, get_all, save]
}
