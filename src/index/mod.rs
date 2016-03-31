pub mod createpoll;
pub mod fileloader;
pub mod frontpage;
pub mod vote;

use iron::mime::*;
use iron::prelude::*;
use iron::status;

fn html_response(content: String) -> Response {
	Response::with((
		Mime(TopLevel::Text, SubLevel::Html, vec![]),
		status::Ok,
		content
	))
}

fn generate_header() -> String {
	let mut buffer = String::new();
	html!(buffer, {
		head {
			style { r#"@import url()"# }
			link rel="icon" type="image/png" href="/storage/favicon48x48.png" /
		}
	});
	buffer
}
