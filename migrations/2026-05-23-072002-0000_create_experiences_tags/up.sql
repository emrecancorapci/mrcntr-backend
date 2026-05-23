-- Your SQL goes here
CREATE TABLE experience_tags (
    experience_id INT REFERENCES experiences(id) ON DELETE CASCADE,
    tag_slug VARCHAR(50) REFERENCES tags(slug) ON DELETE CASCADE ON UPDATE CASCADE,
    PRIMARY KEY (experience_id, tag_slug)
);