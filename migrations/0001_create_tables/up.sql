-- Tabla para información básica del proceso
CREATE TABLE processes (
    pid INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    state TEXT NOT NULL,
    memory_mb REAL,
    start_time INTEGER,
    parent_pid INTEGER
);

-- Tabla para logs de muestreo (sesiones)
CREATE TABLE log_sessions (
    id TEXT PRIMARY KEY,
    process_pid INTEGER NOT NULL,
    started_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    duration_secs INTEGER,
    iterations INTEGER,
    FOREIGN KEY (process_pid) REFERENCES processes(pid)
);

-- Tabla para cada muestra puntual tomada en una sesión
CREATE TABLE samples (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    log_id TEXT NOT NULL,
    timestamp REAL NOT NULL,       -- tiempo relativo (segundos)
    cpu_usage REAL NOT NULL,
    memory INTEGER NOT NULL,
    FOREIGN KEY (log_id) REFERENCES log_sessions(id)
);
