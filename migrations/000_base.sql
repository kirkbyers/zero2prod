CREATE TABLE IF NOT EXISTS subscriptions (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS scrapes (
    id TEXT PRIMARY KEY,
    batch_id INTEGER,
    url TEXT NOT NULL,
    original TEXT,
    content TEXT,
    scraped_at TEXT NOT NULL,
    region TEXT,
    processing TEXT,
    drying TEXT,
    arrival TEXT,
    lot_size TEXT,
    bag_size TEXT,
    packaging TEXT,
    farm_gate TEXT,
    cultivar_detail TEXT,
    grade TEXT,
    appearance TEXT,
    roast_rec TEXT,
    coffee_type TEXT,
    spro_rec TEXT,
    score REAL
);

CREATE TABLE IF NOT EXISTS scrape_embeddings (
  id TEXT PRIMARY KEY,
  scrape_id TEXT,
  embedding BLOB
);

CREATE TABLE IF NOT EXISTS jobs (
    id TEXT PRIMARY KEY,
    job_type INTEGER NOT NULL,
    job_status INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    completed_at TEXT
);