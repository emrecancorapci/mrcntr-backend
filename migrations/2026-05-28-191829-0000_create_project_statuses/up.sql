CREATE TABLE IF NOT EXISTS project_statuses (
    id SERIAL PRIMARY KEY,
    title VARCHAR(50) NOT NULL,
    sort_value SMALLINT,

    -- Dates
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    deleted_at TIMESTAMPTZ
)