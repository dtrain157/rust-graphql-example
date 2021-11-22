use diesel;
use diesel::prelude::*;
use diesel::PgConnection;

pub use crate::db::schema::members;

#[derive(AsChangeset, Queryable, Identifiable, Debug, Clone, PartialEq)]
#[table_name = "members"]
pub struct Member {
    pub id: i32,
    pub name: String,
    pub knockouts: i32,
    pub team_id: i32,
}

impl Member {
    pub fn get_member(connection: &PgConnection, id: i32) -> QueryResult<Member> {
        members::table.find(id).first::<Member>(connection)
    }

    pub fn get_all_members(connection: &PgConnection) -> QueryResult<Vec<Member>> {
        members::table.order(members::id).load::<Member>(connection)
    }
}
