use diesel::prelude::*;

mod schema {
    table! {
        shorty (id) {
            id -> Integer,
            handle -> Text,
            full_url -> Text,
            created -> Text,
        }
    }
}
