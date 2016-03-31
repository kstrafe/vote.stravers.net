#![feature(plugin)]
#![plugin(maud_macros)]
extern crate iron;
extern crate maud;
extern crate router;

use iron::mime::*;
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

fn handler(_req: &mut Request) -> IronResult<Response> {
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

fn html_response(content: String) -> Response {
	Response::with((
		Mime(TopLevel::Text, SubLevel::Html, vec![]),
		status::Ok,
		content
	))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
