use dbcon::DbCon;
use iron::prelude::*;
use iron::status;
use cookie;
use oven::RequestExt;
use oven::ResponseExt;
use super::*;

pub fn index(req: &mut Request) -> IronResult<Response> {
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

