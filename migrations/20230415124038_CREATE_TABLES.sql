-- Add migration script here
CREATE TABLE search_parameter (
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    city VARCHAR(255),
    state VARCHAR(2),
    interest_points VARCHAR(255),
    enabled BIT
);

CREATE TABLE search (
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    started_at TIMESTAMP,
    finished_at TIMESTAMP,
    error VARCHAR(255),
    search_parameter_id VARCHAR(36) NOT NULL,
    FOREIGN KEY (search_parameter_id) REFERENCES search_parameter(id)
);

CREATE TABLE location (
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    name VARCHAR(255),
    kind VARCHAR(255),
    latitude DECIMAL,
    longitude DECIMAL,
    street VARCHAR(255),
    neighbourhood VARCHAR(255),
    number VARCHAR(255),
    zip_code VARCHAR(255)
);