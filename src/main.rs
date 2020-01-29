#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod graphql;
mod context;
mod query;
mod mutation;

pub(crate) mod schema;

pub(crate) type Schema = juniper::RootNode<'static, query::Query, mutation::Mutation>;

#[derive(Queryable)]
pub(crate) struct Photo {
    pub id: i32,
    pub name: String,
    pub file: String,
    pub marker: String,
}

fn main() {
    rocket::ignite()
        .manage(context::create_context())
        .manage(Schema::new(query::Query, mutation::Mutation))
        .mount(
            "/",
            rocket::routes![
                graphql::infrastructure::graphiql,
                graphql::infrastructure::get_graphql_handler,
                graphql::infrastructure::post_request_handler,
                graphql::infrastructure::opt_graphql_handler,
            ],
        )
        .launch();
}
