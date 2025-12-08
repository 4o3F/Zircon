CREATE TABLE anime (
    id INT PRIMARY KEY,
    platform TEXT,
    image_url TEXT,
    summary TEXT,
    name TEXT,
    name_cn TEXT,
    tags JSONB,
    infobox JSONB,
    rating FLOAT,
    rank INT,
    air_date DATE,
    episodes INT,
    created_at TIMESTAMP DEFAULT NOW (),
    updated_at TIMESTAMP DEFAULT NOW ()
);

CREATE TABLE users (id INT PRIMARY KEY, username TEXT, reg_date DATE);

CREATE TABLE interactions (
    user_id INT,
    anime_id INT,
    status INT,
    rate INT,
    updated_at TIMESTAMP,
    PRIMARY KEY (user_id, anime_id)
);

CREATE TABLE crawl_status (
  crawl_mode TEXT,
  crawl_offset INT
);