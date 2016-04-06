use iron::mime::*;
use iron::modifier::Modifier;
use oven::{RequestExt, ResponseExt};
use iron::{AfterMiddleware, BeforeMiddleware, typemap};
use iron::prelude::*;
use iron::status;
use time::precise_time_ns;
use postgres::{Connection, SslMode};
use cookie;

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
					((status::InternalServerError, "Unable to connect to database, check the logs"))
				));
			}
		}
		Ok(())
	}
}


pub struct Html;

impl AfterMiddleware for Html {
	fn after(&self, req: &mut Request, mut res: Response) -> IronResult<Response> {
		trace!("Setting MIME type to html");
		(Mime(TopLevel::Text, SubLevel::Html, vec![])).modify(&mut res);
		Ok(res)
	}
}


struct User;

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
				println!("dt: {} ms", (change as f64) / 1_000_000.0);
			}
			None => {
				println!("Warning: linked in ResponseTime AfterMiddleware without the BeforeMiddleware");
			}
		}
		Ok(res)
	}
}

