table! {
    collects (id) {
        id -> Varchar,
        rut_id -> Varchar,
        item_id -> Varchar,
        item_order -> Int4,
        content -> Text,
        uname -> Varchar,
        collect_at -> Timestamp,
    }
}

table! {
    etcs (id) {
        id -> Varchar,
        content -> Text,
        post_at -> Timestamp,
        petc_id -> Varchar,
        rut_id -> Varchar,
        item_id -> Varchar,
        tname -> Varchar,
        uname -> Varchar,
        vote -> Int4,
    }
}

table! {
    follows (id) {
        id -> Varchar,
        uname -> Varchar,
        fname -> Varchar,
        fo_at -> Timestamp,
        note -> Varchar,
    }
}

table! {
    items (id) {
        id -> Varchar,
        title -> Varchar,
        uiid -> Varchar,
        authors -> Varchar,
        pub_at -> Varchar,
        publisher -> Varchar,
        category -> Varchar,
        url -> Varchar,
        cover -> Varchar,
        edition -> Varchar,
        detail -> Text,
        rut_count -> Int4,
        etc_count -> Int4,
        done_count -> Int4,
        vote -> Int4,
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
        uname -> Varchar,
        credential -> Varchar,
        logo -> Varchar,
        item_count -> Int4,
        comment_count -> Int4,
        star_count -> Int4,
        vote -> Int4,
    }
}

table! {
    staritems (id) {
        id -> Varchar,
        uname -> Varchar,
        item_id -> Varchar,
        star_at -> Timestamp,
        note -> Varchar,
        flag -> Varchar,
        rate -> Int4,
    }
}

table! {
    starruts (id) {
        id -> Varchar,
        uname -> Varchar,
        rut_id -> Varchar,
        star_at -> Timestamp,
        note -> Varchar,
    }
}

table! {
    startags (id) {
        id -> Varchar,
        uname -> Varchar,
        tname -> Varchar,
        star_at -> Timestamp,
        note -> Varchar,
    }
}

table! {
    tagetcs (id) {
        id -> Varchar,
        tname -> Varchar,
        etc_id -> Varchar,
    }
}

table! {
    tagitems (id) {
        id -> Varchar,
        tname -> Varchar,
        item_id -> Varchar,
        count -> Int4,
    }
}

table! {
    tagruts (id) {
        id -> Varchar,
        tname -> Varchar,
        rut_id -> Varchar,
        count -> Int4,
    }
}

table! {
    tags (id) {
        id -> Varchar,
        tname -> Varchar,
        intro -> Text,
        logo -> Varchar,
        pname -> Varchar,
        item_count -> Int4,
        rut_count -> Int4,
        etc_count -> Int4,
        star_count -> Int4,
        vote -> Int4,
    }
}

table! {
    timelines (id) {
        id -> Varchar,
        uname -> Varchar,
        action -> Varchar,
        obj -> Varchar,
        objid -> Varchar,
        act_at -> Timestamp,
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
        location -> Varchar,
        nickname -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    collects,
    etcs,
    follows,
    items,
    ruts,
    staritems,
    starruts,
    startags,
    tagetcs,
    tagitems,
    tagruts,
    tags,
    timelines,
    users,
);
