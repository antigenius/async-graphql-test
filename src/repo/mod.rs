use crate::domain::CharacterEntity;


#[derive(Clone)]
pub struct Repo;

impl Repo {
    pub async fn get(&self) -> CharacterEntity {
        // Super async thing happening here
        CharacterEntity::new(
            "Foo".into(),
            "Bar".into()
        )
    }

    pub async fn insert(&self, character: CharacterEntity) {
        // pretend an asynchoronous write happens here
        println!("Just created Character ({})", character.id)
    }
}
