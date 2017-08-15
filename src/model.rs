use std::ops::Deref;
use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::postgres::Connection;
use r2d2_postgres::postgres::rows::Row;
use r2d2_postgres::postgres::types::{ToSql, FromSql};
use r2d2;

use std::collections::HashMap;
use std::marker::Sized;

pub struct DbConn(pub r2d2::PooledConnection<PostgresConnectionManager>);

impl Deref for DbConn {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

trait ActiveRecord<'a> : Sized {
	fn table_name() -> &'static str;
	fn from_row(conn: &DbConn, row: Row) -> Self;
	fn conn(&self) -> &DbConn;

	fn columns(&self) -> HashMap<&str, &str>;
	fn find(conn: &DbConn, id: i64) -> Self {
		let sql = format!("SELECT * FROM {} WHERE id = ? LIMIT 1", Self::table_name());
		let rows = conn.query(&sql, &[&id]).unwrap();
		Self::from_row(conn, rows.get(0))
	}
	fn lit(conn: &DbConn, condition: &str) -> Vec<Self> {
		let sql = format!("SELECT * FROM {} WHERE {}", Self::table_name(), condition);
		let mut results = Vec::new();
		for row in conn.query(&sql, &[]).unwrap().into_iter() {
			results.push(Self::from_row(conn, row));
		}
		results
	}
	fn update(&self) {
		let set_stmt = self.columns().into_iter()
                             .map(|(k, v)| format!("{} = {}", k, v))
                             .collect::<Vec<String>>()
                             .join(", ");
		let stmt = format!(
			"UPDATE {} SET {} WHERE id = {}",
			Self::table_name(), set_stmt, self.primary_key().unwrap().to_string()
		);
		self.conn().execute(&stmt, &[]).unwrap();
	}
	fn delete(&self) {
		let stmt = format!("DELETE FROM {} WHERE id = {}", Self::table_name(), self.primary_key().unwrap().to_string());
		self.conn().execute(&stmt, &[]).unwrap();
	}
	fn primary_key(&self) -> Option<i64>;
}