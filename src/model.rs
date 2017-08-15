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

pub struct Question<'a> {
	pub conn: &'a DbConn,
	pub id: Option<i32>,
	pub username: String,
	pub body: String
}

impl<'a> ActiveRecord<'a> for Question<'a> {
	fn table_name() -> &'static str { "questions" }
	fn from_row(conn: &'a DbConn, row: Row) -> Self {
		let id: i32 = row.get("id");
		Self {
			conn: conn,
			id: Some(id),
			username: row.get("username"),
			body: row.get("body")
		}
	}
	fn conn(&self) -> &'a DbConn { self.conn }
	fn primary_key(&self) -> Option<i32> { self.id }
	fn set_primary_key(&mut self) -> &mut Option<i32> { &mut self.id }
	fn columns(&self) -> HashMap<&str, &ToSql> {
		let mut map: HashMap<&str, &ToSql> = HashMap::new();
		map.insert("username", &self.username);
		map.insert("body"    , &self.body);
		map
	}
}

pub trait ActiveRecord<'a> : Sized {
	fn table_name() -> &'static str;
	fn from_row(conn: &'a DbConn, row: Row) -> Self;
	fn conn(&self) -> &'a DbConn;
	fn primary_key(&self) -> Option<i32>;
	fn set_primary_key(&mut self) -> &mut Option<i32>;
	fn columns(&self) -> HashMap<&str, &ToSql>; // all columns except primary key.

	fn find(conn: &'a DbConn, id: i32) -> Self {
		let sql = format!("SELECT * FROM {} WHERE id = ? LIMIT 1", Self::table_name());
		let rows = conn.query(&sql, &[&id]).unwrap();
		Self::from_row(conn, rows.get(0))
	}
	fn lit(conn: &'a DbConn, condition: &str) -> Vec<Self> {
		let sql = format!("SELECT * FROM {} WHERE {}", Self::table_name(), condition);
		let mut results = Vec::new();
		for row in conn.query(&sql, &[]).unwrap().into_iter() {
			results.push(Self::from_row(conn, row));
		}
		results
	}
	fn update(&self) {
		let set_stmt = self.columns().into_iter()
                             .map(|(k, _)| format!("{} = ?", k))
                             .collect::<Vec<String>>()
                             .join(", ");
		let stmt = format!(
			"UPDATE {} SET {} WHERE id = {}",
			Self::table_name(), set_stmt, self.primary_key().unwrap().to_string()
		);
		let values: Vec<_> = self.columns().into_iter().map(|(_, v)| v).collect();
		self.conn().execute(&stmt, values.as_slice()).unwrap();
	}
	fn insert(&mut self) {
		let id: i32;
		{
			let immutable_self = &self;
			let (column_names, column_values): (Vec<_>, Vec<_>) = immutable_self.columns().into_iter().unzip();
			let names_string = column_names.join(", ");
			let placeholders_string = column_values.iter().enumerate().map(|(i, _)| format!("${}", i+1))
			                                       .collect::<Vec<String>>().join(", ");
			let stmt = format!(
				"INSERT INTO {} ({}) VALUES ({}) RETURNING id",
				Self::table_name(), names_string, placeholders_string
			);
			let s = self.conn().prepare(&stmt).unwrap();
			let rows = s.query(column_values.as_slice()).unwrap();
			id = rows.get(0).get(0);
		}
		*self.set_primary_key() = Some(id);
	}
	fn delete(&self) {
		let stmt = format!("DELETE FROM {} WHERE id = {}", Self::table_name(), self.primary_key().unwrap().to_string());
		self.conn().execute(&stmt, &[]).unwrap();
	}
}