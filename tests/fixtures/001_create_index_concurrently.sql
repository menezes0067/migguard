-- 1) Transação explícita
BEGIN;
CREATE INDEX CONCURRENTLY idx_clientes_nome ON clientes (nome);
COMMIT;
-- ERROR: CREATE INDEX CONCURRENTLY cannot run inside a transaction block

-- 2) Transação explícita com rollback
START TRANSACTION;
CREATE INDEX CONCURRENTLY idx_clientes_email ON clientes (email);
ROLLBACK;
-- Mesmo erro (a checagem ocorre no CREATE, não no COMMIT)

-- 3) Transação implícita: múltiplos statements numa ÚNICA string
--    (protocolo simple query executa tudo como uma transação só!)
CREATE TABLE clientes (id int, nome text);
CREATE INDEX CONCURRENTLY idx_clientes_nome ON clientes (nome);
-- Se enviados num único PQexec/simple_query/batch_execute:
-- ERROR: CREATE INDEX CONCURRENTLY cannot run inside a transaction block
-- (o caso mais traiçoeiro — statement por statement funciona!)

-- 4) Dentro de DO block
BEGIN
    CREATE INDEX CONCURRENTLY idx_clientes_nome ON clientes (nome);
END $$;
-- ERROR: ... cannot run inside a transaction block
--        (ou "cannot run from a function", conforme o contexto)

-- 5) Dentro de função
CREATE FUNCTION f_cria_idx() RETURNS void AS $$ BEGIN
    CREATE INDEX CONCURRENTLY idx_clientes_email ON clientes (email);
END $$ LANGUAGE plpgsql;
-- ERRO ao chamar a função: ... cannot run from a function

-- 6) Tabela particionada (não suportado, em nenhuma versão até hoje)
CREATE INDEX CONCURRENTLY idx_pedidos_criado ON pedidos (criado_em);
-- ERROR: cannot create index on partitioned table "pedidos" concurrently

-- 7) Idem para os primos: mesma regra de transação
DROP INDEX CONCURRENTLY idx_clientes_nome;      -- não pode em transação