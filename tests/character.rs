use fakeit::{name, words};
use reqwest_graphql::Client;
use uuid::{Uuid, Version};

use async_graphql_test::{Application, CharacterType, CreateCharacterInputType};


#[derive(serde::Serialize)]
struct Vars {
    input: CreateCharacterInputType,
}

async fn spawn_app() {
    let app = Application::new();
    tokio::spawn(app.run_until_stopped());
}

#[tokio::test]
async fn test_create_character() {
    let _ = spawn_app().await;
    let character = CreateCharacterInputType {
        full_name: name::full(),
        description: words::sentence(4),
    };
    let vars = Vars { input: character };
    let query = r#"
        mutation CreateNewCharacter($input: CreateCharacterInputType) {
            createCharacter(character: $input) {
                id
                fullName
                description
            }
        }
    "#;
    let client = Client::new("http://127.0.0.1:8000");
    let response = client
        .query_with_vars::<CharacterType, Vars>(query, vars)
        .await
        .unwrap();
    
    let id = Uuid::parse_str(&response.id).unwrap();
    assert_eq!(Some(Version::Random), id.get_version())
}