#![feature(custom_attribute, custom_derive, plugin)]
#![plugin(clippy, dotenv_macros, maud_macros)]
extern crate chrono;
extern crate dotenv;
extern crate iron;
extern crate maud;
extern crate postgres;
extern crate rand;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use postgres::{Connection, SslMode};

fn main() {
	use router::Router;

	let conn = Connection::connect("postgresql://kefin@localhost/diesel_demo",
		SslMode::None)
		.unwrap();
	for row in conn.query("select * from poll", &[])
		.unwrap().iter() {
		let x: i64 = row.get(0);
		println!("{}", x);
	}
	let router = Router::new();

	println!("Server started on :3000");

	Iron::new(router)
		.http("localhost:3000")
		.unwrap();
}
