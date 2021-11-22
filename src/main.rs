#[macro_use]
extern crate juniper;

#[macro_use]
extern crate diesel;

use crate::db::init_pool;
use crate::db::DbPool;
use crate::graphql::{create_schema, Context, Schema};
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use dotenv::dotenv;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod db;
mod graphql;

itconfig::config! {
    DATABASE_URL: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("dotenv setup not successful");

    let pool = init_pool();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(create_schema())
            .route("/graphql", web::post().to(graphql))
            .route("/graphiql", web::get().to(graphiql))
    })
    .bind("localhost:8080")?
    .run()
    .await
}

pub async fn graphql(
    pool: web::Data<DbPool>,
    schema: web::Data<Schema>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = Context {
        db_pool: pool.get_ref().to_owned(),
    };
    let res = web::block(move || {
        let res = data.execute_sync(&schema, &ctx);
        serde_json::to_string(&res)
    })
    .await
    .map_err(Error::from)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res))
}

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://localhost:8080/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
