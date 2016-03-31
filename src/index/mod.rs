pub mod createpoll;
pub mod fileloader;
pub mod frontpage;
pub mod vote;

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

fn html_response(content: String) -> Response {
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

pub fn establish_connection() -> PgConnection {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL")
		.expect("DATABASE_URL must be set in the .env");
	PgConnection::establish(&database_url)
		.expect(&format!("Error connecting to {}", database_url))
}
