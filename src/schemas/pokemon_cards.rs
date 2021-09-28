use uuid::Uuid;

use crate::diesel_schema::pokemon_cards;

#[derive(Serialize, Deserialize)]
pub struct PokemonCreate {
    pub id: Option<Uuid>,// Option type for main key
    pub type_item: String,
    pub keytag: String,
    pub product_state: String,
    pub lang_edition: String,
    pub subcollection: String,
    pub collection_name: String,
    pub collection_str_id: String,
    pub pokemon_str_id: String,
    pub pokemon_description: String,
    pub is_available: bool,
}

#[derive(Serialize, Deserialize, AsChangeset, Identifiable)]
#[table_name = "pokemon_cards"]
pub struct PokemonUpdate {
    pub id: Uuid,// not Option for main key
    pub type_item: Option<String>,
    pub keytag: Option<String>,
    pub product_state: Option<String>,
    pub lang_edition: Option<String>,
    pub subcollection: Option<String>,
    pub collection_name: Option<String>,
    pub collection_str_id: Option<String>,
    pub pokemon_str_id: Option<String>,
    pub pokemon_description: Option<String>,
    pub is_available: Option<bool>,
}
