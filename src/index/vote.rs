use iron::mime::*;
use iron::prelude::*;
use iron::status;
use super::html_response;

fn handler(_: &mut Request) -> IronResult<Response> {
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
