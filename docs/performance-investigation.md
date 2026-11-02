# Investigação de consumo da interface — 2026-11-01

O salto de RAM aparece em `1067244`, quando as telas passam a usar a nova composição. Além disso, o CSS minificado de produção mantinha o blur ativo no modo Reduzido, apesar do fundo sólido. A correção é testada contra o CSS compilado, e não somente contra o servidor de desenvolvimento. A identidade Ametista foi preservada. As correções diminuem trabalho visual, mas **o orçamento continua reprovado**. Não houve mudança no backend, na segurança, no coletor Rust, no probe, no harness ou nos limites. Nenhuma configuração do Windows foi aplicada.

## Método e ambiente

Cinco versões release foram recompiladas e medidas duas vezes nesta sessão. As versões antigas ficaram num worktree separado; a pasta principal permaneceu em `design/liquid-glass`. A comparação usa Início nativo visível, pt-BR, tema claro, janela configurada em 1100×740, DPI 96/DPR 1 e jogos fechados. Efeitos completos onde disponíveis. Windows 11 Pro 25H2, build 26200.9711; WebView2 156.0.4314.58; AMD Radeon 760M, driver 32.0.21030.2001; 12 processadores lógicos. Outros aplicativos do usuário permaneceram abertos, sem encerramento ou ajuste.

Cada gate original aquece por 30 s e coleta 60 amostras de 1 s. O valor reportado é o **pico do working set privado da árvore inteira**, não memória comprometida nem média. CPU é o pico de um intervalo de 1 s normalizado pela capacidade total. Limites existentes: 150 MiB e 1%. Não houve descarte por host ocupado. Não ocorreram builds, testes, DevTools ou inspeção por UI Automation durante as coletas aceitas.

A primeira tentativa de `v0.6.0` foi contaminada por inspeção de acessibilidade durante o intervalo medido. Seu JSON foi conservado, explicitamente excluído e substituído por outra coleta; para essa tag, a tabela usa os arquivos `run-2` e `run-3`. Todos os outros resultados, incluindo falhas, permanecem na comparação.

As primeiras pastas `stock-reduced` e `bounded-reduced` não têm confirmação confiável do modo efetivo, porque a preferência foi escrita diretamente no storage sem validar uma nova abertura. Foram preservadas como **modo não confirmado** e não compõem a tabela Reduzido final. Uma tentativa (`bounded-reduced/run-2`) ainda devolveu `metrics_unavailable`, sem gerar JSON novo: o wrapper copiou o resultado anterior, que foi conservado e explicitamente marcado como obsoleto. Esse erro no agregador foi corrigido para recusar cópias obsoletas; o harness original e o probe continuam idênticos.

A comparação Reduzido foi refeita após selecionar o controle real da UI, recarregar e reabrir em novo processo. `preflight-select.json` e `preflight-reopen.json` confirmam tema claro, 1100×740/DPR 1, foco e modo reduzido em ambas as versões. O CSS original, mesmo com a preferência correta, conserva três blurs; o final tem zero. As janelas de diagnóstico são encerradas e a porta é conferida fechada antes de cada par de gates; durante as coletas não há CDP ou inspeção. Os horários UTC estão nos metadados. Houve uma pausa na conversa entre o bisect inicial e a rodada confirmada dos modos; a comparação é na mesma sessão/configuração, sem alegação de isolamento temporal perfeito.

Os executáveis, hashes completos, horários, logs e cada uma das 60 amostras estão em `outputs/perf-investigation`. O arquivo `bisect.json` reúne as dez coletas iniciais. `results.json` reúne também os candidatos e os dois modos finais. O SHA-256 do harness pós-renomeação é `8f7894a932943f1d02fd3966c0d7e5ef898f3a7482b8ed9ed9fb2a1e64bb35d7`; o probe é `400d696b92daea466b8f65b8a08170fff56f8b4ac4cca6e96b20e2c7d25eb454`. O harness da tag anterior usa o nome antigo dos binários; seu conteúdo original e hash próprio foram preservados. Os dois probes implementam os mesmos limites e método. Todas as variantes posteriores reutilizam **o mesmo arquivo binário** do probe de `c6e168e`.

<!-- RESULTS -->

| Build | RAM 1 / 2 (MiB) | CPU 1 / 2 (%) | SHA-256 do executável |
| --- | ---: | ---: | --- |
| `v0.6.0` | 137.26 / 149.35 | 0.7812 / 2.3438 | `81f83acf5de72473592dbb843cf10028ca95f52ead5684554983b12b513c1e22` |
| `db9faa2` | 150.41 / 149.70 | 3.2552 / 1.5727 | `996b40e327bc6405852570e8e45f32e5192d2aa0c6db5c4c3fb720f7805deeeb` |
| `c6007fd` | 149.15 / 164.04 | 2.7167 / 3.1332 | `06612c51a858c66c42d8db4f640d45bb9b388517ffcf71aa5cd2e35125017500` |
| `1067244` | 230.38 / 230.58 | 0.6510 / 0.5208 | `795f6fdffbb3a92ab5bf11c29414376c3089cbf1dc99275e7121f6d167a65d97` |
| `c6e168e` | 232.45 / 228.36 | 0.6485 / 0.9115 | `0365a74b68ee0e8aeb6bb2950669aae684872acd12c6d4691759d4033f756469` |

SHA-256 dos executáveis medidos:

- `v0.6.0`: `81f83acf5de72473592dbb843cf10028ca95f52ead5684554983b12b513c1e22`
- `db9faa2`: `996b40e327bc6405852570e8e45f32e5192d2aa0c6db5c4c3fb720f7805deeeb`
- `c6007fd`: `06612c51a858c66c42d8db4f640d45bb9b388517ffcf71aa5cd2e35125017500`
- `1067244`: `795f6fdffbb3a92ab5bf11c29414376c3089cbf1dc99275e7121f6d167a65d97`
- `c6e168e`: `0365a74b68ee0e8aeb6bb2950669aae684872acd12c6d4691759d4033f756469`

O salto maior ocorre em `1067244`; a renomeação não explica esse salto. O `v0.6.0` remedido passa uma vez e falha em CPU na outra. Logo, o desvio atual de CPU também existe sem o redesign. A referência histórica não substitui esse controle.

Comparação atual dos dois modos (mesmo executável/probe por condição, duas execuções):

| Condição | RAM 1 / 2 (MiB) | CPU 1 / 2 (%) |
| --- | ---: | ---: |
| Antes, Completo | 232.45 / 228.36 | 0.6485 / 0.9115 |
| Antes, Reduzido (blur ativo por defeito) | 208.82 / 218.18 | 0.6510 / 1.4323 |
| Depois, Completo | 170.64 / 182.02 | 0.5208 / 0.6510 |
| Depois, Reduzido | 238.23 / 214.59 | 0.9115 / 1.0417 |

Executável final: `73f62e24b8ac6bfcba2d501bed24e62b2c9d757f6067ff175c477dd6871ab7e5`.
final-full: no pior dos dois ensaios, faltam 32.02 MiB e 0.0000 ponto percentual de CPU para o limite. Nenhum pico foi diluído em média.
final-reduced: no pior dos dois ensaios, faltam 88.23 MiB e 0.0417 ponto percentual de CPU para o limite. Nenhum pico foi diluído em média.

## O que o perfil mostrou

Capturamos o equivalente às ferramentas Performance, Rendering/Paint flashing e Layers da WebView real por Chrome DevTools Protocol. A porta de diagnóstico foi habilitada somente no processo de inspeção, em loopback, e encerrada antes dos gates. A opção de diagnóstico não foi incorporada à configuração distribuída. A CSP e as capacidades Tauri permanecem intactas. Procedimento suportado: [depuração de WebView2](https://learn.microsoft.com/en-us/microsoft-edge/webview2/how-to/debug-visual-studio-code); domínios [Tracing](https://chromedevtools.github.io/devtools-protocol/tot/Tracing/), [LayerTree](https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/) e [Overlay](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/).

Arquivos `profiles/1067244/performance-10s.json` e `profiles/bounded/performance-10s.json` podem ser importados na aba Performance do DevTools. Os arquivos `page.json`, `layers.json`, `mutations.json`, `gpu.json` e `process-cpu.json` permitem conferir as condições, razões de composição e atividade.

| Evidência em 10 s | `1067244` | Composição `bounded`, antes do ajuste de prefixos |
| --- | ---: | ---: |
| Camadas / camadas com conteúdo | 11 / 8 | 9 / 7 |
| Alterações observadas no DOM | 79 | 36 |
| Paint / duração somada dos eventos | 10 / 4,599 ms | 10 / 2,287 ms |
| Layout | 2,072 ms | 1,407 ms |
| Script no renderer | 11,909 ms | 9,976 ms |

São janelas diagnósticas individuais, com overhead do DevTools, e não novos gates ou percentuais garantidos de economia. As mutações dependem dos valores reais do sistema.

Ofensores e hipóteses, em ordem de impacto sustentada pelas evidências:

**Defeito confirmado no Reduzido de produção**: `compiled-css/comparison.json` conserva as duas regras minificadas. A antiga contém somente `-webkit-backdrop-filter:none`; a nova contém também `backdrop-filter:none`. A WebView real, com `data-effects="reduced"`, ainda retornava `blur(18px) saturate(1.7)` nas três superfícies originais. A ordem das declarações agora deixa a propriedade padrão por último. Um teste carrega o CSS gerado pelo build e verifica zero blur em ambos os temas e em forced-colors. Isso cobre a diferença que os testes contra CSS de desenvolvimento não detectavam. Em forced-colors também foi corrigida a precedência dos seletores. São correções visuais do comportamento previsto, sem configuração experimental da WebView.

1. **Composição e superfícies da WebView**: o salto de aproximadamente 150–164 para 230 MiB coincide com a aplicação das telas. A árvore do gate tem um filho WebView muito maior que o processo Rust; a inspeção separada identifica o processo GPU como principal consumidor de memória. O perfil confirma aceleração por hardware, não a antiga suposição de composição por software. As camadas são evidência de custo, mas não permitem atribuir um número exato de MiB a cada efeito. Na inspeção separada da composição `bounded`, antes do ajuste de prefixos e após 30 s, o modo Completo estava visível e focado em 1100×740/DPR 1: o processo GPU tinha **151,74 MiB**, renderer **32,14 MiB** e browser **29,38 MiB** de working set privado. As identidades vêm de `SystemInfo.getProcessInfo` da própria WebView e os valores de contadores de desempenho por esses PIDs; não foram abertos processos de jogos. É uma amostra com a porta/cliente de diagnóstico ativos, não um pico do gate nem prova de que essa quantidade pertença somente ao CSS. Evidências: `profiles/bounded/memory-process-identities.json` e `process-memory.json`.
2. **Camada de página retida e fundo fixo**: `1067244` mantém uma página de 811×1179 com razão `ActiveTransformAnimation` após o aquecimento, além do fundo fixo e de uma superfície de sobreposição. Remover o preenchimento da animação reduz uma camada no teste isolado. A alternativa que achatou tudo no documento piorou muito a memória; foi rejeitada. A solução final limita o fundo à primeira viewport, permite que ele role e mantém uma composição estática da página (`Trivial3DTransform`) para evitar a regressão de rasterização do documento inteiro.
3. **Blur e sombras**: três filtros de fundo no Início, com superfícies expandidas pelas sombras. Os limites observados de topbar/sidebar/cartão passam de 891×138 / 278×790 / 576×686 para 853×100 / 240×752 / 538×648. O probe isolado sem blur reduz camadas, mas não demonstra uma economia de RAM a frio. Completo mantém somente duas ou três superfícies grandes desfocadas por tela; após corrigir o CSS compilado, Reduzido efetivamente não usa blur.
4. **Atualizações desnecessárias**: 39 das 79 mutações originais são células de processos de uma tabela fechada. Agora a tabela só é montada quando aberta. O Orb é memoizado e só avança quando a RAM muda mais de 1% em relação à última renderização; mudanças pequenas acumulam. A contagem de mutações no círculo foi de cinco para uma nas capturas comparadas. As sparklines permanecem medidas e deixam de renderizar quando os valores são iguais.
5. **JavaScript do dashboard**: alguns milissegundos por 10 s, sem evidência de ser o principal causador do salto de memória. O status continua sendo atualizado para exibir CPU/processos reais; não se substituem dados por valores fixos. Fora de foco ou oculto, a UI suspende consultas e animações visuais; o coletor/benchmark Rust continua funcionando.
6. **Hipóteses não confirmadas**: o círculo do Orb já tinha duração de transição zero no commit do salto; não havia uma animação de 900 ms a corrigir. A fonte local foi introduzida em `c6007fd`, antes do salto maior, e foi preservada. O perfil registra uma face de fonte com erro e o subconjunto latino carregado; não isola seu custo em MiB. A janela configurada tem o mesmo tamanho em todas as versões. Não há evidência de que uma mudança de tamanho explique o salto.

O Paint flashing foi ativado e houve capturas, mas elas não retiveram realces verdes discerníveis. Portanto **não foi validado visualmente quais regiões piscaram**. O evento `LayerTree.layerPainted` também não foi entregue nesta WebView; seus zeros não significam ausência de pintura. A análise de Paint usa os eventos reais do trace, que incluem documento/página. Não se convertem os clips sentinela do trace em área literal.

## Alterações mantidas e alternativas rejeitadas

Antes de cada alteração visual, o usuário foi informado. Permanecem a paleta, Plus Jakarta Sans, StatusOrb, botões, layouts e telas aprovados. Foram encurtadas as sombras ambientes, limitado o fundo estático, delimitado o blur e removida a retenção da animação de entrada. Após encontrar o defeito na versão minificada, também foi corrigida a ordem das propriedades prefixadas e a precedência de forced-colors, para efetivamente desligar blur no Reduzido e no alto contraste. A alteração foi comunicada antes de ser aplicada. O Orb continua mostrando dados reais, com o limiar acima; há testes de atualização acumulada e troca de idioma. A suspensão fora de foco é exclusivamente visual.

As tabelas de modos usam apenas os pares confirmados. As primeiras coletas com rótulo Reduzido não confirmado são mantidas em `results.json` como `effects: "unconfirmed"`, sem afirmar que eram Reduzido.

A alternativa `optimized` tinha sete camadas, mas usou 286,10/290,78 MiB. A variante `raster`, com PNG local do mesmo fundo, usou 290,69/306,36 MiB. **Ambas foram descartadas**, incluindo os PNGs. Menos camadas não foi tratado como prova de menor consumo. Todos esses resultados permanecem nos arquivos brutos. A composição `bounded` foi mantida, com a correção posterior das propriedades prefixadas. As pastas `final-full` e `final-reduced` contêm o mesmo executável final. Não houve volta à interface anterior.

## Reproduzir e verificar

```powershell
npm ci --prefix apps/desktop
npm run format:check --prefix apps/desktop
npm run lint --prefix apps/desktop
npm run test --prefix apps/desktop
npm run build --prefix apps/desktop
npm run test:e2e --prefix apps/desktop
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
cargo build --release -p ametista-desktop --locked
cargo build --release -p ametista-metrics --example budget --locked
# Escolha o modo em Configurações e feche a janela antes de cada execução
pwsh -NoProfile -File tools/test-resource-budget.ps1 -Mode interface
# Guarde o JSON de work/resource-budget-phase-6/interface.json inclusive se falhar
```

O harness permanece idêntico. Cada execução precisa de janela visível, mesmo tamanho, DPI, tema e condição de foco; não abra DevTools nem rode builds/testes durante a coleta. As falhas de orçamento são esperadas neste host e não devem ser mascaradas. Os logs, snapshots de amostras, metadados dos builds antigos e scripts auxiliares foram conservados no pacote de evidências; os executáveis antigos ficam preservados localmente, fora do ZIP, com seus hashes; não é necessário executar alvos de catálogo para medir.

## Verificação, riscos e pendências

<!-- VALIDATION -->

Rust: **122 aprovados, zero falhas e cinco ignorados** com `--workspace --all-features`. Os cinco são exclusivos de VM: dois roundtrips HKCU, um ensaio de catálogo/reboots e dois ensaios de armazenamento protegido/roundtrip completo. Não foram executados no host. `fmt` e `clippy --all-targets --all-features -D warnings` passaram. Componentes: **22 aprovados**; Prettier, ESLint e TypeScript strict/build passaram. A lista e as contagens derivam dos logs finais, não de uma estimativa.

<!-- BROWSER-VALIDATION -->

Playwright final: **48 aprovados**, incluindo o novo teste de CSS compilado e forced-colors, mais 128 auditorias de páginas na matriz pt-BR/en × claro/escuro × 1440/1100/768/390, sem violações axe reportadas. O axe registra verificações incompletas para conteúdo sobre pseudo-elementos: o resultado não é uma certificação AA completa. O ensaio complementar de contraste sobre fundos renderizados passou em **144 casos e 5.276 trechos**, com grade de 45 amostras por trecho; cobre só a viewport e não cada pixel. Teclado, efeitos reduzidos, movimento reduzido e forced-colors permanecem testados. Foram examinadas as 84 capturas finais (64 de viewport e 20 de página completa) em 12 folhas de contato, além da captura nativa do perfil. A galeria offline é `outputs/perf-investigation/gallery.html`.

`tools/verify-desktop.ps1` passou: asInvoker na UI/monitor, requireAdministrator no auxiliar, nove permissões tipadas, CSP restritiva e ausência de mock/flags customizadas no build de produção. O hash do executável e cada arquivo de produção do frontend foram comparados com `final-full/build.json` e correspondem à versão medida; os dois modos finais usam esse mesmo binário. O harness foi conferido byte a byte por SHA-256. Logs e contagens estão em `verification/summary.json` no pacote de evidências.

Capturas finais estão em `outputs/perf-investigation/screenshots`. Dados determinísticos aparecem somente nas capturas/testes com backend mockado; os gates e traces usam o coletor real. Não houve handles em jogos, encerramento de processos de terceiros, elevação ou aplicação de perfis no host.

O trabalho não comprova conformidade de consumo nem uma economia estável de RAM. Corrigir o Reduzido era necessário para desligar o blur de fato, mas não se traduz automaticamente em menor working set: superfícies que deixam de ser compostas separadamente também podem mudar a rasterização do documento. Os dois ensaios não isolam o custo de cada buffer. O maior custo restante está na composição/árvore da WebView; CPU de pico também varia e falha, inclusive em builds anteriores ao redesign. Dois ensaios não permitem separar causalmente cada efeito ou atribuir o desvio histórico a uma versão específica de Windows/WebView2. A referência antiga de 132,86 MiB/0,7692% não é uma medição pareada atual.

NVDA/Narrador, DPI alto, outras GPUs/WebView2 e testes administrativos em VM continuam pendentes. O monitor Rust separado não foi modificado nem reavaliado. Próximo passo: investigar a alocação do processo GPU/browser com ferramentas de memória, mantendo o visual e o mesmo gate; não iniciar a Fase 7 alegando que o orçamento passou.
