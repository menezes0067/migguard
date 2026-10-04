-- Migration de teste
CREATE TABLE orders (
    id bigint PRIMARY KEY,
    user_id bigint NOT NULL,
    total numeric(10, 2)
);

-- Perigoso: trava escritas na tabela
CREATE INDEX idx_orders_user_id ON orders (user_id);

CREATE UNIQUE INDEX idx_orders_unique
    ON orders (id, user_id); -- comentário no fim da linha

ALTER TABLE orders ADD COLUMN status text;