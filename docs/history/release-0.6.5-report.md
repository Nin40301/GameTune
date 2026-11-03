# Ametista 0.6.5 — relatório consolidado

## Estado da entrega

Implementação organizada na branch `design/liquid-glass`, versão 0.6.5, identificador `io.github.nin40301.ametista`. Simulação continua padrão, segurança/catálogo intactos e nenhuma aplicação real no host. **Aceite de consumo da interface pendente por regressão medida**, além de VM e verificações manuais. Não houve merge, nova tag, assinatura ou release publicado; v0.6.0 permanece em `de75a7531faa45600d78818a3391451811f723de`.

O CI funcional e o gate nativo de consumo são evidências diferentes. Consulte [as execuções da branch](https://github.com/Nin40301/GameTune/actions/workflows/ci.yml?query=branch%3Adesign%2Fliquid-glass) para os Checks da revisão final. A execução anterior `41544526138` aprovou frontend Linux, Rust Linux/Windows e auditorias no commit `e0f0e80`; não deve ser atribuída a commits posteriores. O push final precisa repetir esses checks. O relatório local abaixo não substitui o status vivo.

## Commits por bloco

| Bloco | Commit | Entrega |
| --- | --- | --- |
| A | `8d05477` | Dez execuções, referência versionada e política de medianas RAM/CPU; monitor estrito |
| B | `accb63e` | Versão/identifier/recursos Win32 e processo de release |
| C | `b980ecb` | README pt-BR/en, docs reorganizadas, comunidade e segurança |
| D | `7ef239d` | Ferramentas/Actions fixadas, CI, auditorias, release rascunho/SBOM/procedência |
| E | `c744435` | Auxiliar protegido, WTS/SCM, logs, diagnóstico e orientação de recuperação |
| E, compatibilidade de CI | `e0f0e80` | Compilação de suporte Windows somente no alvo Windows |
| E, revisão de caminho | `89831ee` | Recusa local/fixo antes de ACL/hash; sem UNC/device/removível |
| F | `83dadcd` | Páginas/estado, CSS, AX, onboarding/resultados, marca e gates finais |
| G | Este commit | Transparência gerada, suporte, backlog e descrição/checklist de PR |

Relatórios de arquivos/comandos/riscos por bloco: [A](block-a.md), [B](block-b.md), [C](block-c.md), [D](block-d.md), [E](block-e.md), [F](block-f.md) e [G](block-g.md). Use `git show --stat <commit>` para a lista exata de arquivos de cada bloco. Os dois commits de correção E são incrementais e não reescrevem histórico publicado.

## Consumo antes/depois

Mesmo host Windows 11 Pro 25H2 26200.9711, Radeon 760M 32.0.21030.2001, WebView2 156.0.4314.58, DPI 96/DPR 1, Início 1100×740, pt-BR/claro. Cinco execuções por modo, aquecimento de 30 s + 60 amostras de 1 s. Jogos fechados pelo titular; builds, testes e DevTools não concorreram com as amostras. Sem descarte, aumento de limite ou regravação da base.

| Modo | Referência RAM / CPU medianas | Candidata RAM / CPU medianas | Resultado |
| --- | ---: | ---: | --- |
| Completo | 165,21 MiB / 0% | 163,54 MiB / 0,3932% | Reprova CPU sobre base zero; RAM −1,01% |
| Reduzido | 205,50 MiB / 0% | 244,04 MiB / 0,3886% | Reprova RAM +18,75%, CPU e teto; faltam 44,04 MiB para 200 |
| Monitor, picos | 2,47 MiB / 0,1299% | 2,75 MiB / 0,1305% | Aprova ≤15 MiB e <0,5% |

CPU positiva contra zero bloqueia por decisão explícita do titular. Não existe percentual finito ou “margem de ruído” inventada para essa comparação. p95 final mediano: 1,3323% Completo e 1,4557% Reduzido; alerta, sem veto por pico de 1 s. Completo segue padrão novo pela RAM 80,49 MiB menor; Reduzido permanece opção de acessibilidade, sem alegar menor consumo. Nenhum dos dois passou no gate final da interface.

[Política/comparação completa](../performance.md), [baseline intacta](../perf-baseline.json), [onze séries finais e hashes](perf-final-0.6.5/manifest.json). UI medida SHA-256 `45016ae0c3ec2b94b05529117ea58aa20e15cfb7c2ad6657059852ff4898f2b3`; monitor `0ca0fa512982f8141684a2dd2e6824204e7844a62428231e1428ca71188b62fd`. Uma tentativa do monitor encerrou no aquecimento por recurso de ícone incorreto; foi preservada como falha sem amostras. A referência do ícone foi corrigida e a execução seguinte completou. A investigação antiga de GPU/WebView está encerrada; resultados históricos por pico não são o gate atual.

## Testes e comandos

Rust local: **134 passaram, zero falharam, cinco administrativos ignorados**. Componentes: **30/30, sete arquivos**. Há **66 casos e2e únicos**; seleções afetadas passaram novamente após cada correção e o CI executa todos. Não somar reruns sobrepostos. Axe AA automatizado nos dois idiomas/temas, telas principais em 1440/1100/768/390 e onboarding/resultados em 1440/390; reflow geométrico 640/320, foco/teclado/toque e preferências de movimento/transparência/contraste. TS strict/build, ESLint, fmt e clippy -D warnings verdes localmente.

```powershell
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
cargo test --workspace --all-features --locked --offline
npm ci --prefix apps/desktop --no-audit --no-fund
npm run format:check --prefix apps/desktop
npm run lint --prefix apps/desktop
npm test --prefix apps/desktop
npm run build --prefix apps/desktop
npm run test:e2e --prefix apps/desktop
node tools/verify-version.mjs
node tools/audit-unsafe.mjs
node tools/check-i18n.mjs
node tools/generate-catalog-docs.mjs --check
node tools/check-doc-links.mjs
cargo audit
npm audit --prefix apps/desktop --omit=dev --audit-level=high
pwsh -NoProfile -File tools/build-release.ps1
pwsh -NoProfile -File tools/verify-desktop.ps1
```

Gates nativos (desktop dedicado, sem jogos, preferências conferidas antes):

```powershell
cargo build --release -p ametista-metrics --example budget --locked
pwsh -NoProfile -File tools/test-resource-budget.ps1 -Mode interface -Effects full
pwsh -NoProfile -File tools/test-resource-budget.ps1 -Mode interface -Effects reduced
pwsh -NoProfile -File tools/test-resource-budget.ps1 -Mode game_monitor
```

As duas primeiras chamadas retornam falha nesta candidata; a terceira passa. Avaliar as cópias públicas com o subcomando `budget evaluate` reproduz os motivos sem medir ou alterar o host. Não chamar uma coleta incompleta/apenas coleta de aprovação.

Auditorias atuais: Cargo zero vulnerabilidades classificadas, npm produção zero. Permanecem os avisos informativos **RUSTSEC-2024-0370** (proc-macro-error sem manutenção) e **RUSTSEC-2024-0429** (glib 0.18.5 no grafo Linux/GTK do Tauri, não compilado no produto Windows). Nenhum advisory foi suprimido. Compilação não prova ausência de risco transitivo; acompanhar antes de release.

## Imagens e revisão de backend

[48 imagens com SHA-256](../assets/screenshots/0.6.5/manifest.json), em `docs/assets/screenshots/0.6.5/`: oito telas × claro/escuro × 1440/390, mais três passos iniciais e resultado. Todas inspecionadas, inclusive faixas de listas longas. Dados de teste explicitamente identificados; sem captura do computador do titular. A galeria do README foi atualizada. Ícones vetoriais/ICO em 16–256 px inspecionados; teste físico em DPI alto continua pendente.

De `720582e` até a candidata, **nenhum diff em `catalog/`, `crates/core`, `crates/catalog` ou `crates/watcher`**. Mudanças de execução ficam em E: IPC/integridade pré-UAC, runtime WTS/SCM/suporte/logs e backend de diagnóstico/bootstrap. A adiciona somente política/evidência de consumo; F toca a janela/ícone próprio do monitor. Não há novo comando livre, permissão de rede, bypass de consentimento ou relaxamento de proteção/journal.

[Inventário completo de cada linha unsafe/extern](../security/unsafe-inventory.md): IPC native 40, métricas native 10, bandeja 7, watcher fs_native 1, watcher WMI 2, runtime native 32 — **92 ao todo**, com função, justificativa e hash por arquivo. Restritos a buffers/handles Win32 próprios, SID/ACL/pipe/assinatura, snapshots globais, SCM/WTS e observação WMI. Sem handles de jogos. Lints/auditoria recusam unwrap/expect e novas fronteiras não listadas. Revisão independente continua recomendada.

## Riscos, pendências e próxima etapa

- Consumo da interface ainda reprova pela política vigente; não aprovar por o CI estar verde
- VM descartável: UAC com outra conta, instalação protegida/ACL/hash/Authenticode, SCM roundtrip, tarefas/startup/energia/Appx, crash/reboot/undo e snapshot idêntico; perfis reais e Loja/Edge/Configurações/Segurança/Minecraft no roteiro opt-in
- Proteção do Sistema por volume: a consulta atual mostra desconhecido quando não tem prova, nunca ativo por inferência. Abre ferramenta nativa sem ativar; ponto verificado continua obrigatório
- NVDA/Narrador, zoom real 200/400%, DPI físico 125/150/200%, alto contraste e troca de monitor pendentes do [checklist manual](../dev/accessibility-checklist.md); não declarar conformidade WCAG
- Portátil gravável é somente leitura; instalação protegida/assinatura final pertencem à Fase 7. Logs/journal protegidos podem ficar indisponíveis ao usuário comum; diagnóstico declara isso, não abre ACLs
- Primeiro ensaio de tag/rascunho/atestado/SBOM pelo titular pendente; ZIP local não é release. Segredos de assinatura apenas documentados, sem valores usados
- [Descrição/tópicos](../dev/github-metadata.md) e proteção de main são sugestões para o titular aplicar. [Descrição/checklist de PR](../dev/pr-0.6.5.md) prontos, sem postagem automática ou merge

[Backlog](../backlog.md): P1 separa os aceites/limites acima. P2 contém propostas não implementadas, todas `verified: false`: mouse, EcoQoS só do próprio app, USB, rede/PCIe, hibernação manual, otimizações de tela cheia por jogo e reparo SFC/DISM guiado separado, com fontes oficiais/limites de reversão. VBS/HVCI, proteção do Windows, TCP, RAM/registro, hooks/injeção/drivers nunca entram. Próxima etapa é a revisão do PR e o aceite pendente; Fase 7 exige esses resultados, sem aplicar nada no host.
