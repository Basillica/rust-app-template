-- DROP TABLE IF EXISTS todos;

-- CREATE TABLE todos (
--     id serial PRIMARY KEY,
--     user_id BIGINT NULL,
--     todo  TEXT NOT NULL,
-- );

-- DROP TABLE IF EXISTS users;

-- CREATE TABLE users (
--     id TEXT PRIMARY KEY,
--     firstname TEXT NOT NULL,
--     lastname TEXT NOT NULL,
--     -- password TEXT NOT NULL,
--     emai TEXT UNIQUE NOT NULL,
-- );


CREATE TABLE IF NOT EXISTS todos (
  id serial PRIMARY KEY,
  user_id BIGINT NULL,
  todo TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    firstname TEXT NOT NULL,
    lastname TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL
);
