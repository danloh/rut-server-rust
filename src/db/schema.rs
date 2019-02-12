table! {
    ruts (id) {
        id -> Varchar,
        title -> Text,
        url -> Text,
        content -> Text,
        create_at -> Timestamp,
        user_id -> Varchar,
        user_intro -> Text,
        item_count -> Int4,
        comment_count -> Int4,
        star_count -> Int4,
    }
}

table! {
    users (id) {
        id -> Varchar,
        uname -> Text,
        password -> Text,
        join_at -> Timestamp,
        avatar -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    ruts,
    users,
);
