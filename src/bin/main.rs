#[macro_use]
extern crate nickel;
extern crate rustc_serialize;
extern crate postgres;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use nickel::{Nickel, HttpRouter, MediaType};
use postgres::{Connection, TlsMode};
use rustc_serialize::json;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

// #[derive(Debug, RustcEncodable)]
// struct Order {
//     id: i32,
//     total: f64,
//     currency: String,
//     status: String,
// }

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    let db_url = "postgresql://postgres@localhost:4444/postgres";
    let db = Connection::connect(db_url, TlsMode::None)
        .expect("Unable to connect to the database.");

    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/users",
               middleware! { |request, response| {
        format!("Hello from GET /users")
    }});

    router.post("/users/new",
                middleware! { |request, response| {
        format!("Hello from POST /users/new")
    }});

    router.delete("/users/:id",
                  middleware! { |request, response| {
        format!("Hello from DELETE /users/:id")
    }});

    server.utilize(router);
    server.listen("127.0.0.1:6767");
}