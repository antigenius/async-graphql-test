use fakeit::{name, words};
use reqwest_graphql::Client;
use uuid::{Uuid, Version};

use async_graphql_test::Application;
use async_graphql_test::gql::{CreateCharacterInput, CreateCharacterResponse};


#[derive(serde::Serialize)]
struct Vars {
    input: CreateCharacterInput,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Response {
    create_character: CreateCharacterResponse
}

async fn spawn_app() {
    let app = Application::new();
    tokio::spawn(app.run_until_stopped());
}

#[tokio::test]
async fn test_create_character() {
    let _ = spawn_app().await;
    let character = CreateCharacterInput {
        full_name: name::full(),
        description: words::sentence(4),
    };
    let vars = Vars { input: character };
    let query = r#"
        mutation CreateNewCharacter($input: CreateCharacterInput) {
            createCharacter(character: $input) {
                success
                character {
                    id
                    fullName
                    description
                }
            }
        }
    "#;
    let response = Client::new("http://127.0.0.1:8000")
        .query_with_vars::<Response, Vars>(query, vars)
        .await
        .unwrap();
    
    let id = Uuid::parse_str(&response.create_character.character.id).unwrap();
    assert_eq!(Some(Version::Random), id.get_version())
}
