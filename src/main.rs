#![feature(custom_attribute, custom_derive, plugin)]
#![plugin(clippy, dotenv_macros, maud_macros)]
extern crate chrono;
extern crate cookie;
extern crate dotenv;
extern crate iron;
extern crate maud;
extern crate oven;
extern crate postgres;
extern crate rand;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use oven::prelude::*;
use postgres::{Connection, SslMode};
use router::Router;

fn handle(req: &mut Request) -> IronResult<Response> {
	let cookie = req.get_cookie("hey");
	println!("HEY!");
	let mut resp = Response::new();
	let mut nextval = 1i32;
	if let Some(value) = cookie {
		let val = &value.value;
		if let Ok(val) = val.parse::<i32>() {
			nextval += val;
		}
	}
	println!("{}", nextval);
	resp.set_cookie(cookie::Cookie::new(
		"hey".into(), nextval.to_string()));
	Ok(Response::with((
		status::Ok,
		"Hello world"
	)))
}

fn main() {

	let conn = Connection::connect(
		"postgresql://kefin@localhost/diesel_demo",
		SslMode::None)
		.unwrap();

	for row in conn.query("select * from poll", &[])
	.unwrap().iter() {
		let x: i64 = row.get(0);
		println!("{}", x);
	}

	let router = Router::new();
	let mut chain = iron::middleware::Chain::new(handle);
	chain.link(oven::new(vec![]));

	println!("Server started on :3000");

	Iron::new(chain)
		.http("localhost:3000")
		.unwrap();
}
