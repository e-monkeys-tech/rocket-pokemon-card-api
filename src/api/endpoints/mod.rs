use rocket::Rocket;
use rocket_cors::CorsOptions;

pub mod users;
pub mod pokemon_cards;

pub fn fuel(rocket: Rocket) -> Rocket {
    let mut rocket = rocket;
    let _cors = CorsOptions::default().to_cors().unwrap();

    rocket = pokemon_cards::fuel(rocket);
    rocket = users::fuel(rocket);
    rocket.attach(_cors)
}
