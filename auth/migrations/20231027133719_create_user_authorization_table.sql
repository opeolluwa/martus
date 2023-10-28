-- Add migration script here
CREATE TABLE
    user_information (
        id uuid PRIMARY KEY NOT NULL,
        email VARCHAR UNIQUE NOT NULL,
        password VARCHAR NOT NULL,
        is_verified BOOLEAN NOT NULL DEFAULT FALSE,
        created_at TIMESTAMP NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMP NOT NULL DEFAULT NOW ()
    )