CREATE TABLE entries (
    id text PRIMARY KEY,
    created_at timestamptz DEFAULT NOW(),
    updated_at timestamptz DEFAULT NOW(),
    tags text[],
    title text NOT NULL,
    description text
);

CREATE TABLE comments (
    id text PRIMARY KEY,
    entry_id text NOT NULL REFERENCES entries(id) ON DELETE CASCADE,
    created_at timestamptz DEFAULT NOW(),
    updated_at timestamptz DEFAULT NOW(),
    created_by text NOT NULL,
    text text
);
