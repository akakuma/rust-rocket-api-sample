use std::env;
use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{Pool, PooledConnection, ConnectionManager};

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn connect() -> MysqlPool {
    let database = env::var("DATABASE_URL").expect("DATABASE_URL is not defined");
    let manager = ConnectionManager::<MysqlConnection>::new(database);
    Pool::new(manager).expect("Failed to create pool")
}

pub struct Connection(pub PooledConnection<ConnectionManager<MysqlConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for Connection {
  type Error = ();

  fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
      let pool = request.guard::<State<MysqlPool>>()?;
      match pool.get() {
          Ok(conn) => Outcome::Success(Connection(conn)),
          Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
      }
  }
}

impl Deref for Connection {
  type Target = MysqlConnection;

  fn deref(&self) -> &Self::Target {
      &self.0
  }
}
