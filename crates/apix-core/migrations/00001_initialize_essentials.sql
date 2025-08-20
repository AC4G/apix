CREATE TABLE IF NOT EXISTS events (
    id INTEGER PRIMARY KEY,
    plugin INTEGER,
    project TEXT,
    package TEXT,
    action TEXT NOT NULL,
    args TEXT,
    timestamp TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (plugin) REFERENCES plugins (id) ON DELETE CASCADE,
    UNIQUE (plugin, project, package, action, args)
);
