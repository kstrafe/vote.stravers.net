create sequence poll_id_seq;
create table Poll (
	id bigint primary key default nextval('poll_id_seq')
);
create sequence candidate_id_seq;
create table Candidate (
	id bigint primary key default nextval('candidate_id_seq'),
	poll_id bigint references Poll(id),
	name varchar
);
create table Vote (
	candidate_id bigint references Candidate(id),
	poll_id bigint references Poll(id),
	voter_id bigint,
	unique(voter_id, poll_id, candidate_id)
);
