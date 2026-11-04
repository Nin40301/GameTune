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


## Entrega 2 — coorte v2 parcial (04/11/2026)

A medição temporizada de **Completo** terminou em processo de produção, sem depuração, com cinco execuções de 60 amostras após 30 s de aquecimento. Home ficou visível em 1100×740, pt-BR/claro, DPI96/DPR1; WebView2 156.0.4314.58, três superfícies com blur. Host: Windows 11 Pro 25H2, build 26200.9711, Ryzen 5 8600G, Radeon 760M/driver 32.0.21030.2001, 12 processadores lógicos e 15482.7 MiB de RAM física.

| Modo | Execuções válidas | RAM mediana entre execuções | Variação sobre base | CPU mediana entre execuções | Maior p95 por execução | Resultado |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| Completo | 5 × 60 s | **156.93 MiB** (mín 136.98; máx 211.80) | **−5.01%** | **0.00%** (máx de medianas 1.30%) | **3.00%** | **Passa**; alerta p95 |
| Reduzido | 3 de 5, interrompidas | — | — | — | — | Sem avaliação; não conta como coorte |
| Monitor | não executado | — | — | — | — | Pendente; limite estrito preservado |

Completo passa pelos limites de RAM (≤200 MiB e até 15% acima da base de 165.207 MiB) e CPU mediana (<1%). O p95 máximo ultrapassou 2%, então fica registrado como alerta, sem reprovar. A base original e o harness não foram alterados. A mediana de CPU é quantizada a 0% em parte das execuções; seguimos o critério da política v2 aprovado pelo titular.

Reduzido foi conferido antes de iniciar: zero superfícies com blur e as mesmas dimensões, tema, idioma, DPI e WebView. Três execuções foram coletadas; durante a terceira, uma janela/processo do Roblox reapareceu e o harness também atingiu seu limite ao fechar normalmente a janela do Ametista. O harness não terminou processo algum. As três amostras parciais foram preservadas apenas no diretório local `work/resource-budget-phase-6` e não foram avaliadas, publicadas ou misturadas a outra série. Repetir as cinco execuções de Reduzido do início quando o computador permanecer sem jogos durante toda a coleta. O monitor permanece pendente pelo mesmo motivo.

Séries públicas anonimizadas, seus hashes e avaliação: [manifesto da coorte Completo](history/phase-6.7/perf-v2/full-manifest.json) e [avaliação da política v2](history/phase-6.7/perf-v2/full-evaluation-v2.json). A lista de arquivos-fonte originais permanece no diretório local de trabalho; os hashes no manifesto permitem conferir que os dados publicados correspondem às amostras. `perf-baseline.json` continua inalterado.

**Nota de procedimento:** ao diagnosticar por que a janela de medição não fechou, uma consulta pontual a `Get-Process` leu estado/CPU do processo de jogo, o que abre um handle. Isso excedeu a regra do projeto; não alterou nem encerrou o jogo e não será repetido. Nenhum binário do produto abriu handle de jogo.

Próximos ensaios: fechar o jogo, repetir Reduzido com cinco séries novas e executar o gate estrito do monitor (≤15 MiB e CPU <0.5%). Não aceitar a Entrega 2 até completar esses gates e as validações manuais de VM/acessibilidade.
