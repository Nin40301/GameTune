# Fase 5 — métricas e benchmark

Implementação funcional entregue. O aceite do orçamento de consumo é separado:
o gate mede o app inteiro, incluindo WebView2, e **reprova** violações. Resultados
e contagens exatas ficam em `test-count-phase-5.json`; não se deve interpretar
testes funcionais verdes como prova de que o limite de consumo foi atingido.

## Entregue

- Dashboard real: RAM disponível/total, CPU por delta, processos e top 12 agrupado
- Sampler compartilhado, somente leitura, 2 s no painel/1 s durante uma rodada;
  nenhuma abertura de processos de jogos, injeção, hook, driver ou working-set trim
- Fluxo 5 × 60 s por condição, aquecimento descartado, condições confirmadas a
  cada rodada, progresso/cancelamento e estado persistente entre reinícios
- Detecção conservadora de reboot por timestamp de boot **e uptime menor** que
  na aplicação; relógio alterado sozinho não conta. Se esperar demais, reinicie
  novamente e comece a primeira rodada Depois antes de atingir o uptime anterior
- Mesmo GUID de plano, rejeição de lacunas/suspensão/mudanças e bloqueio de
  ajustes durante coleta; confirmação de intervenção só pela resposta do helper
- Medianas de rodadas, p5/p95, MAD, variação explícita e regra de inconclusivo;
  sem escore de ganho, promessa causal ou conversão de RAM em FPS
- PresentMon opcional por CSV v1: valida instalação escolhida, não executa,
  baixa ou empacota PresentMon; FPS/1% low/frametime só da captura importada
- JSON e HTML offline, CSP sem recursos remotos, escaping e anonimização padrão
- Seletores nativos e 2 comandos tipados adicionais, sem fs/shell geral na UI
- Testes estatísticos, crash/reboot simulado, roundtrip de floats, payloads
  inválidos, componentes e Playwright/axe, com mocks excluídos do build normal

## Arquivos

- `crates/metrics/`: coletor, FFI isolada, snapshot Windows somente leitura,
  agregação/árvore própria, teste nativo e exemplos snapshot/budget
- `crates/benchmark/`: protocolo, estatística, persistência, serviço em background,
  PresentMon, exportadores, locales e testes portáveis
- `apps/desktop/src/Measurements.tsx`, contratos/backend/i18n/estilos/navegação,
  testes de componentes e `e2e/benchmark.spec.ts`
- `apps/desktop/src-tauri/`: bridge tipada, seletores e integração com revisão
- `crates/desktop_backend/src/lib.rs`: extrai apenas resultados reais aplicados
- `tools/test-resource-budget.ps1`, verificação de 8 permissões e workflow
  manual `resource-budget.yml` para desktop interativo sem administrador
- `docs/benchmark.md`, `benchmark-vm.md`, plano, arquitetura e dependências

## Verificar

```powershell
cd apps/desktop
npm ci --no-audit --no-fund
npm run format:check
npm run lint
npm run build
npm test
npm run test:e2e
cd ../..
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
cargo test --workspace --locked --offline
cargo build --workspace --release --locked --offline
./tools/verify-desktop.ps1
cargo build -p gametune-metrics --example budget --release --locked --offline
./tools/test-resource-budget.ps1 -Mode idle
./tools/test-resource-budget.ps1 -Mode monitor
```

O build precisa dos assets antes do Rust/Tauri. Para instalar Chromium de testes,
use `npx playwright install chromium` em `apps/desktop`. Cargo/npm podem usar rede
para dependências de desenvolvimento; o produto executa offline. Os testes
mutáveis continuam ignorados e exclusivos do opt-in de VM. Não foram executados
UAC, aplicar/desfazer, remoção Appx ou reinício no host nesta fase.

## Consumo e limites conhecidos

O gate usa **80 MiB (83.886.080 bytes)** de RAM privada residente somada e
**máximo de CPU de 1 s <1%** da capacidade total, após 30 s de aquecimento e
60 s de coleta. Inclui todos os descendentes WebView2, não apenas o Rust.
Não reduz os limiares quando o host está ocupado nem descarta picos.

Último executável medido (hash no JSON de evidência):

| Modo | Pico de RAM privada, árvore inteira | Máximo de CPU em 1 s | Gate desta rodada |
| --- | --- | --- | --- |
| Idle | 77,55 MiB | 0,65% | Passou |
| Monitor | 79,74 MiB | 0,53% | Passou |

São 60 s por modo depois de 30 s de aquecimento, sem aplicações/desfazeres no
host. O limite foi definido explicitamente em MiB, como usual nas ferramentas
de memória do Windows; não significa 80.000.000 bytes decimais. A margem do
monitor é pequena e as falhas exploratórias abaixo impedem afirmar consistência.

Ensaios exploratórios reprovaram cerca de 131,5 MiB/1,95% antes da composição
por software. Houve execuções posteriores que passaram, mas também reprovação
em idle com aproximadamente 80,3 MiB/1,30%. Essa variabilidade é material; a
meta não está demonstrada de forma consistente. O relatório preserva a falha,
não usa só uma execução favorável e não promete o mesmo consumo em outra máquina.

A janela inicial é 1100 × 740, sem drag/drop nativo não utilizado. Composição por
software/flags afetam somente a WebView do app. SmartScreen, sandbox e isolamento
não foram desativados. A Microsoft desaconselha flags de WebView em produção;
revisar essa dependência antes da Fase 7. O novo diretório da WebView isola flags
de builds anteriores; preferências visuais antigas podem ser redefinidas uma vez.
O journal e as medições ficam separados desse perfil.

Captura PresentMon automática não foi implementada: o console oficial abre
handles para metadados de processos. A importação offline preserva a restrição
de anti-cheat; foi testada com CSV sintético, não com ferramenta instalada no host.
P5/p95 com 4 rodadas são descritivos, não intervalos de confiança. CPU em sistemas
com mais de 64 CPUs lógicas exige validação específica de grupos de processadores.

Retomada/reboot e estatística foram testados com dados sintéticos. O ciclo real
aplicar → reiniciar → 5 Depois → desfazer e a matriz Home/Pro continuam pendentes
da VM descartável. Axe não substitui validação manual com Narrador/NVDA/DPI.

## Próxima fase

Fase 6: descoberta de launchers, watcher e sessões temporárias por jogo. Antes
do aceite de desempenho, resolver/revalidar a variabilidade do custo da WebView;
continuar sem abrir handles de jogos ou afrouxar o gate.
