mod domain;
pub mod gql;
mod http;
mod repo;


use std::net::TcpListener;

use actix_web::{App, guard, HttpServer, web};
use actix_web::dev::Server;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::GraphQL;

use crate::gql::{Query, Mutation};
use crate::http::playground;
use crate::repo::Repo;


pub struct Application {
    server: Server,
}

impl Application {
    pub fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:8000")
            .unwrap();
        let repo = Repo {};
        let server = HttpServer::new(move || {
            let schema = Schema::build(
                Query,
                Mutation,
                EmptySubscription
            )
            .data(repo.clone())
            .finish();

            App::new()
                .app_data(web::Data::new(schema.clone()))
                .service(
                    web::resource("/")
                    .guard(guard::Post())
                    .to(GraphQL::new(schema))
                )
                .service(
                    web::resource("/")
                    .guard(guard::Get())
                    .to(playground)
                )
        })
        .listen(listener)
        .expect("Failed to listen.")
        .run();

        Self { server }
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        println!("Running at: http://127.0.0.1:8000");
        self.server.await
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
