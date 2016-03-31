use iron::prelude::*;
use iron::status;
use router::Router;

pub fn file_handler(req: &mut Request) -> IronResult<Response> {
	println!("Attempt to fetch image\n{:?}", req);
	let filename = &req.extensions.get::<Router>()
		.unwrap()
		.find("filename")
		.unwrap_or("/");
	Ok(file_response(filename))
}

fn file_response(file: &str) -> Response {
	use std::path::Path;
	let mut prefix = String::from("storage/");
	prefix.push_str(file);
	let path = Path::new(&prefix);
	if path.exists() {
		Response::with((
			// Automatically inferred from Path :D
			// Mime(TopLevel::Image, SubLevel::Png, vec![]),
			status::Ok,
			Path::new(&prefix)
		))
	} else {
		Response::with((
			status::NoContent
		))
	}
}
