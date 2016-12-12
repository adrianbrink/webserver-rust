#[macro_use]
extern crate nickel;
extern crate rustc_serialize;
extern crate sendgrid;

use nickel::{Nickel, HttpRouter, MediaType};
use rustc_serialize::json;
use sendgrid::v3::SGMailV3;

fn main() {
    let mut m = SGMailV3::new();
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