-- Migration de teste
CREATE TABLE orders (
    id bigint PRIMARY KEY,
    user_id bigint NOT NULL,
    total numeric(10, 2)
);

-- Seguro
CREATE INDEX CONCURRENTLY idx_orders_total ON orders (total);

