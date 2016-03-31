use iron::mime::*;
use iron::prelude::*;
use iron::status;
use router::Router;

pub fn image(req: &mut Request) -> IronResult<Response> {
	println!("Attempt to fetch image\n{:?}", req);
	let filename = &req.extensions.get::<Router>()
		.unwrap()
		.find("filename")
		.unwrap_or("/");
	Ok(file_response(filename))
}

pub fn handler(_req: &mut Request) -> IronResult<Response> {
	/*let ref query = req.extensions.get::<Router>()
		.unwrap()
		.find("query")
		.unwrap_or("/");
	*/
	let name = "Lyra";
	let mut buffer = String::new();
	html!(buffer, {
		html {
			head {
				style { r#"@import url()"# }
				link rel="icon" type="image/png" href="/storage/favicon48x48.png" /
			}
			body {
				h1 "Pinkie's Brew"
				p { "Hi! " ^name "!" }
			}
		}
	}).unwrap();
	Ok(html_response(buffer))
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

fn html_response(content: String) -> Response {
	Response::with((
		Mime(TopLevel::Text, SubLevel::Html, vec![]),
		status::Ok,
		content
	))
}
