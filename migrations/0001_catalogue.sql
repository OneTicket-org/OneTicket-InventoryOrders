CREATE TABLE venues (
    id BIGINT PRIMARY KEY,
    name TEXT NOT NULL,
    city TEXT NOT NULL
);

CREATE TABLE events (
    id BIGINT PRIMARY KEY,
    venue_id BIGINT NOT NULL REFERENCES venues (id),
    title TEXT NOT NULL,
    starts_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE seats (
    id BIGSERIAL PRIMARY KEY,
    event_id BIGINT NOT NULL REFERENCES events (id),
    label TEXT NOT NULL,
    sector TEXT,
    status TEXT NOT NULL DEFAULT 'FREE' CHECK (status IN ('FREE', 'HELD', 'SOLD')),
    version BIGINT NOT NULL DEFAULT 0,
    price_cents INTEGER NOT NULL DEFAULT 0 CHECK (price_cents >= 0),
    
    UNIQUE (event_id, label)
);