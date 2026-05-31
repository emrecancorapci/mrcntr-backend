// @generated automatically by Diesel CLI.

diesel::table! {
    blogposts (id) {
        id -> Int4,
        #[max_length = 127]
        title -> Varchar,
        #[max_length = 127]
        slug -> Varchar,
        content -> Nullable<Text>,
        #[max_length = 50]
        category_slug -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    categories (slug) {
        #[max_length = 50]
        slug -> Varchar,
        #[max_length = 50]
        title -> Varchar,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    experiences_tags (experience_id, tag_slug) {
        experience_id -> Int4,
        #[max_length = 50]
        tag_slug -> Varchar,
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

diesel::joinable!(blogposts -> categories (category_slug));
diesel::joinable!(experiences_tags -> experiences (experience_id));
diesel::joinable!(experiences_tags -> tags (tag_slug));

diesel::allow_tables_to_appear_in_same_query!(
    blogposts,
    categories,
    experiences,
    experiences_tags,
    tags,
);
