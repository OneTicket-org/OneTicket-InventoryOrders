CREATE TABLE holds (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    event_id BIGINT NOT NULL,
    seat TEXT NOT NULL,
    state TEXT NOT NULL CHECK (
        state IN ('ACTIVE', 'RELEASED', 'EXPIRED', 'CONVERTED')
    ),
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    customer_email TEXT,
    version BIGINT NOT NULL DEFAULT 0,
    
    FOREIGN KEY (event_id, seat) REFERENCES seats (event_id, label)
);

CREATE TABLE orders (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    event_id BIGINT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING' CHECK ( status IN ('PENDING', 'PAID', 'CANCELLED', 'REFUNDED')),
    total_cents INTEGER NOT NULL,
    payment_id TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    paid_at TIMESTAMPTZ NULL,
    cancelled_at TIMESTAMPTZ NULL,
    cancellation_reason TEXT NULL,
    refunded_at TIMESTAMPTZ NULL,
    version BIGINT NOT NULL DEFAULT 0
);

CREATE TABLE order_lines(
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    order_id BIGINT NOT NULL,
    hold_id BIGINT NOT NULL,
    seat_id BIGINT NOT NULL,
    price_cents INTEGER NOT NULL,
    
    FOREIGN KEY (order_id) REFERENCES orders(id),
    FOREIGN KEY (seat_id) REFERENCES seats(id),
    FOREIGN KEY (hold_id) REFERENCES holds(id)
);