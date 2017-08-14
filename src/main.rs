#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use rocket::http::RawStr;

extern crate rocket_contrib;
use rocket_contrib::Template;

extern crate r2d2_postgres;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};

extern crate r2d2;

use std::thread;

mod repository;
use repository::DbConn;
mod entity;
use entity::{Question, User};

type Pool = r2d2::Pool<PostgresConnectionManager>;

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<repository::DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(repository::DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

#[get("/<body>")]
fn create(conn: DbConn, body: &RawStr) -> String {
	repository::question::create(conn, &10, &body.to_string());

	"Create!".to_string()
}

#[get("/")]
fn index(conn: DbConn) -> String {
	let questions = repository::question::all(conn);

	questions.iter().map(|q| q.body.clone()).collect::<Vec<String>>().join("-")
}

fn main() {
    let config = r2d2::Config::default();
    let manager = PostgresConnectionManager::new(
    	"postgresql://question:question@localhost/question",
    	TlsMode::None
    ).unwrap();
    let pool: Pool = r2d2::Pool::new(config, manager).unwrap();

	rocket::ignite()
		.manage(pool)
		.attach(Template::fairing())
		.mount("/", routes![index, create]).launch();
}