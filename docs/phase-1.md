# Fase 1 — Fundação entregue

Data: 31/10/2026 (America/Sao_Paulo). Repositório Git inicializado neste diretório.
Escopo: fundação completa; não inclui executores reais, UI ou instalador.

O plano de arquitetura de uma página foi entregue antes do código e está em
`docs/architecture.md`. A árvore do repositório está em `README.md`.

## Resultado

- 10 crates: domínio/motor/journal, catálogo, mocks compartilhados, seis
  adaptadores por tipo e suíte de aceitação
- 86 descritores do apêndice, schemas fechados, verificação SHA-256 e allowlist
  de alvos independente; 2 mapeamentos documentalmente verificados
- Perfis filtrados por nível, risco, edição, build, disco e reversibilidade
- Aplicar/desfazer idempotentes com preservação do primeiro estado original
- WAL sincronizado antes da ação, lock exclusivo, checksum, detecção de cauda
  interrompida e recuperação por rollback
- Falhas por item preservadas no relatório; falha parcial não perde o original
- Importação de perfis aceita apenas IDs conhecidos e não concede confirmações
- Exemplo JSON de simulação que só usa um computador fictício
- CI configurado para Linux/Windows com Actions fixadas por SHA

## Verificação realizada

Host: Windows 11 Pro x64, build 26200. Rust 1.98.1 e Cargo 1.98.1.
Não houve execução de ajustes reais nem necessidade de elevação.

| Verificação | Resultado local |
| --- | --- |
| `cargo test --workspace --locked --offline` | 37 passaram: 4 unitários do journal + 33 entre crates |
| `cargo fmt --all --check` | Passou |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | Passou sem avisos |
| `cargo build --workspace --release --locked --offline` | Passou |
| `cargo run -p gametune-catalog --example simulate --locked --offline` | JSON com dryRun true, 2 itens planejados em Windows 10 Pro fictício |

Na primeira máquina, execute `cargo fetch --locked` para obter dependências de
compilação. Depois os comandos acima funcionam com o cache e sem rede.

Os testes cobrem schema e hash, IDs/targets duplicados, proteção de recursos,
risco alto, níveis, SSD, limites de build, importação de perfis, snapshot após
dry-run, roundtrip dos cinco tipos automaticamente reversíveis, valores ausentes,
bytes originais, energia criada, idempotência, undo individual, ordem inversa,
serviço/tarefa ausentes, acesso negado, falha após alteração, retry de undo,
crash antes do commit e durante undo, checksum, lock e falha de escrita do WAL
antes da ação. Appx é testado separadamente para não alegar restauração exata.

O CI está preparado, mas não foi executado em GitHub nesta entrega. Linux não
foi executado localmente. Testes com mocks não comprovam roundtrip do Windows,
crash elétrico, compatibilidade Minecraft ou metas de consumo do aplicativo.

## Suposições e divergências resolvidas pelo padrão mais seguro

1. Remover Appx é incompatível com reversão exata sem guardar os pacotes, dados
   e contexto de instalação. Mantidos como reversão manual, fora dos perfis e
   sem aplicação pelo motor nesta fase. A Store será uma ação distinta.
2. A UI futura não usará requireAdministrator. Um auxiliar de aplicar/desfazer
   terá esse manifesto para preservar o requisito de privilégio mínimo.
3. Serviços de rede `icssvc`, `WMPNetworkSvc` e `XboxNetApiSvc` permanecem
   descritos, mas bloqueados mesmo manualmente. Prevalece a proteção absoluta.
4. `journal.wal` é a fonte de recuperação; `journal.json` é uma projeção JSON
   disponível pelo exportador. Não reescrever o array principal a cada ação.
5. Verificação documental não equivale a ganho medido. Os demais 84 itens
   continuam não verificados; perfis Home ainda não têm itens automáticos.
6. Segurança primeiro: recuperação implementada por rollback. Oferecer concluir
   um lote interrompido exige futura revisão do fluxo e confirmações.

## Riscos conhecidos e o que falta

Executores Windows retornam Unavailable. Ainda faltam DACL de ProgramData,
vinculação ao SID interativo, tratamento de HKCU sob outra conta, durabilidade
da criação do WAL no sistema de arquivos Windows, conflitos com alterações
externas, ponto de restauração, migração/compactação do journal, descoberta de
inicialização com IDs opacos, reinstalação Appx, UI e testes em VMs. Consulte
`docs/security.md` e `docs/journal.md` para os limites do contrato atual.

Nenhum ganho de desempenho foi medido. O build release contém bibliotecas;
não é um instalador nem um aplicativo pronto para uso final.

## Próxima fase — Executores Windows

Implementar APIs de registro, serviços, energia e StartupApproved; scripts
fixos/parametrizados de tarefas e Appx; ponto de restauração; identidade
interativa; armazenamento protegido e recuperação. Verificar primeiro em VM
descartável, usando alvos de teste dedicados e comparando snapshot completo
antes/após aplicar e desfazer. Não executar o catálogo de otimização completo
no runner compartilhado do GitHub. Critério de saída: roundtrip real idêntico
dos tipos reversíveis e dry-run sem alteração, com falhas registradas por item.

Fases seguintes: CLI (3), UI (4), métricas/benchmark (5), sessões por jogo (6),
instaladores/assinatura/documentação final e matriz de Windows (7).

## Dúvida pendente, sem bloquear a fundação

Qual licença o titular deseja: MIT, Apache-2.0, GPL-3.0 ou proprietária?
`LICENSE` permanece explicitamente configurável e sem concessão de distribuição.

## Arquivos criados

```text
.gitattributes
.github/workflows/ci.yml
.gitignore
CHANGELOG.md
Cargo.lock
Cargo.toml
LICENSE
README.md
catalog/items.json
catalog/profile.schema.json
catalog/schema.json
catalog/sha256.json
crates/acceptance/Cargo.toml
crates/acceptance/src/lib.rs
crates/acceptance/tests/foundation.rs
crates/acceptance/tests/safety_boundaries.rs
crates/catalog/Cargo.toml
crates/catalog/examples/simulate.rs
crates/catalog/src/lib.rs
crates/core/Cargo.toml
crates/core/src/approved_targets.json
crates/core/src/engine.rs
crates/core/src/executor.rs
crates/core/src/journal.rs
crates/core/src/lib.rs
crates/core/src/model.rs
crates/core/src/profile.rs
crates/core/src/protection.rs
crates/exec_appx/Cargo.toml
crates/exec_appx/src/lib.rs
crates/exec_mock/Cargo.toml
crates/exec_mock/src/lib.rs
crates/exec_power/Cargo.toml
crates/exec_power/src/lib.rs
crates/exec_registry/Cargo.toml
crates/exec_registry/src/lib.rs
crates/exec_scheduled_task/Cargo.toml
crates/exec_scheduled_task/src/lib.rs
crates/exec_service/Cargo.toml
crates/exec_service/src/lib.rs
crates/exec_startup/Cargo.toml
crates/exec_startup/src/lib.rs
docs/architecture.md
docs/catalog.md
docs/journal.md
docs/phase-1.md
docs/security.md
rust-toolchain.toml
tools/update-catalog-hashes.py
```
