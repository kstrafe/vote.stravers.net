pub mod createpoll;
pub mod fileloader;
pub mod frontpage;
pub mod vote;
pub mod poll;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use iron::mime::*;
use iron::modifiers::Redirect;
use iron::prelude::*;
use iron::status;
use iron::Url;
use std::env;

fn html_redirect(content: String) -> Response {
	let mut temp = String::from("http://localhost:3000");
	temp.push_str(&content);
	Response::with((
		status::Found,
		Redirect(Url::parse(&temp).expect("Could not parse url"))
	))
}

pub fn html_response(content: String) -> Response {
	Response::with((
		Mime(TopLevel::Text, SubLevel::Html, vec![]),
		status::Ok,
		content
	))
}

fn generate_header() -> String {
	let mut buffer = String::new();
	html!(buffer, {
		head {
			style { r#"@import url()"# }
			link rel="icon" type="image/png" href="/storage/favicon48x48.png" /
		}
	});
	buffer
}


fn find(code: &[u8], value: u8) -> usize {
	for i in 0..code.len() {
		if code[i] == value {
			return i;
		}
	}
	panic!("Invalid base provided");
}

pub fn establish_connection() -> PgConnection {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL")
		.expect("DATABASE_URL must be set in the .env");
	PgConnection::establish(&database_url)
		.expect(&format!("Error connecting to {}", database_url))
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
	}
	num
}


