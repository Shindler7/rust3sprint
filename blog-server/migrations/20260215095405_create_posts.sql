-- Добавление таблицы posts.
CREATE TABLE IF NOT EXISTS posts (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    author_id BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE
    );

-- Внешний ключ с ON DELETE CASCADE
ALTER TABLE posts
    ADD CONSTRAINT fk_posts_author_id
        FOREIGN KEY (author_id)
            REFERENCES users(id)
            ON DELETE CASCADE;

-- Индексы
CREATE INDEX IF NOT EXISTS idx_posts_author_id ON posts(author_id);
CREATE INDEX IF NOT EXISTS idx_posts_created_at ON posts(created_at);
