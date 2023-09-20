use actix_web::HttpResponse;
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};


pub async fn playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}
