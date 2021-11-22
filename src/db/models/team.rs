use diesel::QueryableByName;

pub use crate::db::schema::teams;

#[derive(QueryableByName)]
#[table_name = "teams"]
pub struct Team {
    pub id: i32,
    pub name: String,
}
