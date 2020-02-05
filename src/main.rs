#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate diesel;
extern crate dotenv;

use rocket::fairing::AdHoc;
use rocket_http::hyper::header::{ AccessControlAllowOrigin, AccessControlAllowHeaders, AccessControlAllowMethods, AccessControlMaxAge };
use rocket_http::hyper::Method;
use unicase::UniCase;

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
        .attach(AdHoc::on_response("CORS", |_, res| {
            res.set_header(AccessControlAllowOrigin::Any);
            res.set_header(AccessControlAllowHeaders(vec![
                UniCase("content-type".to_owned())
            ]));
            res.set_header(AccessControlAllowMethods(vec![
                Method::Get,
                Method::Post,
                Method::Options,
            ]));
            res.set_header(AccessControlMaxAge(1728000u32));
        }))
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
