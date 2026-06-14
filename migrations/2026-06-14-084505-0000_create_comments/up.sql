CREATE TABLE IF NOT EXISTS comments (
    id SERIAL PRIMARY KEY,
    author_uuid UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE,
    content TEXT NOT NULL,
    parent_comment SERIAL REFERENCES comments(id) ON DELETE CASCADE,

    -- Dates
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP
)