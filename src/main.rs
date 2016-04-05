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
mod response;

use dbcon::DbCon;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::prelude::*;
use iron::status;
use mount::Mount;
use oven::prelude::*;
use postgres::{Connection, SslMode};
use router::Router;
use staticfile::Static;
use std::path::Path;
use urlencoded::{QueryMap, UrlEncodedBody, UrlEncodedQuery};

pub struct User;

impl typemap::Key for User { type Value = i64; }

impl AfterMiddleware for User {
	fn after(&self, req: &mut Request, mut res: Response) -> IronResult<Response> {
		if let Some(cookie) = req.get_cookie("user") {
			res.set_cookie(cookie::Cookie::new(
				"hey".into(), "ok".to_string()));
		}
		Ok(res)
	}
}

fn get<'a>(req: &'a mut Request) -> Option<&'a QueryMap> {
	match req.get_ref::<UrlEncodedQuery>() {
		Ok(hashmap) => {
			Some(hashmap)
		}
		Err(_) => None,
	}
}

fn post<'a>(req: &'a mut Request) -> Option<&'a QueryMap> {
	match req.get_ref::<UrlEncodedBody>() {
	Ok(hashmap) => {
		Some(hashmap)
	}
		Err(_) => None,
	}
}

fn handle(req: &mut Request) -> IronResult<Response> {
	println!("{:?}", req.url);
	{
		let conn = req.extensions.get::<DbCon>();
		match conn {
			Some(ref conn) => {
				for row in conn.query("select * from poll", &[])
				.unwrap().iter() {
					let x: i64 = row.get(0);
					println!("{}", x);
				}
			}
			None => {
				println!("Could not open connection!");
			}
		}
	}

	{
		match get(req) {
			Some(map) => println!("{:?}", map),
			None => println!("Could not parse"),
		}
	}

	let cookie = req.get_cookie("hey");
	println!("HEY!");
	let mut resp = Response::with((
		status::Ok,
		"Hello!"
	));
	let mut nextval = 1i32;
	if let Some(value) = cookie {
		let val = &value.value;
		if let Ok(val) = val.parse::<i32>() {
			nextval += val;
		}
	}
	println!("{}", nextval);
	resp.set_cookie(cookie::Cookie::new(
		"hey".into(), nextval.to_string()));
	Ok(resp)
}

fn main() {
	match env_logger::init() {
		Ok(()) => {}
		Err(err) => {
			println!("Error: Logger was already started");
			return;
		}
	}

	info!("Setting up the middleware chain...");
	let router = Router::new();
	let mut chain = iron::middleware::Chain::new(handle);
	chain.link_before(response::ResponseTime);
	chain.link(oven::new(vec![]));
	chain.link_before(DbCon);
	chain.link_after(response::ResponseTime);

	let mut mount = Mount::new();
	mount.mount("/file/", Static::new(Path::new("src/")));
	mount.mount("/", chain);

	info!("Running the server...");
	match Iron::new(mount).http("localhost:3000") {
		Ok(server) => {
			info!("Server started: {:?}", server);
		}
		Err(err) => {
			error!("Could not start the server, {:?}", err);
		}
	}
}
