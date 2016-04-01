use diesel::pg::PgConnection;
use iron::mime::*;
use iron::prelude::*;
use iron::status;
use router::Router;
use super::*;
use ::models::*;
use diesel::prelude::*;

fn get_choices(pid: i64) -> Vec<Candidate> {
	use ::schema::candidate::dsl::*;
	let con = &establish_connection();
	candidate
		.filter(poll_id.eq(pid))
		.load::<Candidate>(con)
		.expect("Could not load candidates!")
}

fn get_poll_description(pid: i64) -> String {
	use ::schema::poll::dsl::*;
	let con = &establish_connection();
	let elem = &poll
		.filter(id.eq(pid))
		.load::<Poll>(con)
		.expect("Could not load candidates!")
		[0];
	match elem.description {
		Some(ref string) => return string.clone(),
		None => return String::from("No Description"),
	}
}

pub fn vote_handler(req: &mut Request) -> IronResult<Response> {
	let pollid = req.extensions.get::<Router>()
		.unwrap()
		.find("value")
		.unwrap_or("0");
	let pollid = radix_36_to_radix_10(&pollid);
	println!("vote handler pol id: {}", pollid);
	let choices = get_choices(pollid);
	let size = choices.len();

	let mut buffer = String::new();
	html!(buffer, {
		html {
			head {
				style { r#"@import url()"# }
				link rel="icon" type="image/png" href="/storage/favicon48x48.png" /
			}
			body {
				h1 ^get_poll_description(pollid)
				h1 "here are your choices!"
				^size
				form action="/votefor" method="post" {
					@ for i in choices {
						input value=^i.name type="submit" /
					}
				}
				p { "Amethyst vs Pearl" }
			}
		}
	}).unwrap();
	Ok(html_response(buffer))
}
