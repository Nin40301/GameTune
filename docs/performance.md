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


## Entrega 2 — nova coorte ainda não coletada (03/11/2026)

A candidata de UI `a5a3c27` foi compilada e conferida no Windows em 1100×740, pt-BR/claro, DPR1/DPI96, WebView2 156.0.4314.58 e três superfícies com blur no Completo. Essa conferência usou depuração temporária e **não é o gate de consumo**. A janela foi fechada depois. Não há amostras temporizadas, médias, medianas ou aprovação nova a partir dessa inspeção.

A verificação somente leitura detectou um jogo em execução. A condição sem jogos ainda não foi satisfeita; a nova coorte fica pendente, inclusive o ensaio separado do monitor. Nenhum processo de terceiro foi encerrado. [Estado e hashes dos binários](history/phase-6.7/delivery-2-perf-status.json). Não usar as séries 0.6.5 da tabela acima como se fossem medições desta UI. Limites, baseline e harness não foram alterados, e o PR continua em rascunho para fechar essa evidência obrigatória.

Ao disponibilizar o computador: conferir as preferências em processo fresco, fechar o preflight, executar o mesmo harness cinco vezes por modo (ele faz as cinco execuções), preservando os relatórios e sem depuração na coleta; monitor somente leitura com limite estrito. Registrar host/WebView/GPU/DPI e anonimizar séries antes de publicar. Completo exige CPU mediana <1%, RAM ≤200 MiB e ≤15% sobre a base; p95 >2% alerta. Reduzido é informativo. Não descartar execuções por host ocupado.
