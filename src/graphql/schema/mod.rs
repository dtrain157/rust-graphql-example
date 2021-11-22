mod member;
mod query;

use crate::Context;
use juniper::{EmptyMutation, EmptySubscription, RootNode};
pub use member::*;
use query::*;

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}
