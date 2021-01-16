use diesel::{self, prelude::*};
use crate::schema::users;

#[table_name="users"]
#[derive(AsChangeset, Serialize, Queryable, Insertable, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String
}

impl User {
    pub fn get(id: String, connection: &MysqlConnection) -> User {
        users::table.find(id).first(connection).unwrap()
    }

    pub fn all(connection: &MysqlConnection) -> Vec<User> {
        users::table.order(users::id).load::<User>(connection).unwrap()
    }

    pub fn create(user: User, connection: &MysqlConnection) -> User {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(connection)
            .expect("Error creating new user");

        users::table.find(user.id).first(connection).unwrap()
    }

    pub fn update(id: String, user: User, connection: &MysqlConnection) -> bool {
        diesel::update(users::table.find(id)).set(&user).execute(connection).is_ok()
    }

    pub fn delete(id: String, connection: &MysqlConnection) -> bool {
        diesel::delete(users::table.find(id)).execute(connection).is_ok()
    }
}
