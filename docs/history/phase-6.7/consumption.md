# Entrega 1 / bloco consumo — 02/11/2026

Política v2 implementada exclusivamente no avaliador de desenvolvimento. CPU mediana <1%, p95 >2% alerta, RAM ≤200 MiB e regressão ≤15%. Reduzido conserva resultado numérico mas é informativo; coleta inválida continua erro. Monitor estrito não mudou. [Decisão do titular](../../performance.md) e duas reavaliações versionadas; v1 e todas as séries/base intactas.

Testes: `cargo test -p ametista-metrics --locked --offline`, clippy -D warnings e fmt passaram; 10 casos de orçamento incluem v1 preservada, limites exatos 1/2% e 200 MiB, 15%, CPU base zero válida em v2 e Reduzido inválido não aprovado. Três Playwright de performance passaram, incluindo CSS compilado.

Investigação Reduzido encerrada em duas tentativas: comparação dos snapshots já coletados e verificação de CSS/condições. Maior diferença no subprocesso WebView, sem blur residual reproduzido; alocação/cache de composição é hipótese, não causa comprovada. Sem mudanças cegas, flags ou novo ensaio de consumo. Completo aprovado pelo titular; Reduzido não bloqueia; VM não marcada.

Arquivos: avaliador/testes/exemplo budget, mensagem do harness, política vigente e cópia histórica, JSONs v2, README pt/en e descrição de PR. Plano das duas entregas em [phase-6.7-plan](../phase-6.7-plan.md). Próximo bloco: CI fixado e pacote interno de VM autossuficiente.
