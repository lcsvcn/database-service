use diesel::{PgConnection, RunQueryDsl, Insertable};
use crate::model::schema::users;

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct User {
    pub(crate) id: i32,
    pub(crate) username: String, // Change to String
    pub(crate) email: String,    // Change to String
    // Add other fields as needed
}

impl<'a> User {
    pub(crate) fn register(connection: &mut PgConnection, new_user: User) {
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(connection)
            .expect("Error inserting user into database");

        println!("Registered user: {:?}", new_user);
    }
}
