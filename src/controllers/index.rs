use iron::prelude::*;
use iron::status;
use cookie;
use oven::RequestExt;
use oven::ResponseExt;
use super::views::render;
use middleware::DbCon;
use super::models::create_poll;

pub fn index(req: &mut Request) -> IronResult<Response> {
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

