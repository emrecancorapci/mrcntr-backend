CREATE TABLE IF NOT EXISTS experiences_tags (
    experience_id INT REFERENCES experiences(id) ON DELETE CASCADE,
    tag_id INT REFERENCES tags(id) ON DELETE CASCADE ON UPDATE CASCADE,
    sort_order SMALLINT,
    PRIMARY KEY (experience_id, tag_id)
);