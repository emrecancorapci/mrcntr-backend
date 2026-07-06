CREATE TABLE IF NOT EXISTS projects_tags (
    project_id INT REFERENCES projects(id) ON DELETE CASCADE,
    tag_id INT REFERENCES tags(id) ON DELETE CASCADE ON UPDATE CASCADE,
    sort_order SMALLINT NOT NULL,
    is_featured BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (project_id, tag_id)
)