# Fase 6.7 — Entrega 1

## Resumo e commits

Branch `design/liquid-glass`; produto continua alfa 0.6.5. Planos de uma página para [as duas entregas](../phase-6.7-plan.md). A Entrega 2 não foi iniciada: exige merge desta branch pelo titular e criação de design/aaa a partir da main integrada. Sem merge/tag automático e sem ajustes reais no host.

| Bloco | Commit | Resultado |
| --- | --- | --- |
| 1 Consumo | `e51ec7d` | Política v2, v1/base/séries preservadas, decisão do titular e investigação limitada |
| 2 CI/VM | `13bace0` | ubuntu-24.04/windows-2025, cinco testes pré-compilados/CLI e runner retomável protegido |
| 3 Defeitos visuais | `5022220` | Doze correções, testes por defeito, 24 capturas revisadas |
| 4 Fechamento | Este commit | Relatório e checklist para PR; pacote final identificado abaixo |

Veja os relatos [consumo](consumption.md), [VM](vm-kit.md) e [UI](ui-fixes.md) para arquivos e limites. `git diff b066483..HEAD` mostra só esta entrega; o PR contra main também inclui a implementação anterior 6.5 ainda não integrada.

## Consumo — decisão de 02/11/2026

Nova política **v2**: CPU mediana <1%, alerta p95 >2%; RAM mediana ≤200 MiB e regressão de RAM ≤15%. Reduzido informativo, sem bloquear; monitor mantém picos ≤15 MiB e CPU <0,5%. Política v1 e avaliador legado continuam reproduzíveis. A base antiga não foi regravada.

| Modo | RAM mediana | CPU mediana | Decisão v2 sobre as séries existentes |
| --- | ---: | ---: | --- |
| Completo | 163,54 MiB | 0,3932% | **Aprovado pelo titular**, RAM −1,01% da base |
| Reduzido | 244,04 MiB | 0,3886% | Informativo: +18,75% RAM, teto excedido em 44,04 MiB e alerta p95 máximo 2,3438% |
| Monitor (picos) | 2,75 MiB | 0,1305% | Aprovado no limite estrito |

São reavaliações, **não novas medições nativas do polimento**. [Política/resultados v2](../../performance.md). A Entrega 2 pede nova coorte após redesenho. Investigação Reduzido encerrada em duas tentativas: maior diferença localizada em subprocesso WebView nos snapshots; CSS compilado e conferência nativa mostram zero blur no Reduzido. Alocação/cache de composição é hipótese provável, não causa demonstrada. Não foram aplicadas flags ou correções às cegas.

## Pacote de VM

Arquivo local: `outputs/phase-6.7-vm-acceptance-final.zip` (8.307.271 bytes). Construído do commit de código `50222202a57b94ecde7a8d433db1badf9aee2df8`.

SHA-256: `5a41c2ca2c0e6a300ecc27ebb6a97605169198cb60a5ec3f77e056d9663be554`.

Cinco executáveis x64: vm-tests, vm-unit-tests, real-profile-tests, real-profile-runner e CLI de produção. CRT estática; inspeção PE sem dependência de redistribuível MSVC. Nove hashes internos conferidos e 2+2+1 testes enumerados por --list, sem executar os testes. A VM precisa só de Windows/PowerShell 5.1/.NET locais; não Rust/Git/Visual Studio. CLM/WDAC/política podem bloquear, sem bypass.

[Guia de build/pacote](../../dev/vm-acceptance-kit.md) e [README embarcado](../../../tools/vm-acceptance/README.md). Runner exige VM/admin/dois opt-ins/snapshot declarado, protege cópia, grava em results, pede dois reinícios e retoma pela mesma conta/VM com tarefa temporária. Guarda estado antes de executar; crash/falha não reaplica cegamente. Recovery é explícito e aborta o aceite. Tudo isso continua sujeito a ensaio real na VM: compilar não valida reinício/ACL/UAC. Não é release; CI guarda somente artefato temporário rotulado INTERNAL-VM-ACCEPTANCE-NOT-A-RELEASE.

## Testes e comandos

Local: **136 Rust passaram, zero falharam, cinco VM ignorados**; **33 componentes em oito arquivos**. 487 chaves pt-BR/en iguais e 32 JSX sem prosa fixa. E2E tem 74 casos únicos: primeira rodada 72 passaram/duas falhas de seletores em inglês; corrigidos e 23 afetados passaram, incluindo quatro capturas/axe e oito regressões dos doze defeitos. Oito regressões foram repetidas com capturas de fim da lista. Reruns não são somados. CI completa novamente os 74 nas imagens fixadas.

Fmt, clippy -D warnings, ESLint, TS strict/build, versão, links, catálogo gerado, inventário unsafe e verify-desktop passaram. Manifestos preservados: UI/monitor asInvoker, helper requireAdministrator, dez permissões tipadas, CSP sem rede, sem mocks/flags no build de produção. Host recusado pela guarda de VM em CheckOnly antes de escrever; parser Windows PowerShell 5.1 aprovado sem executar o script. A política local também recusou execução de script e não foi contornada.

```powershell
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
cargo test --workspace --all-features --locked --offline
npm run format:check --prefix apps/desktop
npm run lint --prefix apps/desktop
npm test --prefix apps/desktop
npm run build --prefix apps/desktop
npm run test:e2e --prefix apps/desktop
node tools/check-i18n.mjs
node tools/check-doc-links.mjs
node tools/generate-catalog-docs.mjs --check
node tools/audit-unsafe.mjs
pwsh -NoProfile -File tools/build-release.ps1 -SkipFrontend
pwsh -NoProfile -File tools/verify-desktop.ps1
# Em desenvolvimento: cria um NOVO destino, nunca sobrescreve evidências
pwsh -NoProfile -File tools/build-vm-acceptance.ps1 -Destination outputs/vm-acceptance-new
node tools/verify-vm-package.mjs outputs/vm-acceptance-new
```

O status remoto verificável fica nos [Checks da branch](https://github.com/Nin40301/GameTune/actions/workflows/ci.yml?query=branch%3Adesign%2Fliquid-glass), associado ao SHA final. Não atribuir os checks antigos de imagens latest à nova revisão. [Descrição/checklist de PR](../../dev/pr-0.6.7.md).

## Capturas e o que ainda está fraco

[Manifesto de 24 capturas](../../assets/screenshots/6.7/manifest.json): Início, Itens, Revisão e Jogos × dois temas × 1440/390, páginas inteiras; Itens tem mais oito viewports de topo/fim. Cada imagem foi inspecionada e cada faixa das páginas longas foi vista. Dados de demonstração explicitamente identificados. Barras fixas aparecem na posição inicial nas capturas inteiras; os viewports finais comprovam a borda e última linha livre, sem retocar imagens.

Corrigidos: espaço banner/contador, faixa vazia, barra/contagem, voz, ícone, precisão do Orb, sparkline de 60 s/alturas, slogan, escala de texto, botão de coleta e controles/contexto de Jogos. Segunda rodada corrigiu ausência de gráfico na fixture que repetia timestamp e o espaço da unidade do Orb. O mock passou a avançar timestamp; a produção não recebeu valores fictícios. Ainda há densidade de selos, telas longas e hierarquia do editor de Jogos a refinar na Entrega 2. Não declarar acabamento AAA completo ou conformidade WCAG AAA.

## Riscos e pendências

- Sem mudanças em catálogo, core, regras de segurança, watcher ou IPC. Backend alterado só no avaliador de desenvolvimento e resolução fixa da CLI irmã no harness de VM, ambos pedidos explicitamente; monitor mudou somente texto localizado da bandeja
- Pacote/retomada/ACL/tarefa/reboots e apply→undo→snapshot real **não executados**; matriz Windows Home/Pro e Minecraft/Loja/Edge/Configurações/Segurança continuam pendentes na VM
- Proteção por volume permanece desconhecida quando não pode ser comprovada; nenhuma ativação automática
- NVDA/Narrador, DPI físico 125/150/200%, zoom real 200/400% e alto contraste físico **não validados**. [Checklist manual](../../dev/accessibility-checklist.md) permanece aberto
- Nenhum segredo de assinatura usado, instalador final criado, tag movida ou release publicado
- Entrega 2 depende de merge pelo titular; não criar design/aaa da branch atual. Partes 3.2–3.7 e medição nova ficam nessa entrega; 3.8 segue opcional, desligada e condicionada a evidência
