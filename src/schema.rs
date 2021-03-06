table! {
    collects (id) {
        id -> Varchar,
        rut_id -> Varchar,
        item_id -> Varchar,
        item_order -> Int2,
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
        slug -> Varchar,
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
        author -> Varchar,
        uname -> Varchar,
        credential -> Varchar,
        logo -> Varchar,
        item_count -> Int4,
        comment_count -> Int4,
        star_count -> Int4,
        vote -> Int4,
        slug -> Varchar,
    }
}

table! {
    staritems (id) {
        id -> Varchar,
        uname -> Varchar,
        item_id -> Varchar,
        star_at -> Timestamp,
        note -> Varchar,
        flag -> Int2,
        rate -> Int2,
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
        tag_at -> Timestamp,
    }
}

table! {
    tagitems (id) {
        id -> Varchar,
        tname -> Varchar,
        item_id -> Varchar,
        count -> Int4,
        tag_at -> Timestamp,
    }
}

table! {
    tagruts (id) {
        id -> Varchar,
        tname -> Varchar,
        rut_id -> Varchar,
        count -> Int4,
        tag_at -> Timestamp,
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
        permission -> Int2,
        link -> Varchar,
        auth_from -> Varchar,
        email_confirmed -> Bool,
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
