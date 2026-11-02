# Fase 4 — entrega e validação — 2026-11-01

## Resultado

UI Tauri 2 + React + TypeScript strict + Tailwind sobre o backend existente:
Início, Itens, Revisão, Limpeza de apps, Histórico e Configurações. pt-BR/en,
dark/light, pesquisa/filtro, teclado, leitor de tela por semântica/status, diálogo
de detalhes com foco contido/restaurado e consentimentos por item/app.

Simulação é o padrão. Aplicação, undo, remoção e abertura da Store passam por
revisão. Tickets de uso único vinculam operação/IDs e expiram em 120 s. UI
asInvoker; helper requireAdministrator, uma transação por processo, pipe local
com DACL por SID, PID/SID reais, nonce, hash de catálogo, limites e anti-replay.
O helper revalida o domínio e recaptura estados pelo WindowsSession.

Análise em background com progresso/cancelamento, até quatro leitores e um
inventário Appx por análise. Cancelar encerra somente os próprios filhos
PowerShell de leitura, sem localizar/encerrar processos de terceiros. Evidência
cancelada/expirada ou de operação incerta não autoriza nova aplicação.

Uma execução local não elevada consultou **88 itens, sem falhas, em 5,41 s**,
comparada à amostra de aproximadamente 33 s da fase anterior. ProgramData/Ametista
continuou ausente. É uma amostra de latência, não medição de FPS ou garantia em
outras máquinas. Cache é apenas de leitura; toda escrita recaptura o original.

## Validação executada

| Suíte/check | Resultado local Windows |
| --- | ---: |
| acceptance/foundation | 27 passaram |
| acceptance/manual_apps | 6 passaram |
| acceptance/safety_boundaries | 7 passaram |
| acceptance/scopes_and_sources | 2 passaram |
| CLI | 9 passaram |
| core/unit + core/read_only | 5 + 4 passaram |
| desktop_backend | 5 passaram |
| IPC/protocolo | 6 passaram |
| IPC/pipes Windows reais, sem elevação | 3 passaram |
| windows_runtime, leitura/parser/cancelamento | 5 passaram |
| **Rust normal** | **79 passaram, 2 ignorados** |
| Componentes Vitest/Testing Library | **6 passaram** |
| Playwright, backend mockado | **10 passaram** |
| axe AA | **25 auditorias sem violações** |
| fmt, Clippy all-targets/all-features, ESLint, TS strict, Prettier | Passaram |
| Build UI/helper release com features padrão | Passou |
| Manifestos extraídos dos PE, seis capabilities, CSP e ausência de mocks | Verificados |

As auditorias axe cobrem seis telas × dois idiomas × dois temas, mais diálogo
de detalhes. E2E cobre simular/revisar/aplicar/desfazer via mock, consentimento
múltiplo/reforçado, cancelamento, UAC cancelado simulado, foco/teclado, idioma/tema,
reinstalação separada e larguras 1440/768/390 sem overflow ou requests HTTP externos.
Screenshots de demonstração foram inspecionados; não representam escrita real.

Os testes reais de pipe verificam ACL negando outro SID, instância exclusiva,
SID/PID do peer, replay e frame malformado/grande. Os de protocolo acrescentam
IDs desconhecidos, campos livres, nonce/hash divergentes e consentimentos ausentes.
Não lançam helper/UAC nem executam alterações de configuração.
Uma reaplicação também recaptura alvos com journal Applied: divergência externa
é reportada como conflito, sem sobrescrever estado ou perder o primeiro original.

## Ocorrência de processo e correção

Nas verificações iniciais executei a suíte herdada que ainda incluía **duas
rotinas de integração de HKCU fora do opt-in de VM**. Elas escreveram somente em
HKCU/Software/AmetistaTest e passaram com limpeza, mas isso contrariou a restrição
desta fase. Nenhum alvo real do catálogo foi aplicado por essas rotinas.

Ambas agora têm ignore e exigem feature vm-tests mais AMETISTA_DISPOSABLE_VM.
O workflow de VM as executa explicitamente. A suíte normal final tem dois testes
ignorados; a ausência da chave AmetistaTest foi confirmada somente para leitura.
O ocorrido não foi apresentado como ausência absoluta de escritas no host.

## Arquivos criados/alterados

- apps/desktop: package/lockfile, React, modelos, i18n, temas, bridge, backend de
  teste isolado, componentes, e2e/axe, configuração Tauri/capabilities e manifesto
- crates/desktop_backend: worker, evidência, progresso/cancelamento e revisão
- crates/ipc: protocolo, validação independente, pipe/FFI e testes de abuso
- crates/helper: binário de uso único e manifesto requireAdministrator
- windows_runtime: scan paralelo/inventário único, cancelamento controlado,
  leitura de histórico e proteção dos testes HKCU por opt-in
- core/catalog: validação compartilhada, metadados de leitura e hash de catálogo
- tools/verify-desktop.ps1: verifica PE/CSP/capabilities e ausência de mocks
- docs/desktop*.md, arquitetura, segurança, README, CHANGELOG e CI

## Comandos

```powershell
cd apps/desktop
npm ci --no-audit --no-fund
npm run format:check
npm run lint
npm run build
npm test
npx playwright install chromium
npm run test:e2e
cd ../..
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
cargo test --workspace --locked --offline
cargo build -p ametista-desktop -p ametista-helper --release --locked --offline
./tools/verify-desktop.ps1
```

Dependências e motivos em desktop-dependencies.md. CI Linux/Windows e job web
configurados; a execução local foi Windows, não uma execução remota do Actions.

## Riscos e próxima fase

- UAC real com escrita/undo, outra conta administrativa, Store/Appx e matriz
  Windows 10/11 Home/Pro ainda precisam do roteiro desktop-vm.md. Não foram
  executados no host. O aceite administrativo continua pendente.
- Leitura do histórico protegido pode exigir UAC explícito, sem abrir a ACL do
  WAL. Análise/validação dos alvos e janela normal continuam sem admin.
- Axe não substitui revisão manual com NVDA/Narrador, alto contraste e escala.
- Não há assinatura/instalador; os binários portáteis devem vir do mesmo pacote.
  DACL/nonce/PID não defendem contra administrador malicioso ou controle total
  dos binários/da mesma conta; veja desktop-security.md.
- Appx não é restaurado por undo. Store pode exigir rede, não recupera dados nem
  garante versão/provisionamento. Resultados parciais permanecem no histórico.
- RAM/CPU idle, benchmark, métricas, watcher e logs com rotação ainda não foram
  implementados/validados nesta fase.

Próxima fase: **Fase 5 — métricas e benchmark**, dashboard medido, antes/depois e
exportação JSON/HTML, sem promessas de ganho de FPS.
