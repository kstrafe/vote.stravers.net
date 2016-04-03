use iron::{BeforeMiddleware, typemap};
use iron::prelude::*;
use postgres::{Connection, SslMode};

pub struct DbCon;

impl typemap::Key for DbCon { type Value = Connection; }

impl BeforeMiddleware for DbCon {
	fn before(&self, req: &mut Request) -> IronResult<()> {
		let conn = Connection::connect(
			"postgresql://kefin@localhost/diesel_demo",
			SslMode::None)
			.unwrap();
		req.extensions.insert::<DbCon>(conn);
		Ok(())
	}
}

