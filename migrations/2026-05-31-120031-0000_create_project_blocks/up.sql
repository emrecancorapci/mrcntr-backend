CREATE TABLE IF NOT EXISTS project_blocks (
    id SERIAL PRIMARY KEY,
    sort_order SMALLINT NOT NULL,
    title VARCHAR(50) NOT NULL,
    content TEXT NOT NULL, -- Markdown

    -- References
    project_id INT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,

    -- Dates
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
)