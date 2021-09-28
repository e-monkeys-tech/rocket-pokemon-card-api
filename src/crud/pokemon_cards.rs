use anyhow::Result;
use diesel::insert_into;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

use crate::diesel_schema::pokemon_cards;
use crate::models::pokemon_cards::Pokemon;
use crate::schemas::pokemon_cards::{PokemonCreate, PokemonUpdate};

pub fn create(db: &PgConnection, obj_in: PokemonCreate) -> Result<Pokemon> {
    use crate::diesel_schema::pokemon_cards::dsl::*;
    // ?? 
    let mut new_pokemon = Pokemon {
        id: Uuid::new_v4(),
        username: obj_in.username,
        is_active: obj_in.is_active,
    };
    new_pokemon = insert_into(pokemon_cards)
        .values(&new_pokemon)
        .on_conflict_do_nothing()
        .get_result(db)?;
    Ok(new_pokemon)
}

pub fn find(db: &PgConnection, obj_id: Uuid) -> Result<Pokemon> {
    let pokemon = pokemon_cards::table.find(obj_id).get_result::<Pokemon>(db)?;
    Ok(pokemon)
}

pub fn find_by_name(db: &PgConnection, username_in: String) -> Result<Pokemon> {
    use crate::diesel_schema::pokemon_cards::dsl::*;
    let pokemon = pokemon_cards.filter(username.eq(username_in.as_str())).first(db)?;
    Ok(pokemon)
}

pub fn update(db: &PgConnection, obj_in: &PokemonUpdate) -> Result<Pokemon> {
    let updated_pokemon = diesel::update(obj_in).set(obj_in).get_result(db)?;
    Ok(updated_pokemon)
}

pub fn delete(db: &PgConnection, obj_id: Uuid) -> Result<Pokemon> {
    use crate::diesel_schema::pokemon_cards::dsl::*;
    let deleted_pokemon = diesel::delete(pokemon_cards.filter(id.eq(obj_id))).get_result(db)?;
    Ok(deleted_pokemon)
}
