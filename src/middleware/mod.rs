use iron::mime::*;
use iron::modifier::Modifier;
use iron::{AfterMiddleware, BeforeMiddleware, typemap};
use iron::status;
use time::precise_time_ns;
use postgres::{Connection, SslMode};
use iron::prelude::*;
use mount::Mount;
use controllers;
use oven;
use std::path::Path;
use staticfile::Static;

pub fn get_middleware() -> Mount {
	info!("Setting up the middleware chain...");
	let mut html = Chain::new(controllers::index::index);
	html.link_before(ResponseTime);
	html.link(oven::new(vec![]));
	html.link_before(DbCon);
	html.link_after(WrapUp);
	html.link_after(Html);
	html.link_after(ResponseTime);

	let mut error = Chain::new(controllers::index::not_found);
	error.link_after(Html);

	info!("Setting up the mounts");
	let mut mount = Mount::new();
	mount.mount("/", html);
	mount.mount("/file/", Static::new(Path::new("file/")));
	mount.mount("/error/", error);
	mount
}

pub struct DbCon;

impl typemap::Key for DbCon { type Value = Connection; }

impl BeforeMiddleware for DbCon {
	fn before(&self, req: &mut Request) -> IronResult<()> {
		match Connection::connect(
			"postgresql://kefin@localhost/maindb",
			SslMode::None) {
			Ok(con) => {
				req.extensions.insert::<DbCon>(con);
			}
			Err(err) => {
				error!("Could not connect to database: {:?}", err);
				return Err(IronError::new (
					err,
					((status::InternalServerError,
						"Unable to connect to database, check the logs"
					))
				));
			}
		}
		Ok(())
	}
}


pub struct Html;

/// Ensures that the response will have a mime type that is Html
impl AfterMiddleware for Html {
	fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
		trace!("Setting MIME type to html");
		(Mime(TopLevel::Text, SubLevel::Html, vec![])).modify(&mut res);
		Ok(res)
	}
}


pub struct Body;

impl typemap::Key for Body { type Value = String; }


pub struct WrapUp;

impl AfterMiddleware for WrapUp {
	fn after(&self, req: &mut Request, mut res: Response) -> IronResult<Response> {
		match req.extensions.get::<Body>() {
			Some(string) => {
				debug!("Got string!: {}", string);
				res.set_mut(string.clone());
			}
			None => error!("Nothing got in WrapUp :("),
		}
		Ok(res)
	}
}


/// Put around the chain to get and report the response time
struct ResponseTime;

impl typemap::Key for ResponseTime { type Value = u64; }

impl BeforeMiddleware for ResponseTime {
	fn before(&self, req: &mut Request) -> IronResult<()> {
		let time = precise_time_ns();
		req.extensions.insert::<ResponseTime>(time);
		Ok(())
	}
}

impl AfterMiddleware for ResponseTime {
	fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
		match req.extensions.get::<ResponseTime>() {
			Some(&time) => {
				let change = precise_time_ns() - time;
				debug!("dt: {} ms", (change as f64) / 1_000_000.0);
			}
			None => {
				error!("Linked in ResponseTime AfterMiddleware without the BeforeMiddleware");
			}
		}
		Ok(res)
	}
}

