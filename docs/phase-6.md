# Fase 6 — perfis por jogo e orçamento separado

## Entregue

- UI sem flags experimentais: argumentos vazios também substituem os defaults
  experimentais do wry. SmartScreen, sandbox e GPU não são desativados
- Gates separados e fechados: UI aberta <=150 MiB; monitor Rust <=15 MiB e CPU
  <0,5%. UI preserva o limite de CPU <1%. Sem elevar limites por host ocupado
- `gametune-monitor.exe` asInvoker, Rust/Win32, sem WebView, iniciado sob demanda
  em Configurações, com bandeja. A UI pode fechar sem encerrar o monitor
- Descoberta apenas por configurações locais Steam/Epic/Battle.net/Riot/Minecraft;
  adição manual por seletor `.exe`; candidatos nunca habilitam perfis sozinhos
- Detector por caminho completo + PID/CreationDate, sessões simultâneas e múltiplas
  instâncias; sem OpenProcess de jogo, hooks, injeção, driver ou working-set trim
- Catálogo temporário com schema/hash: energia, Modo de Jogo e três serviços
  explicitamente pausáveis; pausa não altera StartMode/DelayedAutoStart
- Motor/WAL/undo existentes, journal temporário separado protegido em ProgramData,
  lease de máquina, rollback no fim/EOF/timeout e recuperação antes de nova sessão
- Pipe autenticado, frames sequenciados e somente IDs autorizados; falha/replay
  encerra a sessão. Helper elevado permanece só enquanto há ações temporárias
- Jogos, consentimento por perfil, Configurações, pt-BR/en, teclado, axe e mocks
  exclusivamente de teste. Nenhuma promessa de ganho de FPS

## Arquivos principais

- `crates/watcher/`: biblioteca, descoberta de configurações, detector, WMI local,
  persistência/lock/status e testes; exemplos de processos/eventos sintéticos
- `crates/monitor/`: executável, manifesto asInvoker, ícone/menu da bandeja,
  worker de eventos/sessão/heartbeat e textos em pt-BR/en
- `crates/core/src/temporary.rs`, extensão do trait Executor e motor/mock para
  pausa; `crates/acceptance/tests/game_sessions.rs` testa WAL e crash de filho
- `crates/ipc/src/game_protocol.rs`, `game_pipe.rs`: protocolo da sessão limitada
- `crates/windows_runtime/`: pausa SCM, WAL separado e exclusividade entre usuários
- `catalog/game-actions.json`, schema e hashes: allowlist temporária embutida
- `apps/desktop/src/Games.tsx`, bridge/backend/tipos/i18n/navegação/testes
- `tools/test-resource-budget.ps1`, `test-game-events.ps1`, `verify-desktop.ps1`
- `docs/games.md`, `games-vm.md`, arquitetura, dependências, plano e evidência JSON

## Comandos para validar

Na raiz, depois de `cargo fetch --locked` para obter dependências de build:

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
./tools/test-resource-budget.ps1 -Mode interface
./tools/test-resource-budget.ps1 -Mode game_monitor
cargo build -p gametune-watcher --example synthetic_game --example wmi_probe --release --locked --offline
./tools/test-game-events.ps1
```

Build de frontend precede Tauri; Playwright requer Chromium instalado como
dependência de teste. Não eleve os checks normais. O gate do monitor usa modo
somente leitura com WMI/bandeja reais, sem lançar auxiliar. O teste de eventos
cria somente nosso processo sintético, que encerra sozinho. Nenhum ajuste real,
UAC, remoção Appx, alteração de serviço/energia/registro ou reboot foi executado
no host. Apenas arquivos próprios do app e artefatos temporários foram gravados.

## Resultados

Contagem atual: **121 testes Rust aprovados**, **2 ignorados** exclusivos de VM,
**14 testes de componentes**, **25 Playwright**, **38 auditorias axe** sem violações.
Os 5 testes de sessão incluem uma rotina de entrada do filho simulado, sem efeito
no caminho normal; o teste pai provoca de fato saída 73 após alterar um arquivo
mock com Prepared sincronizado e valida restauração exata na reabertura.

Fmt, clippy com todos os targets/features, ESLint, TypeScript strict, build release,
manifestos/allowlist/CSP e exclusão de mocks passaram. Eventos nativos start/stop
do nosso executável foram observados com caminho completo, sem chamar executores.
Valores finais de recursos e hashes estão em `test-count-phase-6.json`.

Medição separada: 30 s de aquecimento e 60 amostras de 1 s, picos de memória
privada de toda a árvore e máximo de CPU de 1 s. Interface visível, dashboard
ativo; monitor é processo independente. Não esvazia memória nem troca o limiar.
A medição é deste host/runtime/DPI, não garantia para a matriz inteira.

Últimos executáveis medidos no Windows 11 Pro 25H2, build 26200.9711:

| Processo medido separadamente | Pico de RAM privada da árvore | Máximo de CPU de 1 s | Resultado |
| --- | --- | --- | --- |
| Interface aberta e visível | 132.86 MiB | 0.77% | Passou: <=150 MiB e <1% CPU |
| Monitor, observação somente leitura | 2.17 MiB | 0.13% | Passou: <=15 MiB e <0,5% CPU |

O monitor não contém Tauri/WebView na árvore de dependências. O segundo gate
mantém WMI e bandeja ativos; não inclui uma sessão elevada aplicando alterações.
Hashes dos quatro executáveis e contagens exatas estão no JSON de evidência.

## Riscos e pendências da VM

- O provedor WMI usa polling de 1 s; isso não garante latência de 1 s nem captura
  de processos muito curtos. Sem caminho, não aplica. Ausência de geração de um
  PID ativo/falha de fonte/fila interrompida provoca falha fechada e rollback
- Formatos dos launchers não têm contrato público estável: fixtures cobrem os
  layouts implementados. Steam/Battle.net geralmente exigem `.exe` manual;
  instalações fora do caminho padrão e Bedrock podem precisar de cadastro manual
- Jogos Java com o mesmo `javaw.exe` não são distinguíveis sem argumentos/memória;
  estes não são lidos. Caminho exato não garante autenticidade do binário do jogo
- A primeira sessão de jogos pede UAC. Se helper/PC falhar, restaurar na próxima
  execução também exige autorização; não há serviço privilegiado instalado
- O gate sem ajustes mede o **monitor observador**, não o custo da sessão elevada.
  Medir helper e filhos PowerShell, inclusive fora do parentesco devido a UAC,
  e o custo compartilhado de `WmiPrvSE` ainda é pendente em VM
- Aplicar/reverter reais, crash de monitor/helper com Windows alterado, reboot,
  outra conta no UAC, jogos protegidos, Minecraft Launcher/Bedrock e snapshots
  completos antes/depois continuam pendentes. Roteiro em `games-vm.md`
- Conflito externo na restauração mantém dívida e bloqueia novas alterações;
  não sobrescreve mudanças desconhecidas nem apaga WAL para “resolver” o erro
- Validar Narrador/NVDA, bandeja/Explorer/DPI e matriz Home/Pro Windows 10/11 em VM;
  axe complementa, não substitui esse aceite. Linux não foi executado localmente
- Binários ainda sem assinatura; instalador/assinatura ficam para a Fase 7

## Próxima fase

Fase 7: instalador/portátil final, assinatura preparada no pipeline, revisão de
segurança, documentação final e aceite da matriz em VM. Resolver as pendências
acima antes de declarar o produto pronto para uso geral.
