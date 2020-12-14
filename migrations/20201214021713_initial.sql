-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    username TEXT NOT NULL,
    name TEXT NOT NULL DEFAULT "Default User",
    bio TEXT NOT NULL DEFAULT "Default Bio"
);

CREATE UNIQUE INDEX users_username ON users(username);

CREATE TABLE IF NOT EXISTS posts (
    user_id INT NOT NULL,
    content TEXT NOT NULL,
    posted_timestamp TEXT NOT NULL,
    short_url TEXT
);

CREATE UNIQUE INDEX posts_short_url ON posts(short_url);

-- Prevent an extraordinarily rare case where multple posts happen at the same time,
-- breaking pagination

CREATE UNIQUE INDEX posts_posted_timestamp ON posts(posted_timestamp);
