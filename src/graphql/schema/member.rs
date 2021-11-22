use crate::db::models;
use crate::db::models::Member;
use crate::graphql::Context;
use juniper::FieldResult;

#[derive(GraphQLObject)]
#[graphql(description = "A member of a team")]
pub struct MemberObject {
    pub id: i32,
    pub name: String,
    pub knockouts: i32,
    pub team_id: i32,
}

impl From<Member> for MemberObject {
    fn from(member: Member) -> Self {
        MemberObject {
            id: member.id,
            name: member.name.clone(),
            knockouts: member.knockouts,
            team_id: member.team_id,
        }
    }
}

impl Context {
    pub fn get_member(&self, id: i32) -> FieldResult<MemberObject> {
        let connection = &self.db_pool.get()?;
        let member = models::Member::get_member(connection, id)?;
        Ok(member.into())
    }

    pub fn get_all_members(&self) -> FieldResult<Vec<MemberObject>> {
        let connection = &self.db_pool.get()?;
        let members = models::Member::get_all_members(connection)?;
        let member_objects = members
            .iter()
            .map(|member| MemberObject::from(member.clone()))
            .collect();
        Ok(member_objects)
    }
}
