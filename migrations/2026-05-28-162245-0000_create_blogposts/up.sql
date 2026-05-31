CREATE TABLE IF NOT EXISTS blogposts (
    id SERIAL PRIMARY KEY,
    title VARCHAR(127) NOT NULL,
    slug VARCHAR(127) NOT NULL UNIQUE,
    content TEXT,

    -- References
    category_slug VARCHAR(50) REFERENCES categories(slug) ON DELETE
    SET NULL,
    
    -- Dates
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
)