CREATE TYPE TAG_TYPES AS ENUM ('language', 'tech', 'tool', 'os', 'feature');
CREATE TABLE IF NOT EXISTS tags (
    id SERIAL PRIMARY KEY,
    slug VARCHAR(50) NOT NULL UNIQUE,
    title VARCHAR(50) NOT NULL,
    tag_type TAG_TYPES,
    proficiency SMALLINT,
    icon VARCHAR(50),
    color VARCHAR(50),

    -- References
    parent_id INT REFERENCES tags(id) ON DELETE
    SET NULL,

    -- Dates
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    deleted_at TIMESTAMPTZ
);