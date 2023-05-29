CREATE TABLE habits (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name text NOT NULL UNIQUE,
    description text NOT NULL,
    cadence text NOT NULL,
    reps integer NOT NULL
);