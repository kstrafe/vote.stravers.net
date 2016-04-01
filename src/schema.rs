infer_table_from_schema!(dotenv!("DATABASE_URL"), "poll");
infer_table_from_schema!(dotenv!("DATABASE_URL"), "candidate");
infer_table_from_schema!(dotenv!("DATABASE_URL"), "posts");
table!(
	vote(voter_id) {
		candidate_id -> BigInt,
		poll_id -> BigInt,
		voter_id -> BigInt,
		creation -> Date,
	}
);
