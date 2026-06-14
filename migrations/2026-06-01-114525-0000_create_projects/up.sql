CREATE TABLE IF NOT EXISTS projects (
    id SERIAL PRIMARY KEY,
    title VARCHAR (150) NOT NULL,
    project_description TEXT NOT NULL,
    content TEXT NOT NULL,
    
    -- Details
    year_created_at SMALLINT,
    latest_version VARCHAR(50),

    -- References
    project_status INT REFERENCES project_statuses(id) ON DELETE
    SET NULL,
    project_type INT REFERENCES project_types(id) ON DELETE
    SET NULL,
    ai_usage INT REFERENCES project_ai_usage(id) ON DELETE
    SET NULL,

    -- Statuses
    is_featured BOOLEAN,
    is_visible BOOLEAN,

    -- Dates
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    published_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ
)