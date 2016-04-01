create table Poll (
	id bigint primary key,
	description varchar,
	creation date default current_date not null
);
create sequence candidate_id_seq;
create table Candidate (
	id bigint primary key default nextval('candidate_id_seq'),
	poll_id bigint not null references Poll(id),
	name varchar not null,
	creation date default current_date not null
);
create table Vote (
	candidate_id bigint not null references Candidate(id),
	poll_id bigint not null references Poll(id),
	voter_id bigint not null,
	creation date not null default current_date,
	unique(voter_id, poll_id, candidate_id)
);
