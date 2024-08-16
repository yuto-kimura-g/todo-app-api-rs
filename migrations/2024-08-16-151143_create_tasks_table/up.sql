CREATE TABLE tasks (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY
    , title TEXT NOT NULL
    , description TEXT
    , due_date DATETIME
    , is_done BOOLEAN NOT NULL
    -- , created_at DATETIME NOT NULL
    -- , updated_at DATETIME NOT NULL
);
