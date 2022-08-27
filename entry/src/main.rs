mod model;
mod routes;
mod service;

use routes::api_routes;

use crate::service::SQLiteDAO;

#[macro_use]
extern crate rocket;

pub struct AppConfig {
    pub dao: SQLiteDAO,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", api_routes())
        .manage(AppConfig { dao: get_dao() })
}

fn get_dao() -> SQLiteDAO {
    if let Ok(p) = std::env::var("DB_PATH") {
        SQLiteDAO::new(&p)
    } else {
        SQLiteDAO::default()
    }
}
