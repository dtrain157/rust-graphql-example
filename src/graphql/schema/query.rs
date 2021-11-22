use juniper::FieldResult;

use crate::graphql::schema::MemberObject;
use crate::graphql::Context;

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    fn api_version() -> &'static str {
        "0.1"
    }

    fn member(context: &Context, id: i32) -> FieldResult<MemberObject> {
        let result = context.get_member(id)?;
        Ok(result)
    }

    fn members(context: &Context) -> FieldResult<Vec<MemberObject>> {
        let result = context.get_all_members()?;
        Ok(result)
    }
}
