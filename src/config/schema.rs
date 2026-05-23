// @generated automatically by Diesel CLI.

diesel::table! {
    experience_tags (experience_id, tag_slug) {
        experience_id -> Int4,
        #[max_length = 50]
        tag_slug -> Varchar,
    }
}

diesel::table! {
    experiences (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        company_name -> Varchar,
        description -> Text,
        #[max_length = 255]
        location -> Varchar,
        start_date -> Date,
        end_date -> Nullable<Date>,
    }
}

diesel::table! {
    tags (slug) {
        #[max_length = 50]
        slug -> Varchar,
        #[max_length = 50]
        title -> Varchar,
    }
}

diesel::joinable!(experience_tags -> experiences (experience_id));
diesel::joinable!(experience_tags -> tags (tag_slug));

diesel::allow_tables_to_appear_in_same_query!(experience_tags, experiences, tags,);
