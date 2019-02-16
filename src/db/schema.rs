table! {
    collects (id) {
        id -> Varchar,
        rut_id -> Varchar,
        item_id -> Varchar,
        item_order -> Int4,
        content -> Text,
        creator_id -> Varchar,
        collect_at -> Timestamp,
    }
}

table! {
    etcs (id) {
        id -> Varchar,
        content -> Text,
        create_at -> Timestamp,
        etc_id -> Varchar,
    }
}

table! {
    items (id) {
        id -> Varchar,
        title -> Varchar,
        uiid -> Varchar,
        pub_at -> Varchar,
        authors -> Varchar,
        publisher -> Varchar,
        category -> Varchar,
        url -> Varchar,
        cover -> Varchar,
        edition -> Varchar,
        detail -> Text,
        rut_count -> Int4,
        etc_count -> Int4,
        done_count -> Int4,
    }
}

table! {
    ruts (id) {
        id -> Varchar,
        title -> Varchar,
        url -> Varchar,
        content -> Text,
        create_at -> Timestamp,
        renew_at -> Timestamp,
        author_id -> Varchar,
        user_id -> Varchar,
        credential -> Varchar,
        item_count -> Int4,
        comment_count -> Int4,
        star_count -> Int4,
    }
}

table! {
    tagetcs (id) {
        id -> Varchar,
        tag_id -> Varchar,
        etc_id -> Varchar,
    }
}

table! {
    tagitems (id) {
        id -> Varchar,
        tag_id -> Varchar,
        item_id -> Varchar,
    }
}

table! {
    tagruts (id) {
        id -> Varchar,
        tag_id -> Varchar,
        rut_id -> Varchar,
    }
}

table! {
    tags (id) {
        id -> Varchar,
        tname -> Varchar,
        intro -> Text,
        logo -> Varchar,
        parent_id -> Varchar,
        item_count -> Int4,
        rut_count -> Int4,
        etc_count -> Int4,
        star_count -> Int4,
    }
}

table! {
    users (id) {
        id -> Varchar,
        uname -> Varchar,
        password -> Varchar,
        join_at -> Timestamp,
        avatar -> Varchar,
        email -> Varchar,
        intro -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    collects,
    etcs,
    items,
    ruts,
    tagetcs,
    tagitems,
    tagruts,
    tags,
    users,
);
