CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    -- Add other fields as needed
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);