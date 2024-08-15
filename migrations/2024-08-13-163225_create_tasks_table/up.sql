CREATE TABLE tasks (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY
    , title TEXT NOT NULL
    , description TEXT
    , due_date DATETIME
    -- , due_date TIMESTAMP
    , is_done BOOLEAN NOT NULL
    -- , created_at TIMESTAMP NOT NULL
    -- , updated_at TIMESTAMP NOT NULL
);
