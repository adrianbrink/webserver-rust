#[macro_use]
extern crate nickel;
extern crate rustc_serialize;
extern crate sendgrid;
extern crate dotenv;

use nickel::{Nickel, HttpRouter, MediaType};
use rustc_serialize::json;
use sendgrid::v3::{Email, Content, Personalization, SGMailV3, V3Sender};
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let mut m = SGMailV3::new();

    let mut e = Email::new();
    e.set_email("adrian@avogenie.com");

    let mut e1 = Email::new();
    e1.set_email("john@avogeniee.com");

    let mut c = Content::new();
    c.set_content_type("text/html");
    c.set_value("Fuck yeah");

    let mut p = Personalization::new();
    p.add_to(e.clone());

    m.set_from(e1.clone());
    m.set_subject("Fuck yeah from rust");
    m.add_content(c);
    m.add_personalization(p);

    // let mut env_vars = ::std::env::vars();
    // let api_key = env_vars.find(|v| v.0 == "SENDGRID_API_KEY").unwrap();
    let api_key = env::var("SENDGRID_API_KEY").expect("SENDGRID_API_KEY should be set.");
    // let api_key = "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".to_string();
    let sender = V3Sender::new(api_key);
    let code = sender.send(&m);
    println!("{:?}", code);

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