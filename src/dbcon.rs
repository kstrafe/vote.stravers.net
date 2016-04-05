use iron::{BeforeMiddleware, typemap};
use iron::prelude::*;
use postgres::{Connection, SslMode};

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
			}
		}
		Ok(())
	}
}

