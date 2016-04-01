use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use iron::mime::*;
use iron::prelude::*;
use iron::status;
use ::models::*;
use rand;
use router::Router;
use std::io::Read;
use super::{radix_36_to_radix_10, html_redirect, html_response};
use urlencoded::UrlEncodedBody;

pub fn see_poll(req: &mut Request) -> IronResult<Response> {
	{
		let parsed = req.get_ref::<UrlEncodedBody>();
		let mut binding;
		match parsed {
			Ok(ref hashmap) => binding = hashmap,
			Err(ref e) => {}
		};
	}
	let id = req.extensions.get::<Router>()
		.unwrap()
		.find("value")
		.unwrap_or("0");

	println!("{:?}", radix_36_to_radix_10(id));
	Ok(Response::with((
		Mime(TopLevel::Text, SubLevel::Html, vec![]),
		status::Ok,
		"Hello world!"
	)))
}
