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

mod model;
use model::*;

type Pool = r2d2::Pool<PostgresConnectionManager>;

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<model::DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(model::DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

#[get("/<username>/<body>")]
fn create(conn: DbConn, username: &RawStr, body: &RawStr) -> String {
    let mut q = Question {
        conn: &conn,
        id: None,
        username: username.to_string(),
        body: body.to_string()
    };
    println!("{:?}", q.id);
    q.insert();
    println!("{:?}", q.id);
	String::from("create!")
}

#[get("/")]
fn index(conn: DbConn) -> String {
    let qs = Question::all(&conn);
    qs.into_iter().map(|q| q.username).collect::<Vec<String>>().join(", ")
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