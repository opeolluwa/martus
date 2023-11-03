-- Add migration script here
CREATE TABLE
    blacklisted_jwt (
        id UUID PRIMARY KEY NOT NULL,
        token VARCHAR NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT NOW (),
        updated_ad TIMESTAMP NOT NULL DEFAULT NOW ()
    )