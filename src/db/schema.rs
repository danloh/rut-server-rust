table! {
    users (id) {
        id -> Varchar,
        uname -> Text,
        password -> Text,
        join_at -> Timestamp,
        avatar -> Text,
    }
}

table! {
    ruts (id) {
        id -> Varchar,
        title -> Text,
        content -> Text,
        join_at -> Timestamp,
        user_id -> Text,
        user_intro -> Text,
        item_count -> i32,
        comment_count -> i32,
        star_count -> i32,
    }
}