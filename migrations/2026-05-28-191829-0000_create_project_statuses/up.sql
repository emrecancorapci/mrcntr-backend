CREATE TABLE IF NOT EXISTS project_statuses (
    id SERIAL PRIMARY KEY,
    title VARCHAR(50) NOT NULL,
    sort_value SMALLINT
)