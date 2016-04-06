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

mod controllers;
mod middleware;
mod setup_chain;

use iron::prelude::*;

fn main() {
	match env_logger::init() {
		Ok(()) => {}
		Err(_) => {
			println!("Error: Logger was already started");
			return;
		}
	}

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
