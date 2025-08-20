CREATE TABLE IF NOT EXISTS plugins (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    major INTEGER NOT NULL,
    minor INTEGER NOT NULL,
    patch INTEGER NOT NULL,
    timestamp TEXT NOT NULL DEFAULT (datetime('now'))
);

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

CREATE TABLE IF NOT EXISTS monorepo (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    plugin INTEGER,
    projects TEXT NOT NULL,
    packages TEXT NOT NULL,
    timestamp TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (plugin) REFERENCES plugins (id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    plugin INTEGER NOT NULL,
    timestamp TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (plugin) REFERENCES plugins (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS packages (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    plugin INTEGER NOT NULL,
    timestamp TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (plugin) REFERENCES plugins (id) ON DELETE CASCADE
);
