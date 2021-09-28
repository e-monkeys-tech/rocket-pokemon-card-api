use anyhow::Result;
use rocket::Rocket;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid as RocketUuid;
use uuid::Uuid;

use crate::crud::pokemon_cards;
use crate::db::guard::DbConn;
use crate::models::pokemon_cards::User;
use crate::schemas::pokemon_cards::{PokemonCreate, PokemonUpdate};

#[post("/", format = "json", data = "<obj_in>")]
fn create(obj_in: Json<PokemonCreate>, db: DbConn) -> Result<Json<Pokemon>> {
    let inserted_pokemon = pokemon_cards::create(&db, obj_in.0)?;
    Ok(Json(inserted_pokemon))
}

#[get("/<obj_id>")]
fn read(obj_id: RocketUuid, db: DbConn) -> Result<Json<Pokemon>> {
    let uuid = Uuid::from_bytes(*obj_id.as_bytes());
    let found_pokemon = pokemon_cards::find(&db, uuid)?;
    Ok(Json(found_pokemon))
}

#[patch("/", format = "json", data = "<obj_in>")]
fn update(obj_in: Json<PokemonUpdate>, db: DbConn) -> Result<Json<Pokemon>> {
    let updated_pokemon = pokemon_cards::update(&db, &obj_in.0)?;
    Ok(Json(updated_pokemon))
}

#[delete("/<obj_id>")]
fn delete(obj_id: RocketUuid, db: DbConn) -> Result<Json<Pokemon>> {
    let uuid = Uuid::from_bytes(*obj_id.as_bytes());
    let deleted_pokemon = pokemon_cards::delete(&db, uuid)?;
    Ok(Json(deleted_pokemon))
}

pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.mount("/api/pokemon_cards", routes![create, read, update, delete])
}
