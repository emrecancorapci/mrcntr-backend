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
    project_blocks (id) {
        id -> Int4,
        sort_order -> Int2,
        #[max_length = 50]
        title -> Varchar,
        content -> Text,
        project_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    project_links (id) {
        id -> Int4,
        sort_order -> Int2,
        #[max_length = 50]
        title -> Varchar,
        #[max_length = 255]
        link -> Varchar,
        project_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    project_status (id) {
        id -> Int4,
        #[max_length = 50]
        title -> Varchar,
    }
}

diesel::table! {
    project_statuses (id) {
        id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        sort_value -> Nullable<Int2>,
    }
}

diesel::table! {
    project_types (id) {
        id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        sort_order -> Int2,
    }
}

diesel::table! {
    projects (id) {
        id -> Int4,
        #[max_length = 150]
        title -> Varchar,
        project_description -> Text,
        content -> Text,
        year_created_at -> Nullable<Int2>,
        #[max_length = 50]
        latest_version -> Nullable<Varchar>,
        project_status -> Nullable<Int4>,
        project_type -> Nullable<Int4>,
        is_featured -> Nullable<Bool>,
        is_visible -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        published_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
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
diesel::joinable!(project_blocks -> projects (project_id));
diesel::joinable!(project_links -> projects (project_id));
diesel::joinable!(projects -> project_statuses (project_status));
diesel::joinable!(projects -> project_types (project_type));

diesel::allow_tables_to_appear_in_same_query!(
    blogposts,
    categories,
    experiences,
    experiences_tags,
    project_blocks,
    project_links,
    project_status,
    project_statuses,
    project_types,
    projects,
    tags,
);
