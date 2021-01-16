use uuid::Uuid;
use rocket_contrib::json::{Json, JsonValue};
use crate::db;
use crate::models::user::User;

#[derive(Deserialize, Debug)]
pub struct PostUser {
    pub name: String,
    pub email: String,
}

#[get("/users")]
pub fn list(connection: db::Connection) -> Json<JsonValue>  {
    Json(json!(User::all(&connection)))
}

#[post("/users", data = "<user>")]
pub fn create(user: Json<PostUser>, connection: db::Connection) -> Json<User> {
    let PostUser { name, email } = user.into_inner();

    let uuid = Uuid::new_v4().to_hyphenated().to_string();
    let insert = User {
      id: uuid,
      name,
      email,
    };
    Json(User::create(insert, &connection))
}

#[get("/users/<id>")]
pub fn find(id: String, connection: db::Connection) -> Json<User> {
    Json(User::get(id, &connection))
}

#[put("/users/<id>", data = "<user>")]
pub fn update(id: String, user: Json<PostUser>, connection: db::Connection) -> Json<JsonValue> {
    let PostUser { name, email } = user.into_inner();
    let update = User {
        id: id.clone(),
        name,
        email,
    };
    Json(json!({
        "success": User::update(id, update, &connection)
    }))
}
