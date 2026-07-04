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
        author_uuid -> Uuid,
        is_visible -> Bool,
        #[max_length = 50]
        category_slug -> Nullable<Varchar>,
        published_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    categories (slug) {
        #[max_length = 50]
        slug -> Varchar,
        #[max_length = 50]
        title -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    comments (id) {
        id -> Int4,
        author_uuid -> Uuid,
        content -> Text,
        blogpost_id -> Int4,
        parent_comment_id -> Nullable<Int4>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
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
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
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
    featured_blogposts (id) {
        id -> Int4,
        blogpost_id -> Nullable<Int4>,
        sort_value -> Int2,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    project_ai_usages (id) {
        id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        val -> Int2,
        #[max_length = 500]
        description -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    project_blocks (id) {
        id -> Int4,
        sort_order -> Int2,
        #[max_length = 50]
        title -> Varchar,
        content -> Text,
        is_active -> Bool,
        project_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
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
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    project_statuses (id) {
        id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        sort_value -> Nullable<Int2>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    project_types (id) {
        id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        sort_order -> Int2,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    projects (id) {
        id -> Int4,
        #[max_length = 150]
        title -> Varchar,
        project_description -> Text,
        content -> Text,
        year_created_at -> Int2,
        #[max_length = 50]
        latest_version -> Nullable<Varchar>,
        project_status_id -> Nullable<Int4>,
        project_type_id -> Nullable<Int4>,
        project_ai_usage_id -> Nullable<Int4>,
        is_featured -> Bool,
        is_visible -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        published_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    projects_tags (project_id, tag_id) {
        project_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        #[max_length = 15]
        title -> Varchar,
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
        parent_id -> Nullable<Int4>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (uuid) {
        uuid -> Uuid,
        #[max_length = 50]
        first_name -> Nullable<Varchar>,
        #[max_length = 50]
        last_name -> Nullable<Varchar>,
        summary -> Nullable<Text>,
        #[max_length = 500]
        image_url -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        role_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(blogposts -> categories (category_slug));
diesel::joinable!(blogposts -> users (author_uuid));
diesel::joinable!(comments -> blogposts (blogpost_id));
diesel::joinable!(comments -> users (author_uuid));
diesel::joinable!(experiences_tags -> experiences (experience_id));
diesel::joinable!(experiences_tags -> tags (tag_id));
diesel::joinable!(featured_blogposts -> blogposts (blogpost_id));
diesel::joinable!(project_blocks -> projects (project_id));
diesel::joinable!(project_links -> projects (project_id));
diesel::joinable!(projects -> project_ai_usages (project_ai_usage_id));
diesel::joinable!(projects -> project_statuses (project_status_id));
diesel::joinable!(projects -> project_types (project_type_id));
diesel::joinable!(projects_tags -> projects (project_id));
diesel::joinable!(projects_tags -> tags (tag_id));
diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    blogposts,
    categories,
    comments,
    experiences,
    experiences_tags,
    featured_blogposts,
    project_ai_usages,
    project_blocks,
    project_links,
    project_statuses,
    project_types,
    projects,
    projects_tags,
    roles,
    tags,
    users,
);
