use std::ops::Deref;
use rocket::http::Status;
use rocket::{Request, State, Outcome};
use rocket::request::{self, FromRequest};
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::Connection;

pub mod schematic {
    table! {
        invites {
            id   -> Integer,
        }
    }

    table! {
        clients {
            id   -> Integer,
            identity -> Text,
        }
    }
}

type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;
static DATABASE_URL: &'static str = env!("ILLNESS_DATABASE_URL");

pub fn init_pool() -> SqlitePool {
    let manager = ConnectionManager::<SqliteConnection>::new(DATABASE_URL);
    Pool::new(manager).expect("Failed to initialize database pool.")
}

pub struct DBConnection(pub PooledConnection<ConnectionManager<SqliteConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DBConnection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<SqlitePool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DBConnection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for DBConnection {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


pub fn create_tables(conn: DBConnection) {

}