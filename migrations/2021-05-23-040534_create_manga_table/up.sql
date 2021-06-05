-- Your SQL goes here
CREATE TABLE mangas (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    description VARCHAR NOT NULL,
    fandom_id INT NOT NULL,
    CONSTRAINT fk_fandom_manga
        FOREIGN KEY(fandom_id) 
        REFERENCES fandoms(id)
);