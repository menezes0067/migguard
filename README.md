# migguard

Linter que detecta migrations SQL perigosas antes de chegarem em produção.

> **Status:** em desenvolvimento. Nada disso funciona ainda, o README descreve o que está planejado.

## Por que existe

Algumas migrations parecem inofensivas, mas travam a tabela em produção. Um `CREATE INDEX` sem `CONCURRENTLY` bloqueia escritas enquanto o índice é criado, e um `DROP COLUMN` pode quebrar a versão antiga da aplicação que ainda está rodando durante o deploy. Isso costuma passar batido no code review.

O `migguard` lê suas migrations, aponta o que é perigoso, explica o motivo e sugere a alternativa segura. Ele foi pensado pra rodar localmente e no CI, falhando o pipeline antes que o problema chegue em produção.

## Uso (planejado)

```bash
migguard ./migrations
```

Saída esperada:

```
migrations/002_add_index.sql:4: erro [create-index-concurrently]
    CREATE INDEX sem CONCURRENTLY bloqueia escritas na tabela enquanto o índice é criado
    dica: use CREATE INDEX CONCURRENTLY

1 arquivo(s) analisado(s): 1 erro(s), 0 aviso(s)
```

O comando sai com código diferente de zero quando encontra erros, então funciona direto em pipelines de CI.

## Regras

| Regra | O que detecta | Status |
|-------|---------------|--------|
| `create-index-concurrently` | `CREATE INDEX` sem `CONCURRENTLY` | planejada |
| `add-column-not-null` | `ADD COLUMN ... NOT NULL` sem `DEFAULT` | planejada |
| `drop-column` | `DROP COLUMN` | planejada |
| `drop-table` | `DROP TABLE` | planejada |
| `rename` | `RENAME COLUMN` / `RENAME TABLE` | planejada |
| `alter-column-type` | `ALTER COLUMN ... TYPE` | planejada |

## Roadmap

- [ ] CLI que lê arquivos `.sql` de um arquivo ou diretório
- [ ] Primeira regra: `create-index-concurrently`
- [ ] Demais regras da tabela acima
- [ ] Testes com migrations boas e ruins para cada regra
- [ ] CI com `cargo fmt`, `cargo clippy` e `cargo test`
- [ ] Parser real (`sqlparser`) no lugar da análise por texto
- [ ] Saída em JSON (`--format json`) e colorida
- [ ] Arquivo de configuração para desligar regras
- [ ] Exemplo de GitHub Action no README

## Limitações conhecidas

- Foco em PostgreSQL por enquanto.

## Desenvolvimento

```bash
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt
```

## Licença

MIT