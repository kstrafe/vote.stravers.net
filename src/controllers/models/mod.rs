use iron::prelude::*;
use middleware::DbCon;

pub fn create_poll(req: &mut Request) {
	let conn = req.extensions.get::<DbCon>();
	match conn {
		Some(ref conn) => {
			match conn.query("select * from poll", &[]) {
				Ok(ref rows) => {
					for row in rows.iter() {
						let x: i64 = row.get(0);
						trace!("{}", x);
					}
				}
				Err(err) => {
					error!("Db error: {:?}", err);
				}
			}
		}
		None => {
			error!("Could not open connection!");
		}
	}
}
