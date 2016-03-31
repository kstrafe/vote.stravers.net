use iron::mime::*;
use iron::prelude::*;
use iron::status;
use std::io::Read;
use super::{html_response};
use urlencoded::UrlEncodedBody;

pub fn create_poll(req: &mut Request) -> IronResult<Response> {
	match req.get_ref::<UrlEncodedBody>() {
		Ok(ref hashmap) => println!("Parsed GET request query string:\n {:?}", hashmap),
		Err(ref e) => println!("{:?}", e)
	};
	println!("{:?}", req);
	println!("{:?}", req.headers);
	// println!("{:?}", req.extensions);
	let mut string = String::new();
	req.body.read_to_string(&mut string).expect("Could not read to string");
	println!("{:?}", string);
	Ok(html_response("".into()))
}
