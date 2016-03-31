use iron::mime::*;
use iron::prelude::*;
use iron::status;
use maud::PreEscaped;
use super::{html_response, generate_header};

pub fn frontpage(_: &mut Request) -> IronResult<Response> {
	let name = "Lyra";
	let mut buffer = String::new();
	let header = generate_header();
	html!(buffer, {
		html {
			^PreEscaped(header)
			body {
				h1 "Pinkie's Brew"
				form action="/createpoll" method="post" {
					input name="description" type="text" placeholder="description" /
					input name="options" type="text" placeholder="comma-separated options" /
					input type="submit" /
				}
				p { "Hi! " ^name "!" }
			}
		}
	}).unwrap();
	Ok(html_response(buffer))
}
