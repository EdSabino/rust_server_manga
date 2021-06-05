table! {
    chapters (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        pages -> Int4,
        manga_id -> Int4,
    }
}

table! {
    fandoms (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        photo -> Nullable<Varchar>,
    }
}

table! {
    mangas (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        fandom_id -> Int4,
        photo -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        admin -> Bool,
        photo -> Nullable<Varchar>,
    }
}

table! {
    users_from_fandom (id) {
        id -> Int4,
        user_id -> Int4,
        fandom_id -> Int4,
        admin -> Bool,
    }
}

joinable!(chapters -> mangas (manga_id));
joinable!(mangas -> fandoms (fandom_id));
joinable!(users_from_fandom -> fandoms (fandom_id));
joinable!(users_from_fandom -> users (user_id));

allow_tables_to_appear_in_same_query!(
    chapters,
    fandoms,
    mangas,
    users,
    users_from_fandom,
);
