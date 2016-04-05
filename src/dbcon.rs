use iron::{BeforeMiddleware, typemap};
use iron::prelude::*;
use iron::status;
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
				return Err(IronError::new (
					err,
					((status::InternalServerError, "Unable to connect to database, check the logs"))
				));
			}
		}
		Ok(())
	}
}

