CREATE TABLE habit_completions (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    habit_id integer NOT NULL,
    completion_timestamp timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (habit_id) REFERENCES habits(id)
);