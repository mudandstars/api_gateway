-- Your SQL goes here
CREATE TABLE users (
    id INT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(255) NOT NULL,
    email TEXT NOT NULL
);