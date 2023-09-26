-- user
CREATE TABLE twitch.user (
    user_id VARCHAR(10) PRIMARY KEY,
    user_login VARCHAR(32) NOT NULL,
    cached_user_name VARCHAR(32)
);

-- authentication
CREATE TABLE twitch.auth (
    user_id VARCHAR(10) NOT NULL,
    refresh_token VARCHAR(50) NOT NULL,
    token VARCHAR(30) NOT NULL,
    last_authenticated TIMESTAMP NOT NULL,
    CONSTRAINT FK_auth_user FOREIGN KEY(user_id) REFERENCES twitch.user(user_id)
);

CREATE UNIQUE INDEX idx_user_id ON twitch.auth(user_id);
