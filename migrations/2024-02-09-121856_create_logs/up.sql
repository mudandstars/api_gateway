-- Your SQL goes here
CREATE TABLE logs (
    id INT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    api_key_id INT UNSIGNED NOT NULL REFERENCES api_keys(id),
    `method` VARCHAR(6) NOT NULL,
    `uri` VARCHAR(255) NOT NULL,
    `status` SMALLINT UNSIGNED NOT NULL,
    type_ TINYINT UNSIGNED NOT NULL,
    error_message VARCHAR(255),
    duration_in_microseconds BIGINT UNSIGNED NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
