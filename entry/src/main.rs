mod model;

use crate::model::{Entry, Save};


#[macro_use]
extern crate rocket;

#[get("/")]
fn getall() -> Vec<Entry> {
    unimplemented!()
}

#[get("/<id>")]
fn getone(id: &str) -> Entry {
    unimplemented!()
}

#[post("/<id>")]
fn save(id: Option<&str>) -> Save {
    unimplemented!()
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![getall, getone, save])
}

