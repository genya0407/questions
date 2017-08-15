#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use rocket::http::RawStr;

extern crate rocket_contrib;
use rocket_contrib::Template;

mod model;
use model::*;
use model::config::Pool;

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
    q.insert();
	String::from("create!")
}

#[get("/")]
fn index(conn: DbConn) -> String {
    let qs = Question::all(&conn);
    qs.into_iter().map(|q| q.username).collect::<Vec<String>>().join(", ")
}

fn main() {
    let pool = model::config::get_pool();
	rocket::ignite()
		.manage(pool)
		.attach(Template::fairing())
		.mount("/", routes![index, create]).launch();
}