use iron::mime::*;
use iron::modifier::Modifier;
use oven::{RequestExt, ResponseExt};
use iron::{AfterMiddleware, BeforeMiddleware, typemap};
use iron::status;
use time::precise_time_ns;
use postgres::{Connection, SslMode};
use cookie;
use iron::prelude::*;
use mount::Mount;
use controllers;
use oven;
use std::path::Path;
use staticfile::Static;

pub fn get_middleware() -> Mount {
	info!("Setting up the middleware chain...");
	let mut chain = Chain::new(controllers::index::index);
	chain.link_before(ResponseTime);
	chain.link(oven::new(vec![]));
	chain.link_before(DbCon);
	chain.link_after(Html);
	chain.link_after(ResponseTime);

	info!("Setting up the mounts");
	let mut mount = Mount::new();
	mount.mount("/", chain);
	mount.mount("/file/", Static::new(Path::new("src/")));
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
	fn after(&self, req: &mut Request, mut res: Response) -> IronResult<Response> {
		trace!("Setting MIME type to html");
		(Mime(TopLevel::Text, SubLevel::Html, vec![])).modify(&mut res);
		Ok(res)
	}
}


pub struct ResponseTime;

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

