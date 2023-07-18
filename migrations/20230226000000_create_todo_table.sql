-- -- Updated with Uuid for ID'ss
-- CREATE TABLE IF NOT EXISTS users (
--   id         TEXT NOT NULL PRIMARY KEY,
--   username   TEXT NOT NULL UNIQUE,
--   password   TEXT NOT NULL,
--   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
-- );

-- CREATE TABLE IF NOT EXISTS user_permissions (
--     user_id  TEXT NOT NULL,
--     token    TEXT NOT NULL,
--     FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
-- );

-- CREATE TABLE IF NOT EXISTS todos (
--   id         TEXT PRIMARY KEY,
--   user_id    TEXT NOT NULL,
--   title      TEXT NOT NULL,
--   completed  BOOLEAN,
--   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
--   is_guest  BOOLEAN NOT NULL
--   -- FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
-- );



CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
  id         UUID NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
  username   VARCHAR(255) NOT NULL UNIQUE,
  password   TEXT NOT NULL,
  created_at TIMESTAMPTZ DEFAULT current_timestamp
);

CREATE TABLE IF NOT EXISTS user_permissions (
    user_id  UUID NOT NULL,
    token    TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS todos (
  id         UUID NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
  user_id    UUID NOT NULL,
  title      TEXT NOT NULL,
  completed  BOOLEAN,
  created_at TIMESTAMPTZ DEFAULT current_timestamp,
  is_guest   BOOLEAN NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);


CREATE TABLE IF NOT EXISTS chat_message_pairs (
    message_id        UUID NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    chat_id           UUID NOT NULL,
    user_message      TEXT,
    assistant_message TEXT,
    message_time      TIMESTAMPTZ,
    documents_upload  TEXT[]
);

CREATE TABLE IF NOT EXISTS chats (
    chat_id        UUID NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    user_id        UUID NOT NULL,
    creation_time  TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    chat_name      VARCHAR(255),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);












-- CREATE TABLE IF NOT EXISTS users (
--   id         INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
--   username   TEXT NOT NULL UNIQUE,
--   password   TEXT NOT NULL,
--   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
-- );

-- CREATE TABLE IF NOT EXISTS user_permissions (
--     user_id  INTEGER NOT NULL,
--     token    TEXT NOT NULL
-- );

-- -- INSERT INTO users (id, anonymous, username, password) 
-- -- SELECT 0, true, 'Guest', ''
-- -- ON CONFLICT(id) DO UPDATE SET
-- --     anonymous = EXCLUDED.anonymous,
-- --     username = EXCLUDED.username;


-- CREATE TABLE IF NOT EXISTS todos (
--   id         INTEGER PRIMARY KEY AUTOINCREMENT,
--   user_id    INTEGER NOT NULL,
--   title      TEXT NOT NULL,
--   completed  BOOLEAN,
--   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
--   -- FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
-- );


