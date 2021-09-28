use uuid::Uuid;

use crate::diesel_schema::pokemon_cards;

#[table_name = "pokemon_cards"]
#[derive(AsChangeset, Queryable, Insertable, Serialize, Deserialize)]
pub struct Pokemon {
    pub id: Uuid,
    pub username: String,
    pub is_active: bool,
}
