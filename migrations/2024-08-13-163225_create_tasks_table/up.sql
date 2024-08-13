CREATE TABLE tasks (
    id VARCHAR(256) NOT NULL PRIMARY KEY
    , title TEXT NOT NULL
    , description TEXT
    , due_date DATE
    , is_done BOOLEAN NOT NULL
);
