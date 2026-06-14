CREATE TABLE IF NOT EXISTS blogposts (
    id SERIAL PRIMARY KEY,
    title VARCHAR(127) NOT NULL,
    slug VARCHAR(127) NOT NULL UNIQUE,
    content TEXT,
    author_uuid UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE,
    is_visible BOOLEAN NOT NULL DEFAULT TRUE,

    -- References
    category_slug VARCHAR(50) REFERENCES categories(slug) ON DELETE
    SET NULL,
    
    -- Dates
    published_at TIMESTAMP NOT NULL DEFAULT current_timestamp, 
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at TIMESTAMP
)