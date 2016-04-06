use iron::prelude::*;
use iron::status;
use cookie;
use oven::RequestExt;
use oven::ResponseExt;
use super::views::render;
use middleware::DbCon;

pub fn index(req: &mut Request) -> IronResult<Response> {
	// This must be put inside a model!
	{
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

	// This too inside a model
	let cookie = req.get_cookie("hey");
	let mut resp = Response::with((
		status::Ok,
		render(),
	));
	let mut nextval = 1i32;
	if let Some(value) = cookie {
		let val = &value.value;
		if let Ok(val) = val.parse::<i32>() {
			nextval += val;
		}
	}
	resp.set_cookie(cookie::Cookie::new(
		"hey".into(), nextval.to_string()));
	Ok(resp)
}

