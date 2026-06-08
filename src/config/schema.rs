// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tag_types"))]
    pub struct TagTypes;
}

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
    experiences_tags (experience_id, tag_id) {
        experience_id -> Int4,
        tag_id -> Int4,
        sort_order -> Nullable<Int2>,
    }
}

diesel::table! {
    project_ai_usage (id) {
        id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        val -> Int2,
        #[max_length = 500]
        description -> Nullable<Varchar>,
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
        ai_usage -> Nullable<Int4>,
        is_featured -> Nullable<Bool>,
        is_visible -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        published_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    projects_tags (project_id, tag_id) {
        project_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TagTypes;

    tags (id) {
        id -> Int4,
        #[max_length = 50]
        slug -> Varchar,
        #[max_length = 50]
        title -> Varchar,
        tag_type -> Nullable<TagTypes>,
        proficiency -> Nullable<Int2>,
        #[max_length = 50]
        icon -> Nullable<Varchar>,
        #[max_length = 50]
        color -> Nullable<Varchar>,
        parent -> Nullable<Int4>,
    }
}

diesel::table! {
    users (uuid) {
        uuid -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(blogposts -> categories (category_slug));
diesel::joinable!(experiences_tags -> experiences (experience_id));
diesel::joinable!(experiences_tags -> tags (tag_id));
diesel::joinable!(project_blocks -> projects (project_id));
diesel::joinable!(project_links -> projects (project_id));
diesel::joinable!(projects -> project_ai_usage (ai_usage));
diesel::joinable!(projects -> project_statuses (project_status));
diesel::joinable!(projects -> project_types (project_type));
diesel::joinable!(projects_tags -> projects (project_id));
diesel::joinable!(projects_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    blogposts,
    categories,
    experiences,
    experiences_tags,
    project_ai_usage,
    project_blocks,
    project_links,
    project_statuses,
    project_types,
    projects,
    projects_tags,
    tags,
    users,
);
