pub(crate) struct Mutation;

#[juniper::object(Context = super::context::Context)]
impl Mutation {
    pub fn apiVersion () -> &str {
        "1.0"
    }
}
