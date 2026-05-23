-- Your SQL goes here
CREATE TABLE experiences (
    id SERIAL PRIMARY KEY,                -- Auto-incrementing ID
    title VARCHAR(255) NOT NULL,
    company_name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,            -- TEXT is better than VARCHAR if descriptions get long
    location VARCHAR(255) NOT NULL,
    start_date DATE NOT NULL,             -- Pro-tip: avoid using SQL keywords like 'start' or 'end' as column names!
    end_date DATE                         -- Leave this nullable (no NOT NULL) for current jobs!
);