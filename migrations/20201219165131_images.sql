-- Add migration script here

CREATE TABLE image_drafts (
    image_thumbnail_path TEXT,
    image_medium_path TEXT,
    image_full_path TEXT
);

ALTER TABLE posts ADD COLUMN images TEXT NOT NULL DEFAULT "[]";
