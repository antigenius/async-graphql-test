use uuid::Uuid;


#[derive(Clone, Debug)]
pub struct CharacterEntity {
    pub id: String,
    pub full_name: String,
    pub description: String,
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
