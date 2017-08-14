pub struct User {
	pub id: i32,
	pub name: String
}

pub struct Question {
	pub id: i32,
	pub user_id: i32,
	pub user: Option<User>,
	pub body: String
}