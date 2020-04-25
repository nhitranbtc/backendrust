table! {
    users (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Varchar,
        company -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}
