#![feature(plugin)]
#![plugin(maud_macros)]
extern crate iron;
extern crate maud;
extern crate router;

use iron::headers::{ContentType, Headers};
use iron::prelude::*;
use iron::status;
use router::Router;

fn setup_router() -> Router {
	let mut router = Router::new();
	router.get("/", handler);
	router.get("/:query", handler);
	router
}

fn main() {

	let router = setup_router();

	println!("Server started on :3000");

	Iron::new(router)
		.http("localhost:3000")
		.unwrap();
}


fn handler(req: &mut Request) -> IronResult<Response> {
	let ref query = req.extensions.get::<Router>()
		.unwrap()
		.find("query")
		.unwrap_or("/");
	let name = "Lyra";
	let mut buffer = String::new();
	html!(buffer, {
		html {
			head {
			}
			body {
				h1 "Pinkie's Brew"
				p { "Hi! " ^name "!" }
			}
		}
	}).unwrap();
	Ok(html_response(buffer))
}

fn html_response(content: String) -> Response {
	let mut headers = Headers::new();
	headers.set(ContentType::html());
	let mut response = Response::with((
		status::Ok, content));
	response.headers = headers;
	response
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
