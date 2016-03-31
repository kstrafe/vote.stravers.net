create table Poll (
	id bigint primary key,
	description varchar,
	creation date default current_date not null
);
create sequence candidate_id_seq;
create table Candidate (
	id bigint primary key default nextval('candidate_id_seq'),
	poll_id bigint references Poll(id),
	name varchar,
	creation date default current_date
);
create table Vote (
	candidate_id bigint references Candidate(id),
	poll_id bigint references Poll(id),
	voter_id bigint,
	creation date default current_date,
	unique(voter_id, poll_id, candidate_id)
);
