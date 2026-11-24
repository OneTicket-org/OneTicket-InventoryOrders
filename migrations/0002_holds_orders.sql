CREATE SEQUENCE hold_ids START WITH 98213;

CREATE TABLE holds (
    id BIGINT PRIMARY KEY DEFAULT nextval ('hold_ids'),
    event_id BIGINT NOT NULL,
    seat TEXT NOT NULL,
    status TEXT NOT NULL CHECK (
        status IN ('ACTIVE', 'RELEASED', 'EXPIRED', 'CONVERTED')
    ),
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    customer TEXT,
    version BIGINT NOT NULL DEFAULT 0,
    
    FOREIGN KEY (event_id, seat) REFERENCES seats (event_id, label)
);

CREATE TABLE orders (
    id BIGINT PRIMARY KEY,
    event_id BIGINT NOT NULL,
    status TEXT CHECK ( status IN ('PENDING', 'PAID', 'CANCELLED', 'REFUNDED')),
    total_cents INTEGER NOT NULL,
    payment_id TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    paid_at TIMESTAMPTZ NULL,
    cancelled_at TIMESTAMPTZ NULL,
    cancellation_reason TEXT NULL,
    refunded_at TIMESTAMPTZ NULL,
    version BIGINT NOT NULL DEFAULT 0
)

CREATE TABLE order_lines(
    id BIGINT PRIMARY KEY, 
    order_id BIGINT NOT NULL,
    hold_id BIGINT NOT NULL,
    seat_id BIGINT NOT NULL,
    price_cents INTEGER NOT NULL,
    
    FOREIGN KEY (order_id) REFERENCES orders(id),
    FOREIGN KEY (seat_id) REFERENCES seats(id),
    FOREIGN KEY (hold_id) REFERENCES holds(id),
)