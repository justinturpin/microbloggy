-- Add migration script here

CREATE TABLE image_uploads (
    post_id INT,
    image_thumbnail_path TEXT,
    image_medium_path TEXT,
    image_full_path TEXT
);

CREATE INDEX image_uploads_post_id ON image_uploads(post_id);
