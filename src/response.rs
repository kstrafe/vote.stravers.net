use iron::{AfterMiddleware, BeforeMiddleware, typemap};
use iron::prelude::*;
use time::precise_time_ns;

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

