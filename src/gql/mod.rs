use async_graphql::{Context, FieldResult, InputObject, Object, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::domain::CharacterEntity;
use crate::repo::Repo;


#[derive(Debug, Deserialize, Serialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct CharacterType {
    pub id: String,
    pub full_name: String,
    pub description: String,
}

impl From<CharacterEntity> for CharacterType {
    fn from(character: CharacterEntity) -> Self {
        Self {
            id: character.id,
            full_name: character.full_name,
            description: character.description,
        }
    }
}

#[derive(InputObject, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCharacterInput {
    pub full_name: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct CreateCharacterResponse {
    pub character: CharacterType,
    pub success: bool,
}

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    async fn character(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<CharacterType> {
        let repo = ctx.data::<Repo>().unwrap();
        Ok(repo.get().await.into())
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_character(
        &self,
        ctx: &Context<'_>,
        character: CreateCharacterInput,
    ) -> CreateCharacterResponse {
        let c = CharacterEntity::new(
            character.full_name,
            character.description
        );
        let repo = ctx.data::<Repo>().unwrap();
        repo.insert(c.clone()).await;
        CreateCharacterResponse {
            character: c.into(),
            success: true
        }
    }
}
