-- Your SQL goes here
CREATE TABLE users_from_fandom (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    fandom_id INT NOT NULL,
    CONSTRAINT fk_user
        FOREIGN KEY(user_id) 
        REFERENCES users(id),
    CONSTRAINT fk_fandom
        FOREIGN KEY(fandom_id) 
        REFERENCES fandoms(id)
);

CREATE UNIQUE INDEX unique_relation_user_from_fandom ON users_from_fandom(user_id, fandom_id);