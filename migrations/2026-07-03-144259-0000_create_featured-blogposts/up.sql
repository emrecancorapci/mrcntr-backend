CREATE TABLE IF NOT EXISTS featured_blogposts (
    id SERIAL PRIMARY KEY,
    blogpost_id INT REFERENCES blogposts(id) ON DELETE CASCADE,
    sort_value SMALLINT NOT NULL,

    -- Dates
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    deleted_at TIMESTAMPTZ
)