-- Add migration script here
-- define relation ship BETWEEN the user information and the otp
ALTER TABLE user_information
ADD COLUMN otp_id UUID;

-- add foreign KEY
ALTER TABLE user_information ADD CONSTRAINT fk_otp_id FOREIGN KEY (otp_id) REFERENCES one_time_passwords (id);