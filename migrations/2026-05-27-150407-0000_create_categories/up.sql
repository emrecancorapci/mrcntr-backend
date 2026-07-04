CREATE TABLE IF NOT EXISTS categories (
    slug VARCHAR(50) PRIMARY KEY,
    title VARCHAR(50) NOT NULL,

    -- Dates
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    deleted_at TIMESTAMPTZ
);