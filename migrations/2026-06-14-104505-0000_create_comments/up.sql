CREATE TABLE IF NOT EXISTS comments (
    id SERIAL PRIMARY KEY,
    author_uuid UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE,
    content TEXT NOT NULL,
    blogpost_id INT NOT NULL REFERENCES blogposts(id) ON DELETE CASCADE,
    parent_comment_id INT REFERENCES comments(id) ON DELETE CASCADE,

    -- Dates
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
)