CREATE TABLE IF NOT EXISTS featured_blogposts (
    id SERIAL PRIMARY KEY,
    blogpost_id INT REFERENCES blogpost(id) ON DELETE CASCADE,
    sort_value SMALLINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    deleted_at TIMESTAMPTZ
)