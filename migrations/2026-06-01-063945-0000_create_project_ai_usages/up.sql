CREATE TABLE IF NOT EXISTS project_ai_usages (
    id SERIAL PRIMARY KEY,
    title VARCHAR(50) NOT NULL,
    val SMALLINT NOT NULL,
    description VARCHAR(500)
)