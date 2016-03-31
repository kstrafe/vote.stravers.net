use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use iron::mime::*;
use iron::prelude::*;
use iron::status;
use ::models::*;
use rand;
use std::io::Read;
use super::{html_redirect, html_response};
use urlencoded::UrlEncodedBody;

fn back_to_start() -> IronResult<Response> {
	Ok(html_redirect("".into()))
}

pub fn create_poll(req: &mut Request) -> IronResult<Response> {
	macro_rules! invalid_options {
		($x:expr) => {{ $x.len() == 1 && $x[0].len() == 0 }}
	}
	let parsed = req.get_ref::<UrlEncodedBody>();
	let mut binding;
	match parsed {
		Ok(ref hashmap) => binding = hashmap,
		Err(ref e) => return back_to_start(),
	};
	if let Some(ref options) = binding.get("options") {
		if invalid_options!(options) {
			println!("Redirecting!");
			return back_to_start();
		} else {
			let con = super::establish_connection();
			make_poll(&con, &binding.get("description").unwrap()[0]);
		}
	} else {
		return back_to_start();
	}
	Ok(html_response("".into()))
}

fn make_poll(conn: &PgConnection, description: &str) -> Option<Poll> {
	use diesel::result::Error::DatabaseError;
	use schema::poll;

	fn generate_big_random() -> i64 {
		rand::random::<i64>()
	}

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

