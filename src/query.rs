use juniper::{FieldResult};
use crate::graphql::resolver::photos::{Photo, DestinationScore, UserPreference};

pub(crate) struct Query;

#[juniper::object(Context = super::context::Context)]
impl Query {
    pub fn apiVersion () -> &str {
        "1.0"
    }

    pub fn photos(&self, context: &super::context::Context, count_per_category: i32) -> FieldResult<Vec<Photo>> {
        self.photos_implementation(context, count_per_category)
    }

    pub fn recommendation(&self, context: &super::context::Context, user_preferences: Vec<UserPreference>) -> FieldResult<Vec<DestinationScore>> {
        self.recommendation_implementation(context, user_preferences)
    }
}
