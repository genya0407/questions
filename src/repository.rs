use std::ops::Deref;
use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::postgres::Connection;
use r2d2;

pub struct DbConn(pub r2d2::PooledConnection<PostgresConnectionManager>);

impl Deref for DbConn {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub mod question {
	use repository::DbConn;
	use entity::{Question, User};

	struct QuestionRepository {
		
	}

	pub fn create(conn: DbConn, user_id: &i32, body: &String) {
		conn.execute(
			"INSERT INTO Question (user_id, body) VALUES ($1, $2)",
			&[user_id, body]
		).unwrap();
	}

	pub fn all(conn: DbConn) -> Vec<Question> {
		let mut questions = Vec::new();

		for row in &conn.query("SELECT id, user_id, body FROM Question", &[]).unwrap() {
			let question = Question {
				id: row.get("id"),
				user_id: row.get("user_id"),
				user: None,
				body: row.get("body"),
			};

			questions.push(question);
		}

		return questions;
	}
}