CREATE TABLE todo
(
    id          SERIAL  PRIMARY KEY,
    text        TEXT    NOT NULL,
    completed   BOOLEAN NOT NULL DEFAULT false
);
