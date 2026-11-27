CREATE UNIQUE INDEX one_active_hold_per_seat
    ON holds (event_id, seat)
    WHERE state = 'ACTIVE';
