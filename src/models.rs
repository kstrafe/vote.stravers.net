use super::schema::*;
use chrono::naive::date::NaiveDate;

#[derive(Queryable)]
pub struct Post {
	pub id: i32,
	pub title: String,
	pub body: String,
	pub published: bool,
}

#[derive(Queryable)]
pub struct Poll {
	pub id: i64,
	pub description: Option<String>,
	pub creation: NaiveDate,
}

#[derive(Queryable)]
pub struct Candidate {
	pub id: i64,
	pub poll_id: i64,
	pub name: String,
	pub creation: NaiveDate,
}

#[insertable_into(candidate)]
pub struct NewCandidate<'a> {
	pub poll_id: i64,
	pub name: &'a str,
}

#[derive(Queryable)]
pub struct Vote {
	pub candidate_id: i64,
	pub poll_id: i64,
	pub voter_id: i64,
	pub creation: NaiveDate,
}

#[insertable_into(vote)]
pub struct NewVote {
	pub candidate_id: i64,
	pub poll_id: i64,
	pub voter_id: i64,
}

#[insertable_into(posts)]
pub struct NewPost<'a> {
	pub title: &'a str,
	pub body: &'a str,
}

#[insertable_into(poll)]
pub struct NewPoll<'a> {
	pub id: i64,
	pub description: &'a str,
}
