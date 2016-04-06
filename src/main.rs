#![feature(custom_attribute, custom_derive, plugin)]
#![plugin(clippy, dotenv_macros, maud_macros)]
extern crate chrono;
extern crate cookie;
extern crate dotenv;
extern crate iron;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate maud;
extern crate mount;
extern crate oven;
extern crate postgres;
extern crate rand;
#[macro_use]
extern crate router;
extern crate staticfile;
extern crate time;
extern crate urlencoded;

mod dbcon;
mod controllers;
mod live;
mod response;
mod setup_chain;

use iron::{AfterMiddleware, typemap};
use iron::prelude::*;
use oven::RequestExt;
use oven::ResponseExt;

pub struct User;

impl typemap::Key for User { type Value = i64; }

impl AfterMiddleware for User {
	fn after(&self, req: &mut Request, mut res: Response) -> IronResult<Response> {
		if let Some(_) = req.get_cookie("user") {
			res.set_cookie(cookie::Cookie::new(
				"hey".into(), "ok".to_string()));
		}
		Ok(res)
	}
}

fn main() {
	match env_logger::init() {
		Ok(()) => {}
		Err(_) => {
			println!("Error: Logger was already started");
			return;
		}
	}

	live::read_handler();

	let handler = setup_chain::get_middleware();

	info!("Running the server...");
	match Iron::new(handler).http("localhost:3000") {
		Ok(server) => {
			info!("Server started: {:?}", server);
		}
		Err(err) => {
			error!("Could not start the server, {:?}", err);
		}
	}
}
