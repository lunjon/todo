CREATE TABLE todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created TEXT NOT NULL,
    status TEXT NOT NULL,
    prio TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    tags TEXT,
    context TEXT,
    FOREIGN KEY (context) REFERENCES contexts(name)
    ON DELETE CASCADE
);

CREATE TABLE events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    action TEXT NOT NULL,
    json TEXT NOT NULL,
    ts INTEGER NOT NULL
);

-- Only one row to track current context.
CREATE TABLE context (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    value TEXT,
    FOREIGN KEY (value) REFERENCES contexts(name)
);

-- Keeps record of all valid contexts.
CREATE TABLE contexts (
    name TEXT PRIMARY KEY
);

-- Set current context to NULL.
INSERT INTO context (value) VALUES (NULL);
