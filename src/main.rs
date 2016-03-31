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
use diesel::pg::PgConnection;
use dotenv::dotenv;
use iron::prelude::*;
use self::models::*;
use self::routing::setup_router;
use std::env;

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
	use schema::posts;

	let new_post = NewPost {
		title: title,
		body: body,
	};

	diesel::insert(&new_post).into(posts::table)
		.get_result(conn)
		.expect("Can't save new post")
}

pub fn establish_connection() -> PgConnection {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL")
		.expect("DATABASE_URL must be set in the .env");
	PgConnection::establish(&database_url)
		.expect(&format!("Error connecting to {}", database_url))
}

fn generate_big_random() -> i64 {
	rand::random::<i64>()
}

pub fn create_poll(conn: &PgConnection, description: &str) -> Option<Poll> {
	use diesel::result::Error::DatabaseError;
	use schema::poll;

	for _ in 1..1000 {
		let new_poll = NewPoll {
			id: generate_big_random(),
			description: description,
		};
		let results = diesel::insert(&new_poll).into(poll::table).get_result(conn);
		match results {
			Ok(poll) => return Some(poll),
			Err(DatabaseError(_)) => {}
			_ => return None,
		}
	}
	None
}

fn main() {
	use schema::posts::dsl::*;

	let con = establish_connection();

	create_post(&con, "Hello world", "Today is a good day!");
	create_poll(&con, "Vote!");

	let res = posts
		.limit(5)
		.load::<Post>(&con)
		.expect("Could not load posts");

	println!("Posts: {}", res.len());

	let router = setup_router();

	println!("Server started on :3000");

	Iron::new(router)
		.http("localhost:3000")
		.unwrap();
}
