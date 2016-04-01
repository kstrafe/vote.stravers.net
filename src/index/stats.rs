use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use iron::mime::*;
use iron::prelude::*;
use iron::status;
use router::Router;
use super::*;
use ::models::*;
use urlencoded::UrlEncodedBody;
use super::createpoll::back_to_start;

fn get_candidate_by_name(poll_id_val: i64, candidate_name: &str) -> i64 {
	use ::schema::candidate::dsl::*;
	let con = &establish_connection();
	candidate
		.filter(name.eq(candidate_name).and(poll_id.eq(poll_id_val)))
		.load::<Candidate>(con)
		.expect("Could not load candidates!")
		[0]
		.id
}

fn register_answer(poll_id_val: i64, candidate_name: &str) {
	use ::schema::candidate::dsl::*;
	let con = &establish_connection();
	let cand_id = get_candidate_by_name(poll_id_val, candidate_name);
	let results: Result<Vote, _> = diesel::insert(
		&NewVote {
			candidate_id: cand_id,
			poll_id: poll_id_val,
			voter_id: 0,
		}
	).into(::schema::vote::table).get_result(con);
}

pub fn vote_answer(req: &mut Request) -> IronResult<Response> {
	let parsed = req.get_ref::<UrlEncodedBody>();
	let mut map;
	match parsed {
		Ok(ref hashmap) => map = hashmap,
		Err(_) => return back_to_start(),
	}
	let radix = &map.get("identifier").unwrap()[0];
	let name = &map.get("candname").unwrap()[0];
	let pollid = radix_36_to_radix_10(radix);
	println!("Updating: {}", pollid);
	register_answer(pollid, name);

	Ok(html_response("empty".into()))
}
