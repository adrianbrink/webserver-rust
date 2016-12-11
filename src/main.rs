#[macro_use]
extern crate nickel;

use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};

fn hello_world<'mv>(_req: &mut Request, res: Response<'mv>) -> MiddlewareResult<'mv> {
    res.send("Hello World")
}
fn main() {
    println!("Hello, world!");
    let mut server = Nickel::new();
    server.get("**", hello_world);
    server.listen("127.0.0.1:6767").unwrap();
}