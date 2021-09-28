table! {
    pokemon_cards (id) {
        id -> Uuid,
        type_item -> Varchar,
        keytag -> Varchar,
        product_state -> Varchar,
        lang_edition -> Varchar,
        subcollection -> Varchar,
        collection_name -> Varchar,
        collection_str_id -> Varchar,
        pokemon_str_id -> Varchar,
        pokemon_description -> Varchar,
        is_available -> Bool,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        is_active -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    pokemon_cards,
    users,
);
