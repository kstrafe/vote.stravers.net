use iron::mime::*;
use iron::prelude::*;
use iron::status;
use router::Router;

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

fn html_response(content: String) -> Response {
	Response::with((
		Mime(TopLevel::Text, SubLevel::Html, vec![]),
		status::Ok,
		content
	))
}
