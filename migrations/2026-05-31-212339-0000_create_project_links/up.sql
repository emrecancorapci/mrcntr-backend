CREATE TABLE IF NOT EXISTS project_links (
    id SERIAL PRIMARY KEY,
    sort_order SMALLINT NOT NULL,
    title VARCHAR(50) NOT NULL,
    link VARCHAR(255) NOT NULL,

    -- References
    project_id INT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,

    -- Dates
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
)