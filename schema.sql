CREATE TABLE question (
	id serial primary key,
	user_id int not null,
	body text not null
);