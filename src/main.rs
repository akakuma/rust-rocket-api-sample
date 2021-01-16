#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate serde;
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_contrib;

use dotenv::dotenv;

mod db;
mod models;
mod routes;
mod schema;

fn main() {
    dotenv().ok();
    rocket::ignite()
        .manage(db::connect())
        .mount(
            "/",
            routes![
                routes::index::ping,
                routes::users::find,
                routes::users::list,
                routes::users::create,
                routes::users::update,
            ]
        )
        .launch();
}
