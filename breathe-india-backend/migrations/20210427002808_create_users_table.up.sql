-- Add up migration script here

CREATE TABLE users (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    profile_pic_url TEXT NOT NULL,
    bio TEXT NOT NULL
);
