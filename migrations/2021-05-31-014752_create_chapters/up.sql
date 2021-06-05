-- Your SQL goes here
CREATE TABLE chapters (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    description VARCHAR NOT NULL,
    pages INT NOT NULL,
    manga_id INT NOT NULL,
    CONSTRAINT fk_chapter_manga
        FOREIGN KEY(manga_id) 
        REFERENCES mangas(id)
);