CREATE TABLE seats (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    event_id BIGINT NOT NULL,
    label TEXT NOT NULL,
    sector TEXT,
    status TEXT NOT NULL DEFAULT 'FREE' CHECK (status IN ('FREE', 'HELD', 'SOLD')),
    version BIGINT NOT NULL DEFAULT 0,
    price_cents INTEGER NOT NULL DEFAULT 0 CHECK (price_cents >= 0),
    
    UNIQUE (event_id, label)
);