extern crate questions;
use questions::model::*;
use questions::model::config::get_pool;

fn get_dbconn() -> DbConn {
	let pooled_connection = get_pool().get().unwrap();
	DbConn(pooled_connection)
}

#[test]
fn insert() {
	let conn = get_dbconn();
    let mut q = Question {
        conn: &conn,
        id: None,
        username: "username".to_string(),
        body: "body".to_string()
    };
    assert!(!q.id.is_some());
    let previous_row_count = Question::all(&conn).len();
    q.insert();
    let current_row_count = Question::all(&conn).len();
    assert_eq!(current_row_count - previous_row_count, 1);
    assert!(q.id.is_some());
}