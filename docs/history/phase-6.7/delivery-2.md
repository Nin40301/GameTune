# Fase 6.7 — Entrega 2

Estado: implementação e verificações automáticas concluídas na branch `design/aaa`; **aceite de consumo ainda pendente**. [PR #12 em rascunho](https://github.com/Nin40301/GameTune/pull/12), sem merge. A branch nasceu da main integrada `16a0d08`, após o merge do PR #1 autorizado pelo titular. `main` e a tag v0.6.0 (`de75a7531faa45600d78818a3391451811f723de`) foram preservadas nesta entrega. Produto permanece alfa 0.6.5.

## Resumo

Tokens/escalas e voz documentados, oito telas refinadas e navegação agrupada com segurança/status úteis. Cartões de perfil têm contagem e risco; Itens tem filtros/grupos, selos e seleção conservadora; Revisão tem valores Agora→Depois e consentimentos por item no mesmo backend. Resultado mostra falhas e Desfazer; Histórico agrupa por dia e reinstala apps pela Loja; Medições mostra intervalos p5–p95/mediana e prévia de anonimização; Jogos mostra monitor/sessão e lê ícone do arquivo cadastrado. Configurações organiza aparência, idioma, monitor, suporte e avisos legais offline.

Tema de sistema/claro/escuro, efeitos completo/reduzido/automático, texto 90–130% e movimento reduzido preservam preferências existentes. /design-system só em desenvolvimento; galeria, pseudo e mock ficam fora do JavaScript de produção. Sem biblioteca de gráficos nova, rede, flags experimentais ou promessa de FPS.

## Commits por bloco

| Commit | Bloco |
| --- | --- |
| 041759c | Plano de uma página e base integrada |
| 5bea857 | Tokens, voz, preferências e galeria de desenvolvimento |
| 6c8108c | Shell, perfis, grupos, revisão e dashboard medido |
| fec0406 | Apps, Histórico, Medições, Jogos, Configurações e ícones locais |
| e59cba5 | Pseudo, teclado, axe, reflow e regressão visual Linux |
| a5a3c27 | Conferência nativa: legenda do Orb e altura das métricas |
| 3f52e38 | Bases Linux, capturas persistentes e contagem confirmada |

O commit deste relatório registra o estado de aceite; não muda o código da aplicação. Referências geradas no CI ficam em commit próprio para revisão das imagens.

## Validação e comandos

[CI da candidata](https://github.com/Nin40301/GameTune/actions/runs/41629989145) verde em ubuntu-24.04/windows-2025: Rust Linux **130 ativos**, Windows **138 ativos + 5 administrativos ignorados**; frontend **41 componentes + 99 e2e**. Não somar testes equivalentes entre plataformas ou as quatro gerações de baseline aos 99 cenários da suíte completa. [Contagem extraída dos logs](delivery-2-test-count.json). A execução normal após commitar referências é acompanhada no PR; não atualiza snapshots automaticamente.

```powershell
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
cargo test --workspace --all-features --locked --offline
npm --prefix apps/desktop run format:check
npm --prefix apps/desktop run lint
npm --prefix apps/desktop test
npm --prefix apps/desktop run build
npm --prefix apps/desktop run test:e2e
node tools/check-i18n.mjs
node tools/sync-ui-licenses.mjs
node tools/check-doc-links.mjs
node tools/audit-unsafe.mjs
node tools/verify-production-ui.mjs
pwsh -NoProfile -File tools/build-release.ps1
pwsh -NoProfile -File tools/verify-desktop.ps1
cargo run --release -p ametista-windows-runtime --example icon_probe --locked --offline
```

Não rodar testes administrativos no host. Somente o harness de VM tem opt-in para isso. verify-desktop confirma UI/monitor asInvoker, auxiliar requireAdministrator, hash embutido, dez comandos tipados e CSP restritiva. O probe extraiu a gema do nosso próprio executável em PNG 32×32 sem abrir processo.

## Capturas e inspeção

[Índice persistente de todas as telas/estados](../../design/screenshots.md), [hashes das 36 referências Linux](delivery-2-visual-manifest.json), [processo de regressão](../../design/visual-regression.md). Há 32 capturas completas das oito telas, 36 de estados aplicáveis e quatro da galeria, claro/escuro e 1440/390. A matriz AA também cobre en e 1100/768. Evidência local completa, com 136 PNGs, em `outputs/phase-6.7/delivery-2/gallery.html` e subpastas screenshots/states; painéis de inspeção em inspection. README recebeu imagens atuais. Pacote local offline: `outputs/phase-6.7/delivery-2-evidence.zip` (148 arquivos; SHA-256 `bf7c2dc221a72a8b430bda3d019addc26224a2139e317bd1cc5093b25bc0eaa8`). Não contém binários, aceite administrativo nem medição temporizada.

Inspecionados os 72 alvos solicitados por painéis de contato e ampliações críticas, em duas rodadas de acabamento complementadas pela janela nativa. Corrigidos: menu Mais encoberto pela barra fixa, cards Appx altos demais, palavra de resultado dividida, estado de monitor repetido, posição de Simular, definição HTML do Agora→Depois, legenda do Orb e vazamento da nota de CPU no tamanho nativo. A legenda do Orb pode ocupar duas linhas no tamanho nativo. Itens/Configurações continuam longos em 390 px; grupos/filtros ajudam, mas não eliminam a rolagem. Capturas de documento inteiro conservam barras fixas na posição de viewport; testes de fim de lista conferem margem real para a última linha.

## Consumo antes/depois — parcial

| Modo | Referência final 0.6.5 preservada | Nova UI |
| --- | --- | --- |
| Completo | 163,54 MiB / CPU mediana 0,3932% | **156,93 MiB / CPU mediana 0,00%**; −5,01% RAM; aprovado, alerta p95 3,00% |
| Reduzido | 244,04 MiB / CPU mediana 0,3886% | Interrompido após 3 de 5; sem avaliação |
| Monitor, picos | 2,75 MiB / CPU 0,1305% | Pendente de ensaio separado |

Completo mediu cinco execuções válidas sem depuração e passou a política v2; o p95 máximo de 3,00% é alerta informativo. Reduzido foi interrompido após três execuções quando Roblox reapareceu; nenhuma dessas três é tratada como coorte aprovada. Monitor ainda não executado. Séries Completo anonimizadas, hashes e avaliação estão em [perf-v2](perf-v2/full-manifest.json); estado de todos os ensaios em [delivery-2-perf-status.json](delivery-2-perf-status.json). Baseline, limites e harness permanecem intactos. O PR continua em rascunho.

Quando o computador estiver livre:

```powershell
cargo build --release -p ametista-metrics --example budget --locked
# Escolher o modo na UI, conferir tamanho/tema/DPI, fechar a UI de preflight
pwsh -NoProfile -File tools/test-resource-budget.ps1 -Mode interface -Effects full
# Preservar a avaliação Completo antes do próximo comando
pwsh -NoProfile -File tools/test-resource-budget.ps1 -Mode interface -Effects reduced
pwsh -NoProfile -File tools/test-resource-budget.ps1 -Mode game_monitor
```

## Segurança, riscos e pendências

Catálogo, core, executores existentes, IPC elevado, helper, monitor, watcher, métricas e harness não mudaram. O diff nativo se limita a `games_action::Icons` sem argumentos e ao leitor Shell/GDI de recurso local cadastrado. Não há caminho livre da UI, execução do arquivo, handle em jogo, injeção ou hook. PNG 0.18.1 já era transitivo e agora é direto só no Windows. [Fronteira de ícones](../../security/icon-resources.md); [inventário completo de 94 linhas unsafe](../../security/unsafe-inventory.md), duas novas com RAII e comentários SAFETY. unwrap/expect continuam proibidos.

Não validados: coorte Reduzido v2 completa e monitor, roundtrip/UAC/Appx/reinício/Minecraft na matriz VM, NVDA/Narrador, DPI 125/150/200%, zoom físico e outras GPUs/versões WebView. O reflow automatizado é geometria equivalente, não prova de zoom físico. [Checklist manual](../../dev/accessibility-checklist.md) mantém resultado Pendente. Não declaramos conformidade WCAG AAA.

Experimentos 3.8 e refinamentos opcionais de ícones ficam no [backlog](../../backlog.md). Próximo passo obrigatório: repetir Reduzido em cinco execuções válidas, medir o monitor e registrar o resultado; depois revisar o PR, sem merge automático. Fase 7 continua dependente do aceite de VM e dos riscos já documentados.
