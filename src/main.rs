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

fn two() {
	use std::thread;
	use std::sync::mpsc::channel;
	use std::io::BufRead;
	use std::io;

	let (send, recv) = channel();
	let stdin = io::stdin();
	thread::spawn(move || {
		let mut stdin = stdin.lock();
		loop {
			let command = {
				let mut command = String::new();
				match stdin.read_line(&mut command) {
					Ok(_) => debug!("Read stdin to string"),
					Err(err) => error!("Unable to read std: {:?}", err),
				}
				command
			};
			match send.send(command) {
				Ok(()) => debug!("Message sent"),
				Err(err) => error!("Unable to channel: {:?}", err),
			}
		}
	});

	loop {
		use std::sync::mpsc::TryRecvError;
		match recv.try_recv() {
			Ok(message) => trace!("Gotten message {}", message),
			Err(TryRecvError::Empty) => {}
			Err(TryRecvError::Disconnected) => error!("Sender dc'd"),
		}
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

	two();

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
