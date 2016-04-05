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

#[derive(Clone, Debug)]
enum CommandType {
	Empty,
	Get(String),
	Set(String, String),
}

fn classify_string(command: &str) -> CommandType {
	let parts = &mut command.splitn(2, '=');
	match parts.clone().count() {
		0 => CommandType::Empty,
		1 => CommandType::Get(command.trim().into()),
		2 => {
			let error = "This can never happen because we check the amount of parts in the iterator. Defaulting to an Empty sequence";
			let left = match parts.next() {
				Some(left) => left.trim(),
				None => {
					error!("Left, {}", error);
					return CommandType::Empty;
				}
			};
			let right = match parts.next() {
				Some(right) => right.trim(),
				None => {
					error!("Right, {}", error);
					return CommandType::Empty;
				}
			};
			CommandType::Set(left.into(), right.into())
		}
		_ => {
			error!("Count not 0, 1, or 2. Indicating an splitn method error. Defaulting to no operation");
			CommandType::Empty
		}
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
		'looper: loop {
			let command = {
				let mut command = String::new();
				match stdin.read_line(&mut command) {
					Ok(bytes) => debug!("Read {} stdin bytes to string", bytes),
					Err(err) => {
						error!("Unable to read std: {:?},\nExiting interpreter", err);
						break 'looper;
					}
				}
				command
			};

			{
				let arg = classify_string(&command);
				match send.send(arg) {
					Ok(()) => debug!("Message sent"),
					Err(err) => error!("Unable to send to channel: {:?}", err),
				}
			}
		}
	});

	loop {
		use std::sync::mpsc::TryRecvError;
		match recv.try_recv() {
			Ok(message) => trace!("Gotten message {:?}", message),
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
