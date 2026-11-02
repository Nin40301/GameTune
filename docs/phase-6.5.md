# Fase 6.5 — Ametista

> Histórico: orçamento de pico e padrão de efeitos desta fase foram substituídos. Consulte [a política vigente](performance.md); não use este relato como estado atual.
## Resumo

Branch `design/liquid-glass`. Produto e identificadores renomeados para Ametista/ametista. O backend conserva catálogo, perfis, confirmações, IPC autenticado, WAL e recuperação. A única extensão funcional é o bloqueio de escrita exigido quando há dados no diretório anterior de ProgramData: não migra, não move e não apaga. O teste usa somente uma pasta temporária.

Interface branco/lilás e violeta-tinta, Plus Jakarta Sans variável local e ícones Lucide SVG. StatusOrb recebe a RAM disponível do coletor existente; os gráficos usam amostras reais e medianas das rodadas. Nenhum ganho de FPS é anunciado. Início, Itens, Revisão, Limpeza de apps, Jogos, Medições, Histórico e Configurações reutilizam os mesmos handlers e contratos do backend.

Tema claro e efeitos **Reduzidos** são o padrão para novas preferências. O modo Completo continua disponível. A tabela pode receber foco e rolar pelo teclado; seletores usam setas/Home/End, switches têm papel e nome, foco de 3 px e alvos de 40 px. A navegação vira barra inferior até 900 px; o seletor de quatro perfis usa duas linhas em 390 px.

## Commits

1. `db9faa2` — renomeação e bloqueio do diretório legado
2. `c6007fd` — tokens, fonte local, componentes e sistema visual
3. `1067244` — aplicação às oito telas, ícone nativo e layout responsivo
4. Commit deste relatório — testes, evidências e ajustes de seleção para contraste

Apache-2.0 e o titular original do copyright foram preservados. A busca textual pelo nome anterior, sem distinção de caixa, retorna apenas o CHANGELOG e o titular histórico no NOTICE. Essa exceção ao filtro literal é necessária para cumprir a exigência de não trocar o titular; não há identificador de execução antigo. O nome exato do diretório anterior é construído como constante separada da marca atual e explicado no CHANGELOG. O remoto e a tag v0.6.0 continuam sendo o checkpoint histórico; não houve publicação de uma nova versão.

## Arquivos principais

- Renomeação mecânica: workspace/Cargo.lock, manifests npm/Tauri/Windows, executores, scripts fixos, IPC, CI, catálogo/hashes, fixtures, locales e documentação
- Guarda: `crates/windows_runtime/src/lib.rs`, `scripts/storage.ps1`, IPC e mensagens CLI/UI
- Sistema visual: `apps/desktop/src/styles/{tokens,base,buttons,components,screens}.css` e `components/{Button,Icon,Segmented,Switch,StatusOrb,Sparkline}.tsx`
- Telas: `App.tsx`, `Measurements.tsx`, `Games.tsx`, locales e ícone `.ico`
- Testes: `test/Design.test.tsx`, testes de componentes existentes e `e2e/design.spec.ts`
- Licenças da fonte e dos ícones: `LICENSES/`

## Comandos de verificação

Na raiz do repositório:

```powershell
cargo fmt --all -- --check
cargo test --workspace --locked --offline
cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
npm --prefix apps/desktop ci
npm --prefix apps/desktop run format:check
npm --prefix apps/desktop run lint
npm --prefix apps/desktop run build
npm --prefix apps/desktop run test
npm --prefix apps/desktop run test:e2e
cargo build --release --workspace --locked --offline
pwsh -NoProfile -File tools/verify-desktop.ps1
```

O modo `--offline` requer dependências já presentes no cache. No primeiro checkout, obtenha-as antes. Os testes Playwright usam exclusivamente o backend mockado, indicado nas capturas; o bundle de produção foi verificado sem mock. Nenhuma aplicação, remoção Appx, reversão, UAC ou escrita de ajuste foi executada no host.

## Resultado dos testes

- Rust: **122 passaram, 0 falharam, 2 ignorados** (VM), contados pela soma dos resultados do log de `cargo test`; não há testes ignorados incluídos nos aprovados
- Componentes: **18 passaram**
- Playwright: **43 passaram, 0 falharam, 0 ignorados, 0 instáveis** (fluxos, matriz de oito telas, teclado, temas, idiomas e CSP)
- Matriz axe: **128 auditorias** (8 telas × 2 idiomas × 2 temas × 4 larguras), sem violações; mais 38 verificações dos fluxos existentes e 1 em cores forçadas
- fmt, clippy em todos os targets/features, ESLint, Prettier, TypeScript strict e build release verificados
- Manifestos extraídos e conferidos: UI/monitor `asInvoker`, auxiliar `requireAdministrator`; nove comandos tipados, CSP restritiva, ausência de mocks e de flags experimentais

**Limite do axe:** as 128 auditorias da matriz incluem resultados `color-contrast` inconclusivos porque o algoritmo não determina o fundo através de pseudo-elementos. Esses resultados não foram suprimidos. A revisão visual e a amostragem adicional do fundo renderizado complementam o axe; não equivalem a uma certificação de contraste de cada pixel/estado. O JSON original registra `violations` e `incomplete` por tela.

`tools/verify-glass-contrast.mjs` inicia apenas um Vite próprio com backend mockado, captura o fundo sem texto e calcula luminância WCAG em 45 pontos por faixa de texto visível e não ocluída. Inclui o brilho do botão estacionado no centro como pior posição do efeito. Não chama IPC nativo nem aplica ajustes. Requer as dependências de desenvolvimento e o Chromium do Playwright instalados:

```powershell
node tools/verify-glass-contrast.mjs
```

Resultado complementar: **144 casos, 5.818 faixas de texto, nenhum ponto amostrado abaixo do limiar aplicável**. As limitações do método (amostragem, viewport e fundos renderizados) ficam registradas em `work/glass-contrast/report.json`. A faixa de demonstração foi ajustada para texto secundário com contraste suficiente; o brilho transitório dos botões foi limitado a 8% de branco. O resumo versionado está em `docs/contrast-phase-6.5.json`, e as contagens finais em `docs/test-count-phase-6.5.json`.

## Evidências visuais

Matriz: oito telas × dois temas × quatro larguras (1440, 1100, 768 e 390), altura 1000 px. As 64 imagens foram abertas e inspecionadas. Os números dessas capturas são fixtures do backend de teste e **não** medições deste computador.

Pasta: `outputs/phase-6.5/screenshots/`. Convenção: `<tela>-<light|dark>-<largura>.png`. Exemplo: `home-light-1440.png`, `items-dark-390.png`. Galeria offline: `outputs/phase-6.5/screenshots/index.html`. Manifesto com SHA-256: `docs/screenshots-phase-6.5.json`. Arquivos binários/evidências completas permanecem em `outputs/` (ignorado pelo Git), disponíveis no pacote da entrega.

Primeira inspeção: correção do rótulo estreito de Personalizado, rolagem de tabela pelo teclado e hover escuro da navegação. Removido o fade do texto para manter contraste durante a entrada. Segunda revisão: marcador sólido para a seleção da navegação e contorno do seletor, sem depender da transparência para comunicar estado. Sem blur aninhado no DOM e sem animação contínua ou de filtro.

A janela nativa foi iniciada apenas em modo de recursos/somente leitura. A captura estava coberta por um jogo em tela cheia e não permitiu validar a composição nativa. O usuário pediu explicitamente que o gate ficasse pendente; a janela criada para inspeção foi fechada, sem atuar no jogo.

## Consumo antes/depois

Valores anteriores são da Fase 6 e não foram reapresentados como teste desta fase. Mesmo gate: 30 s de aquecimento + 60 s, pico da RAM privada residente da árvore inteira e máximo de CPU por intervalo de 1 s, normalizado pela capacidade total. Sem tolerância adicional por host ocupado.

| Condição | RAM privada residente | Pico de CPU | Estado |
| --- | ---: | ---: | --- |
| Interface anterior, Fase 6 | 132,86 MiB | 0,7692% | Evidência histórica aprovada |
| Interface Ametista, Completo | — | — | Pendente, a pedido do usuário |
| Interface Ametista, Reduzido | — | — | Pendente, a pedido do usuário |
| Monitor anterior, Fase 6 | 2,17 MiB | 0,1311% | Evidência histórica aprovada |

Metas conservadas: interface ≤150 MiB e CPU <1%; monitor ≤15 MiB e CPU <0,5%. Não é possível concluir que o redesign cumpre o orçamento sem os dois novos gates. Por isso, Reduzido é o padrão conservador. Não foram adicionadas flags do WebView; a configuração vazia que remove os defaults experimentais foi preservada.

Para retomar: selecione Completo em Configurações, volte ao Início e feche somente a janela do app; execute o comando abaixo, mantendo a interface visível. Copie o JSON para um nome `interface-full.json`. Repita com Reduzido e guarde `interface-reduced.json`. Não altere os limites se um teste falhar.

```powershell
cargo build --release -p ametista-metrics --example budget
pwsh -NoProfile -File tools/test-resource-budget.ps1 -Mode interface
```

O harness usa a interface real e bloqueia mutações. O relatório fica em `work/resource-budget-phase-6/interface.json` (nome histórico da pasta do harness, sem mudança de protocolo).

## Riscos e pendências

- Gates pós-redesign Completo/Reduzido pendentes por instrução expressa do usuário; sem estimativas ou alegação de melhora de consumo
- NVDA/Narrador, DPI alto, GPUs/versões diferentes do WebView2 e matriz Windows Home/Pro ainda exigem avaliação manual/VM
- Aplicar/desfazer, Appx, ACLs e recuperação com o novo nome precisam de regressão em VM descartável com snapshot, conforme `docs/windows-vm.md` e `docs/desktop-vm.md`; não executados no host
- axe contém verificações de contraste inconclusivas sobre pseudo-elementos; a amostragem adicional e a revisão visual não substituem avaliação completa com leitores de tela/tecnologias assistivas e contraste em todos os estados
- Diretório legado bloqueia ações de escrita e não é migrado; requer decisão humana sobre os dados antigos, sem limpeza automática
- Recursos completos precisam de uma máquina com a janela visível para validar o custo do blur

Próxima fase: concluir os gates e a revisão assistiva/VM, então Fase 7 (empacotamento, assinatura, documentação final e matriz Windows).
