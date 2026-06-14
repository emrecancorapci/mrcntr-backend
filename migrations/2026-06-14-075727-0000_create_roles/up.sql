CREATE TABLE IF NOT EXISTS roles (
    id SERIAL PRIMARY KEY,
    title VARCHAR(15)
);

INSERT INTO roles
VALUES
(1, 'admin'),
(2, 'author'),
(3, 'user');