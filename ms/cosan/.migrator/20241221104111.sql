CREATE TABLE IF NOT EXISTS users (
    user_id BIGSERIAL,
    last_name VARCHAR(50) NOT NULL,
    first_name VARCHAR(50) NOT NULL,
    login_id VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    country VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id),
    UNIQUE (login_id),
    UNIQUE (email)
);
COMMENT ON TABLE users IS 'user is supported supporters';
COMMENT ON COLUMN users.user_id IS 'user id';
COMMENT ON COLUMN users.last_name IS 'user last name';
COMMENT ON COLUMN users.first_name IS 'user first name';
COMMENT ON COLUMN users.login_id IS 'user login id';
COMMENT ON COLUMN users.password IS 'user password';
COMMENT ON COLUMN users.email IS 'user email';
COMMENT ON COLUMN users.country IS 'user country';

CREATE TABLE IF NOT EXISTS words (
    word_id BIGSERIAL,
    word VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (word_id),
    UNIQUE (word)
);
COMMENT ON TABLE words IS 'words';
COMMENT ON COLUMN words.word_id IS 'word id';
COMMENT ON COLUMN words.word IS 'word';

CREATE TABLE IF NOT EXISTS user_words (
    user_word_id BIGSERIAL,
    user_id BIGINT NOT NULL,
    word_id BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_word_id),
    FOREIGN KEY (user_id) REFERENCES users(user_id),
    FOREIGN KEY (word_id) REFERENCES words(word_id)
);
COMMENT ON TABLE user_words IS 'user words';
COMMENT ON COLUMN user_words.user_word_id IS 'user word id';
COMMENT ON COLUMN user_words.user_id IS 'user id';
COMMENT ON COLUMN user_words.word_id IS 'word id';