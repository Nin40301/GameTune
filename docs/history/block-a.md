# Bloco A — linha de base e política de consumo

## Resumo

Cinco execuções reais por modo, sem descartar nenhuma, com 30 s de aquecimento e 60 amostras por execução. Linha de base em `../perf-baseline.json`; dados públicos anonimizados em `perf-baseline-0.6.5/`. Protocolo e decisão em `../performance.md`. O monitor foi medido separadamente em modo somente leitura.

O titular confirmou bloqueio de regressão >15% para medianas de RAM e CPU. CPU mediana zero permanece zero: qualquer mediana positiva regride, sem piso. Pico de CPU não veta a UI; p95 ≥1% é alerta. O teto mediano de RAM de 200 MiB permanece, mesmo quando a linha de base o excede. O monitor conserva picos ≤15 MiB e CPU <0,5%.

Completo: mediana de RAM 165,21 MiB; passa. Reduzido: 205,50 MiB; falha por 5,50 MiB acima do teto. CPU mediana 0% em todas as execuções; p95 abaixo de 1% nos dois modos. Monitor: 2,47 MiB e 0,1299%, aprovado. Completo é o padrão para novas preferências; opções salvas e preferências de acessibilidade são preservadas.

## Arquivos

- `crates/metrics/src/resource_budget.rs`, testes e exemplo `budget`: estatística única, candidato explícito de baseline, avaliação da coorte e monitor estrito
- `tools/test-resource-budget.ps1`: cinco execuções da UI, arquivos únicos, avaliação obrigatória e recusa de dados ausentes/obsoletos
- `docs/perf-baseline.json`, `docs/performance.md`, plano e evidências públicas
- Preferências/CSS e testes da UI: padrão completo e transparência reduzida, incluindo CSS compilado
- Documentação vigente atualizada; critérios antigos identificados como históricos

## Comandos e resultado

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
cargo test --workspace --all-features --locked --offline
npm run format:check --prefix apps/desktop
npm run lint --prefix apps/desktop
npm run test --prefix apps/desktop
npm run build --prefix apps/desktop
npm run test:e2e --prefix apps/desktop
pwsh -NoProfile -File tools/verify-desktop.ps1
```

128 testes Rust aprovados, zero falhas, cinco exclusivos de VM ignorados. 24 componentes e 48 Playwright aprovados. Fmt, clippy -D warnings, ESLint, TypeScript/build e verify-desktop passaram. Os limites e a avaliação têm testes sintéticos para extremos, fronteira de 15%, CPU zero, p95 apenas alerta, coleta inválida, coorte incompleta e baseline inconsistente.

As coletas usaram o executável conhecido do commit 720582e e o contador original, cujos hashes estão no baseline; o novo avaliador Rust foi executado sobre as dez coletas. A mudança de preferência não é apresentada como um novo ensaio de binário. O novo harness teve sintaxe conferida e seus componentes de coleta/avaliação exercitados separadamente; não foi feita uma segunda coorte redundante só para repetir a orquestração.

## Riscos e pendências

Reduzido permanece fora do teto neste host; o resultado não foi convertido em aprovação. Mediana de CPU zero deixa o gate estrito contra qualquer aumento futuro, conforme a escolha do titular. Outros hosts precisam de avaliação própria; não há descarte por host ocupado ou substituição automática da referência. NVDA/Narrador, DPI físico e aceite administrativo na matriz de VM continuam pendentes. Nenhum ajuste real foi aplicado no host. A investigação de GPU/WebView foi encerrada; a próxima entrega é o Bloco B (identidade, versão e processo de release).
