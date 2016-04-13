use iron::prelude::*;
use iron::status;
use cookie;
use oven::RequestExt;
use oven::ResponseExt;
use super::views::{render, render_not_found};
use super::models::create_poll;
use middleware::Body;

pub fn index(req: &mut Request) -> IronResult<Response> {
	req.extensions.insert::<Body>(render());
	create_poll(req);
	let cookie = req.get_cookie("hey");
	let mut resp = Response::with((
		status::Ok
	));
	let mut nextval = 1i32;
	if let Some(value) = cookie {
		let val = &value.value;
		if let Ok(val) = val.parse::<i32>() {
			nextval += val;
		}
	}
	resp.set_cookie(cookie::Cookie::new(
		"hey".into(), nextval.to_string()));
	Ok(resp)
}

trait SetA {
	fn set_title(&mut self, title: &str);
}

impl SetA for Response {
	fn set_title(&mut self, title: &str) {
		self.extensions.insert::<Body>(title.into());
	}
}

pub fn not_found(req: &mut Request) -> IronResult<Response> {
	req.extensions.insert::<Body>(render_not_found());
	Ok(Response::with((
		status::Ok
	)))
}
