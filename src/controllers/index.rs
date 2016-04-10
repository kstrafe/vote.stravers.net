use iron::prelude::*;
use iron::status;
use cookie;
use oven::RequestExt;
use oven::ResponseExt;
use super::views::{render, render_not_found};
use super::models::create_poll;
use middleware::Body;

pub fn index(req: &mut Request) -> IronResult<Response> {
	req.extensions.insert::<Body>("Hello!".into());
	create_poll(req);
	let cookie = req.get_cookie("hey");
	let mut resp = Response::with((
		status::Ok,
		render(),
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

pub fn not_found(req: &mut Request) -> IronResult<Response> {
	Ok(Response::with((
		status::Ok,
		render_not_found()
	)))
}
