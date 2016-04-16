use iron::prelude::*;
use iron::status;
use cookie;
use oven::RequestExt;
use oven::ResponseExt;
use super::views::{render, render_not_found};
use super::models::create_poll;
use middleware::Body;

/**
	Idea for a new XML-like tree language:
	TOML without [tag."øther_tag"].
	TOML isn't nest-friendly. I kinda like Maud tho:

	body {
		a href="#" { ABC }
		div {}
		a href="#" { DEF }
	}

	<body>
		<a href="#"> ABC </a>
		<div></div>
		<a href="#"> DEF </a>
	</body>

	servers {
		alpha {
			name = "cool"
			ip = "123.44.43.32"
		} beta {
			name = "cool2"
			ip = "123.44.43.31"
		}
	}

	[servers]
		[servers.alpha]
		name = "cool"
		ip = "123.44.43.32"
		[severs.beta]
		name = "cool2"
		ip = "123.44.43.31"

*/

pub fn index(req: &mut Request) -> IronResult<Response> {
	trace!("Index page");
	req.set_body(&render());
	req.set_title("Hello");
	// create_poll(req);
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
	Ok(resp)
}

trait SetA {
	fn set_title(&mut self, title: &str);
	fn set_body(&mut self, body: &str);
}

impl<'a, 'b> SetA for Request<'a, 'b> {
	fn set_title(&mut self, title: &str) {
		self.extensions.insert::<Body>(title.into());
	}
	fn set_body(&mut self, body: &str) {
		self.extensions.insert::<Body>(body.into());
	}
}

pub fn not_found(req: &mut Request) -> IronResult<Response> {
	trace!("Not found");
	req.set_body(&render_not_found());
	Ok(Response::with((
		status::Ok
	)))
}
