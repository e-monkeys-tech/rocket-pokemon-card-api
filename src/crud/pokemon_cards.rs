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
 
    let mut new_pokemon = Pokemon {
        id: Uuid::new_v4(),
        type_item: obj_in.type_item,
        keytag: obj_in.keytag,
        product_state: obj_in.product_state,
        lang_edition: obj_in.lang_edition,
        subcollection: obj_in.subcollection,
        collection_name: obj_in.collection_name,
        collection_str_id: obj_in.collection_str_id,
        pokemon_str_id: obj_in.pokemon_str_id,
        pokemon_description: obj_in.pokemon_description,
        is_available: obj_in.is_available,
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

pub fn find_by_name(db: &PgConnection, pokemon: String) -> Result<Pokemon> {
    use crate::diesel_schema::pokemon_cards::dsl::*;
    let pokemon = pokemon_cards.filter(pokemon_description.eq(pokemon.as_str())).first(db)?;
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
