use diesel::{PgConnection, RunQueryDsl, Insertable};
use crate::model::schema::users;

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String, 
    pub email: String,
}

impl User {
    pub fn register(connection: &mut PgConnection, new_user: &Self) {
        diesel::insert_into(users::table)
            .values(new_user)
            .execute(connection)
            .expect("Error inserting user into database");

        println!("Register {:?}", new_user);
    }
}
