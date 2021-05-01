-- Add up migration script here
CREATE TYPE PostType AS ENUM ('needs', 'supplies');

CREATE TABLE posts (
    id uuid NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    userid uuid NOT NULL REFERENCES users(id) ON UPDATE RESTRICT ON DELETE CASCADE,
    post_type PostType NOT NULL,
    state TEXT NOT NULL,
    district TEXT NOT NULL,
    city TEXT NOT NULL,
    spot TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    message TEXT NOT NULL
);

CREATE TABLE post_items (
    id uuid NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id uuid NOT NULL REFERENCES posts(id) ON UPDATE RESTRICT ON DELETE CASCADE,
    item TEXT NOT NULL,
    quantity TEXT NOT NULL
);
