use std::net::TcpListener;

use actix_web::{App, guard, HttpServer, HttpResponse, web};
use actix_web::dev::Server;
use async_graphql::{Context, EmptySubscription, FieldResult, InputObject, Object, Schema, SimpleObject};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::GraphQL;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


// Domain Model for a character.
#[derive(Clone, Debug, SimpleObject)]
struct CharacterEntity {
    id: String,
    full_name: String,
    description: String,
}

impl CharacterEntity {
    pub fn new(
        full_name: String,
        description: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            full_name,
            description
        }
    }
}

// GQL type for a Character, note the underscore `full_name` which
// `async_graphql` will convert to `fullName` for me because of the
// `SimpleObject` derive.
#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct CharacterType {
    pub id: String,
    pub full_name: String,
    pub description: String,
}

// GQL type to create a character
// #[derive(InputObject, Serialize)]
// // #[Object(rename_all = "camelCase")]
// #[serde(rename_all = "camelCase")]
// pub struct CreateCharacterInputType {
//     #[graphql(name = "fullName")]
//     pub full_name: String,
//     pub description: String,
// }
// the above errors: Failed to parse response


#[derive(InputObject, Serialize)]
#[graphql(rename_fields = "camelCase")]
pub struct CreateCharacterInputType {
    #[graphql(name = "fullName")]
    pub full_name: String,
    pub description: String,
}
// the above errors: Invalid value for argument "character", field "fullName" of type "String!" is required but not provided

// GQL Query type
#[derive(Default)]
struct Query;

#[Object]
impl Query {
    async fn character(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<CharacterEntity> {
        let repo = ctx.data::<Repo>().unwrap();
        Ok(repo.get().await)
    }
}

// GQL Mutation type
struct Mutation;

#[Object]
impl Mutation {
    async fn create_character(
        &self,
        ctx: &Context<'_>,
        character: CreateCharacterInputType,
    ) -> CharacterEntity {
        let c = CharacterEntity::new(
            character.full_name,
            character.description
        );
        let repo = ctx.data::<Repo>().unwrap();
        repo.insert(c.clone()).await;
        c
    }
}

// Mock repo
#[derive(Clone)]
struct Repo;

impl Repo {
    async fn get(&self) -> CharacterEntity {
        // Super async thing happening here
        CharacterEntity::new(
            "Foo".into(),
            "Bar".into()
        )
    }
    async fn insert(&self, character: CharacterEntity) {
        // pretend an asynchoronous write happens here
        println!("Just created Character ({})", character.id)
    }
}

// Web handler
pub async fn playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

// Application struct for testing
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


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Application::new();
    app.run_until_stopped().await?;
    Ok(())
}
