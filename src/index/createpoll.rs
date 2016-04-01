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

fn continue_to_poll(id: i64) -> IronResult<Response> {
	println!("Serializing {}", id);
	let number = i64_to_str_radix_36(id);
	Ok(html_redirect(String::from("/") + &number))
}

fn find(code: &[u8], value: u8) -> usize {
	for i in 0..code.len() {
		if code[i] == value {
			return i;
		}
	}
	panic!("Invalid base provided");
}
pub fn radix_36_to_radix_10(mut text: &str) -> i64 {
	let mut num: i64 = 0;
	let j = text.len();
	let code = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-_";
	let uref: &[u8] = text.as_ref();
	for i in 0..j {
		println!("Index: {}", uref.len()-1-i);
		let temp = find(code, uref[uref.len() - 1 - i]) as usize
			* code.len().pow(i as u32);
		num += temp as i64;
		// text = text.substring(0,text.length()-1);
	}
	num
}

fn i64_to_str_radix_36(mut num: i64) -> String {
	let code = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-_";
	let mut text = String::new();
	let j = ((num as f32).ln()/(code.len() as f32).ln()).ceil() as i32;
	for i in 0..j {
		let mut pre = String::new();
		pre.push(code[num as usize % code.len()] as char);
		text = pre + &text;
		num /= code.len() as i64;
	}
	text
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
			if let Some(poll) = make_poll(&con, &binding.get("description").unwrap()[0]) {
				return continue_to_poll(poll.id);
			}
		}
	} else {
		return back_to_start();
	}
	back_to_start()
}

fn make_poll(conn: &PgConnection, description: &str) -> Option<Poll> {
	use diesel::result::Error::DatabaseError;
	use schema::poll;

	fn generate_big_random() -> i64 {
		use rand::distributions::{IndependentSample, Range};
		let range = Range::new(1_000_000_000, 10_000_000_000);
		let mut rng = rand::thread_rng();
		range.ind_sample(&mut rng)
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

