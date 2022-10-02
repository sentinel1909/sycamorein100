// server main.rs
use rocket::fs::{FileServer, relative};
#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", FileServer::from(relative!("../frontend/dist")))
}
