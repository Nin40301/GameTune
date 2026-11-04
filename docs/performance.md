# Consumo — política v2

**Decisão do titular em 2 de novembro de 2026 (Fase 6.7).** A mediana de CPU zero da referência é um artefato de amostragem de 1 s com tela parada; regressão relativa contra zero deixa de ser critério da interface. Não altera segurança, catálogo ou o monitor.

## Critérios vigentes

- Interface Completo: cinco execuções, 30 s de aquecimento e 60 amostras de 1 s por execução, árvore inteira, Início visível, mesmo tamanho/tema/DPI e sem jogos
- CPU: mediana das cinco medianas **<1%**. Alerta se o p95 de qualquer execução **>2%**, sem veto por pico de 1 s
- RAM: mediana **≤200 MiB** e sem piora **>15%** sobre a base preservada do mesmo modo
- Reduzido: mesmos resultados numéricos, mas **informativo e não bloqueante**, por ser opção de acessibilidade. Dados ausentes/inválidos continuam erro, nunca aprovação
- Monitor separado: **pico de RAM ≤15 MiB e pico de CPU <0,5%**, inalterado

Política v2 é explícita no JSON (`policyVersion: 2`); `passed` informa o resultado numérico e `blockingPassed` considera o caráter opcional do Reduzido. Não confundir os dois. A v1 continua implementada em `evaluate_interface_v1`/subcomando `evaluate-v1`, com seus testes. [Política v1 arquivada](history/phase-6.7/performance-policy-v1.md), [baseline original](perf-baseline.json), [séries originais](history/perf-final-0.6.5/manifest.json) e avaliações históricas não foram regravadas.

## Decisão sobre as medições finais 0.6.5

| Modo | RAM mediana | CPU mediana | p95 máximo entre execuções | Resultado v2 |
| --- | ---: | ---: | ---: | --- |
| Completo | 163,54 MiB | 0,3932% | 1,8392% | **Aprovado pelo titular**, RAM −1,01% sobre 165,21 MiB |
| Reduzido | 244,04 MiB | 0,3886% | 2,3438% | Informativo: RAM +18,75%, 44,04 MiB sobre teto; alerta p95 |
| Monitor (picos) | 2,75 MiB | 0,1305% | — | Aprovado no limite estrito |

[Avaliação Completo v2](history/phase-6.7/full-policy-v2.json) · [Reduzido v2](history/phase-6.7/reduced-policy-v2.json). São reavaliações das mesmas séries, **não novas coletas**. Host: Windows 11 Pro 25H2 26200.9711, Radeon 760M/32.0.21030.2001, WebView2 156.0.4314.58, DPI96/DPR1, Início 1100×740, pt-BR/claro. Hashes, mín/med/máx e onze séries no manifesto original. Resultados não preveem FPS nem dispensam VM.

Completo permanece padrão novo. Reduzido preserva opções salvas e preferências de transparência/acessibilidade; não promete usar menos RAM. Aprovação pelo titular fecha o critério de consumo desta referência; não aprova automaticamente todo binário futuro ou uma UI redesenhada na Entrega 2.

## Investigação limitada do Reduzido — encerrada em duas tentativas

1. **Séries existentes, sem novo ensaio:** nos snapshots finais, a mediana de memória do processo Rust foi igual nos dois modos, 4,93 MiB. O maior subprocesso WebView foi de 87,34 para 162,09 MiB (+74,75 MiB). São snapshots finais, não a mediana temporal da árvore; não atribuem função interna/GPU sem trace. Localizam a maior parte da diferença na WebView, não no coletor Rust.
2. **CSS de produção e condições:** o teste compilado verificou Reduzido com zero blur em ambos os temas, preferência de transparência reduzida e forced-colors; Completo mantém três superfícies. As conferências nativas preservadas também registram zero/três, mesmo tamanho, foco e DPI. Não há blur oculto reproduzido para corrigir nesta tentativa.

**Causa provável, não comprovada:** diferenças de alocação/cache de superfícies na composição da WebView2, dependentes de efeitos, recursos e ordem das execuções. A diferença não justifica adicionar flags de GPU ou mudar o visual às cegas. Nenhuma correção foi aplicada; investigação encerrada no limite de duas tentativas. Não confundir esta hipótese com uma medição causal de 80 MiB em um efeito específico.

## Conhecimento preservado

O bisect anterior localizou o salto em `1067244`; o processo GPU era o maior consumidor no perfil daquele ensaio. O blur compilado que permanecia no Reduzido foi corrigido antes destas séries. Orb com limiar de 1%, tabelas sob demanda, superfícies limitadas e suspensão visual fora de foco foram preservados. A investigação antiga de GPU/WebView permanece encerrada; detalhes e limitações no arquivo histórico, sem alegações obsoletas de reprovação pelo gate antigo.

## Reproduzir

```powershell
pwsh -NoProfile -File tools/build-release.ps1
cargo build --release -p ametista-metrics --example budget --locked
# Escolher modo/tema/tamanho, fechar a UI, manter jogos fechados
pwsh -NoProfile -File tools/test-resource-budget.ps1 -Mode interface -Effects full
pwsh -NoProfile -File tools/test-resource-budget.ps1 -Mode interface -Effects reduced
pwsh -NoProfile -File tools/test-resource-budget.ps1 -Mode game_monitor
```

Para reavaliar sem medir: `budget.exe evaluate docs/perf-baseline.json full <saida.json>` e os cinco arquivos `docs/history/perf-final-0.6.5/full-N.json`; use reduced e suas séries para o outro modo. `evaluate-v1` reproduz as reprovações históricas; `evaluate` aplica v2. Código 2 indica dados inválidos; não é aprovação. Não substitua automaticamente a base. O harness de coleta continua com 5×60 amostras, sem descarte por host ocupado.


## Entrega 2 — coortes v2 concluídas (04/11/2026)

Os modos Completo e Reduzido foram medidos em cinco execuções de 60 amostras após 30 s de aquecimento. Home ficou visível em 1100×740, pt-BR/claro, DPI96/DPR1; WebView2 156.0.4314.58. Completo usou três superfícies com blur; Reduzido, zero. Host: Windows 11 Pro 25H2, build 26200.9711, Ryzen 5 8600G, Radeon 760M/driver 32.0.21030.2001, 12 processadores lógicos e 15482.7 MiB de RAM física.

| Modo | Execuções válidas | RAM mediana entre execuções | Variação sobre base | CPU mediana entre execuções | Maior p95 por execução | Resultado |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| Completo | 5 × 60 s | **156.93 MiB** (mín 136.98; máx 211.80) | **−5.01%** | **0.00%** (máx das medianas 1.30%) | **3.00%** | **Passa**; alerta p95 |
| Reduzido | 5 × 60 s | **172.19 MiB** (mín 170.35; máx 180.81) | **−16.21%** | **0.00%** (máx das medianas 0.13%) | **0.65%** | Passa, informativo |
| Monitor, picos | 1 × 60 s | **2.44 MiB** | — | **0.13%** | 0.13% | **Passa**; limites estritos |

Completo passa pelos limites de RAM (≤200 MiB e até 15% acima da base de 165.207 MiB) e CPU mediana (<1%). O p95 máximo de 3.00% fica como alerta, sem reprovar. Reduzido fica cerca de 15.27 MiB acima de Completo, mas teve p95 máximo de 0.65% e passou os critérios numéricos informativos; não bloqueia. Monitor permanece abaixo de 15 MiB e 0.5% de CPU. A base original e o harness não foram alterados. A mediana de CPU é quantizada a 0% em várias execuções; seguimos a política v2 aprovada pelo titular.

A primeira tentativa de Reduzido foi interrompida após três execuções quando Roblox reapareceu e o harness não fechou normalmente a janela Ametista. Essas séries parciais não entram na tabela. Por solicitação do titular, o processo Roblox residual sem janela foi encerrado; o lote Reduzido foi então refeito integralmente do zero, sem jogos abertos. O monitor somente leitura foi medido depois, também sem jogos. Um Gerenciador de Tarefas aberto durante a preparação permaneceu como processo auxiliar; não faz parte da árvore medida. Os valores individuais completos e a avaliação reproduzida estão nos manifestos abaixo.

Séries anonimizadas, hashes e avaliações: [manifesto Completo](history/phase-6.7/perf-v2/full-manifest.json), [manifesto Reduzido](history/phase-6.7/perf-v2/reduced-manifest.json), [manifesto do monitor](history/phase-6.7/perf-v2/monitor-manifest.json). A reavaliação das séries públicas produziu as mesmas medianas e o mesmo resultado do coletor. Hash da base preservada: `c639415970e0e7bc510b6e59c92356a65e634f0461aea54379e235bda69887b7`; `perf-baseline.json` permanece inalterado.

**Nota de procedimento:** ao diagnosticar a interrupção anterior, uma consulta pontual a `Get-Process` leu estado/CPU do processo de jogo, o que abre um handle. Isso excedeu a regra do projeto; não alterou o jogo e não será repetido. Depois da instrução explícita do titular para fechar os jogos, o processo Roblox sem janela foi encerrado à força; isso poderia descartar estado não salvo. Nenhum binário do produto abriu handle de jogo.

Os três gates numéricos da política v2 estão concluídos. Permanecem separados o aceite em VM e as verificações manuais de acessibilidade/DPI; não aceitar a Entrega 2 até essas validações.
