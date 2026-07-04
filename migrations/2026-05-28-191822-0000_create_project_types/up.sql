CREATE TABLE IF NOT EXISTS project_types (
    id SERIAL PRIMARY KEY,
    title VARCHAR(50) NOT NULL,
    sort_order SMALLINT NOT NULL,
    
    -- Dates
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    deleted_at TIMESTAMPTZ
)