-- Add migration script here
CREATE TABLE registration (
    id uuid NOT NULL,
    PRIMARY KEY (id),
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    registered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
