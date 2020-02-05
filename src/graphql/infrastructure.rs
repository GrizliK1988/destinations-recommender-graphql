use rocket::response::content;
use rocket::{State, Response};
use juniper_rocket::{GraphQLResponse, GraphQLRequest};

#[rocket::get("/")]
pub(crate) fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[rocket::options("/graphql")]
pub(crate) fn opt_graphql_handler() -> Response<'static> {
    Response::new()
}

#[rocket::get("/graphql?<request>")]
pub(crate) fn get_graphql_handler(
    context: State<crate::context::Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<crate::Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[rocket::post("/graphql", data = "<request>")]
pub(crate) fn post_request_handler(
    context: State<crate::context::Context>,
    request: GraphQLRequest,
    schema: State<crate::Schema>,
) -> GraphQLResponse {
    request.execute(&schema, &context)
}
