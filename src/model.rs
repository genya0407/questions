use std::ops::Deref;
use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::postgres::Connection;
use r2d2_postgres::postgres::types::ToSql;
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

trait ActiveRecord : Sized {
	fn new(params: HashMap<&str, &ToSql>) -> Self;
	fn create(params: HashMap<&str, &ToSql>) -> Self;
	fn find(id: i64) -> Self;
	fn hoge(condition: String) -> Vec<Self>;
	fn save(&self) -> bool;
	fn delete(&self) -> bool;
}