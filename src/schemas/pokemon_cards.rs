use uuid::Uuid;

use crate::diesel_schema::pokemon_cards;

#[derive(Serialize, Deserialize)]
pub struct PokemonCreate {
    pub id: Option<Uuid>,
    pub username: String,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize, AsChangeset, Identifiable)]
#[table_name = "pokemon_cards"]
pub struct PokemonUpdate {
    pub id: Uuid,
    pub username: Option<String>,
    pub is_active: Option<bool>,
}
