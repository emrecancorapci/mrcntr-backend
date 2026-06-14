CREATE TABLE IF NOT EXISTS projects (
    id SERIAL PRIMARY KEY,
    title VARCHAR (150) NOT NULL,
    project_description TEXT NOT NULL,
    content TEXT NOT NULL,
    
    -- Details
    year_created_at SMALLINT NOT NULL,
    latest_version VARCHAR(50),

    -- References
    project_status_id INT REFERENCES project_statuses(id) ON DELETE
    SET NULL,
    project_type_id INT REFERENCES project_types(id) ON DELETE
    SET NULL,
    project_ai_usage_id INT REFERENCES project_ai_usage(id) ON DELETE
    SET NULL,

    -- Statuses
    is_featured BOOLEAN NOT NULL DEFAULT FALSE,
    is_visible BOOLEAN NOT NULL DEFAULT TRUE,

    -- Dates
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    published_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ
)