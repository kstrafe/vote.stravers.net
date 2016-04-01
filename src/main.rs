#![feature(custom_attribute, custom_derive, plugin)]
#![plugin(clippy, diesel_codegen, dotenv_macros, maud_macros)]
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate iron;
extern crate maud;
extern crate rand;
extern crate router;
extern crate urlencoded;

mod index;
mod models;
mod routing;
mod schema;

use diesel::prelude::*;
use iron::prelude::*;
use self::models::*;
use self::routing::setup_router;

fn main() {
	use schema::posts::dsl::*;

	let con = index::establish_connection();

	let res = posts
		.limit(5)
		.load::<Post>(&con)
		.expect("Could not load posts");

	let router = setup_router();

	println!("Server started on :3000");

	Iron::new(router)
		.http("localhost:3000")
		.unwrap();
}
