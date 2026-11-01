# Polimento da Fase 6.5

## Resumo

Alterações restritas à UI, traduções, testes e documentação. Nenhum crate, comando Tauri, manifesto, executor, confirmação do backend ou política de segurança foi alterado. Nenhum ajuste do Windows foi aplicado no host.

- Títulos: tracking de h1 passou de −0,035 em para −0,01 em; h2 de −0,02 em para zero. Texto corrido usa espaçamento normal e largura de até 62 ch. `font-optical-sizing: auto` explícito; a fonte local instalada oferece o eixo wght, sem inventar um eixo óptico ausente
- Hierarquia: nome da página somente no h1; barra superior conserva status. Removidos eyebrow de página, Offline da lateral e status de análise fora de Início/Itens/Apps. Revisão conserva somente a orientação para conferir riscos e confirmar
- Revisão: contagens por tipo numa fileira compacta, ícones distintos de registro/serviço/tarefa/app/energia/inicialização e itens agrupados por risco. Risco baixo não se repete por linha; alto recebe destaque próprio. O domínio atual não tem risco médio; nenhum novo nível foi inventado
- Checkbox compartilhado: desenho do app sobre um input com semântica nativa, rótulos, Space, foco e estado desabilitado. Os handlers e payloads de consentimento são os mesmos
- Itens: Analise no cabeçalho, busca com ícone/placeholder, select de categoria com superfície e seta próprias. Espaço reservado no fim das listas; teste verifica que a última linha termina acima da barra de ação nas quatro larguras
- Celular: Início, Itens, Medições, Jogos e Mais. Painel Mais acessível por teclado, fecha com Escape e após navegar. Histórico, Apps, Revisão e Configurações continuam disponíveis
- Espaço entre botões e parágrafos, rótulos longos limitados e tipografia conferida em 1440, 1100, 768 e 390 px

## Arquivos

`apps/desktop/src/components/Checkbox.tsx`, `components/Icon.tsx`, `App.tsx`, `Games.tsx`, `Measurements.tsx`, `styles/{base,polish}.css`, import de estilos e locales pt-BR/en. Testes em `test/Design.test.tsx`, `e2e/design.spec.ts`; ferramenta complementar `tools/verify-glass-contrast.mjs` adaptada ao menu Mais.

## Testes e comandos

```powershell
cargo fmt --all -- --check
cargo test --workspace --locked --offline
cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
npm --prefix apps/desktop run format:check
npm --prefix apps/desktop run lint
npm --prefix apps/desktop run test
npm --prefix apps/desktop run build
npm --prefix apps/desktop run test:e2e
node tools/verify-glass-contrast.mjs
cargo build --release -p ametista-desktop --locked --offline
pwsh -NoProfile -File tools/verify-desktop.ps1
```

As contagens exatas e os resultados finais estão em `docs/test-count-phase-6.5-polish.json`. Testes de VM continuam ignorados; não houve execução destrutiva no host. Os testes e2e usam o backend mockado. Verificação nativa conserva asInvoker/requireAdministrator, CSP restritiva, nove comandos e ausência de mocks e flags experimentais.

Rust: **122 passaram, 0 falharam, 2 ignorados de VM**. Componentes: **19 passaram**. Suíte e2e completa: **45 passaram, 0 falharam, 0 ignorados, 0 instáveis**. Após as correções finais de CSS, 20 casos de design e 12 casos da matriz foram repetidos e passaram; são repetições, não testes distintos adicionais. fmt, clippy, ESLint, Prettier, TypeScript strict, build release e verificação nativa passaram.

As 128 auditorias da matriz axe não encontraram violações e conservaram verificações de contraste inconclusivas. Amostragem complementar final: **144 casos, 5.276 faixas de texto, zero falhas nos pontos amostrados**.

O axe mantém seus resultados inconclusivos de contraste sobre pseudo-elementos, sem suprimi-los. A amostragem complementar verifica luminância do fundo renderizado e o brilho do botão; viewport e pontos amostrados não equivalem a todos os pixels/estados. Não houve certificação com NVDA/Narrador.

## Capturas

Galeria offline: `outputs/phase-6.5-polish/screenshots/index.html`. São 64 capturas de viewport (oito telas × dois temas × quatro larguras), mais 20 de página inteira: Início, Jogos, Medições, Histórico e Apps, nos dois temas, em 1440 e 390 px. `benchmark` nos nomes dos arquivos corresponde à tela Medições. Sufixo `-full` identifica página inteira.

Os dados visuais são fixtures e não representam este computador. Manifesto de dimensões e SHA-256: `docs/screenshots-phase-6.5-polish.json`. Imagens/evidências completas ficam em `outputs/`, fora do Git, e no ZIP da entrega. Inspeção visual em imagens originais e folhas de contato; leitores de tela e DPI alto ainda pendentes.

## Gate de consumo — reprovado nos dois modos

Executado no Windows real, com confirmação do usuário de que os jogos estavam fechados, janela nativa aberta e visível no Início, tema claro, coleta real ligada. Não foi usado backend mockado. O mesmo harness faz 30 s de aquecimento e 60 amostras de 1 s, somando RAM privada residente da árvore inteira, com CPU normalizada pela capacidade total. Limites inalterados: RAM ≤150 MiB e CPU <1%; nenhum descarte por host ocupado.

| Condição | Pico de RAM privada residente | Pico de CPU | Resultado |
| --- | ---: | ---: | --- |
| Referência histórica Fase 6 | 132,86 MiB | 0,7692% | Aprovado naquela fase |
| Fase 6.5 antes do polimento | — | — | Não medido, havia ficado pendente |
| Polimento, Completo | 238,29 MiB | 3,1209% | **Reprovado** |
| Polimento, Reduzido | 240,62 MiB | 1,5748% | **Reprovado** |

Os valores históricos não constituem uma nova medição pareada antes/depois. Não é possível atribuir a diferença ao polimento ou isolá-la de alterações do ambiente/WebView. Uma execução por modo não prova que Reduzido usa mais memória. O relatório registra picos, não uma alegação de ganho.

JSONs brutos: `outputs/phase-6.5-polish/evidence/interface-full.json` e `interface-reduced.json`; ambos contêm `passed: false`. O harness terminou com erro nos dois casos, como exigido. A falha não foi transformada em aprovação. Hash do executável medido em `measured-binary.json`. Depois dos gates houve apenas o ajuste final da largura do select de Itens e a restauração da marca no cabeçalho móvel; a janela desktop medida e o coletor não foram alterados. O build final foi recompilado e verificado.

```powershell
cargo build --release -p ametista-metrics --example budget --locked --offline
# Escolha Completo nas configurações e feche a janela antes de iniciar
pwsh -NoProfile -File tools/test-resource-budget.ps1 -Mode interface
# Guarde o JSON mesmo quando o harness falhar
Copy-Item work/resource-budget-phase-6/interface.json work/interface-full.json
# Repita após escolher Reduzido
```

O monitor separado não foi reavaliado nem modificado nesta tarefa. Reduzido continua sendo o padrão para novas preferências, mas **não cumpre o orçamento neste host**.

## Riscos e próximo passo

O polimento visual está implementado; o orçamento de consumo da interface segue reprovado e bloqueia a afirmação de conformidade de desempenho. Próximo trabalho: investigar a composição e as invalidações da WebView, otimizar dentro dos limites existentes e repetir os dois gates. Isso pode exigir uma tarefa além da camada visual autorizada aqui. Não foram reintroduzidas flags experimentais nem modificados backend, amostragem, limiares ou recursos de segurança.

Continuam pendentes NVDA/Narrador, DPI alto, outras GPUs/WebView2 e regressão aplicar/desfazer em VM descartável. Não declarar o desempenho aceito ou a Fase 7 pronta com base nesta entrega.
