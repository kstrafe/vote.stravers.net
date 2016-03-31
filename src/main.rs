#![feature(custom_attribute, custom_derive, plugin)]
#![plugin(clippy, diesel_codegen, dotenv_macros, maud_macros)]
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate iron;
extern crate maud;
extern crate router;

mod index;
mod models;
mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use iron::prelude::*;
use router::Router;
use self::index::*;
use self::models::*;
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

fn setup_router() -> Router {
	let mut router = Router::new();
	router.get("/", handler);
	router.get("/storage/:filename", image);
	router
}

fn main() {
	use schema::posts::dsl::*;

	let con = establish_connection();

	create_post(&con, "Hello world", "Today is a good day!");

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
