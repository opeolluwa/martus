-- Add migration script here
CREATE TABLE
    one_time_passwords (
        id UUID PRIMARY KEY NOT NULL,
        otp VARCHAR(6) NOT NULL,
        exp BIGINT NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT NOW (),
        updated_ad TIMESTAMP NOT NULL DEFAULT NOW ()
    )