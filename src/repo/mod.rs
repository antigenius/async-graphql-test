use crate::domain::CharacterEntity;


#[derive(Clone)]
pub struct Repo;

impl Repo {
    pub async fn get(&self, id: String) -> CharacterEntity {
        // Super async thing happening here
        let c = CharacterEntity::new(
            "Foo".into(),
            "Bar".into()
        );

        let c = CharacterEntity {
            id,
            full_name: c.full_name,
            description: c.description
        };

        c
    }

    pub async fn insert(&self, character: CharacterEntity) {
        // pretend an asynchoronous write happens here
        println!("Just created Character ({})", character.id)
    }
}
