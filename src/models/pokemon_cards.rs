use uuid::Uuid;

use crate::diesel_schema::pokemon_cards;

#[derive(AsChangeset, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "pokemon_cards"]
pub struct Pokemon {
    pub id: Uuid,
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
