# Consumo da interface e do monitor

A diferença de memória da interface foi aceita pelo titular para fechar a Fase 6.5. O critério antigo de pico de 150 MiB/1% não é mais o orçamento da interface. Ele permanece apenas como evidência histórica nos relatórios antigos. **Isso não altera a segurança, o catálogo, o journal ou as regras de VM.** A investigação de GPU/WebView está encerrada nesta entrega.

## Política vigente

- Interface: cinco execuções por modo; cada uma aquece por 30 s e coleta 60 amostras de 1 s, com árvore inteira, janela visível no Início, mesmo tamanho, tema e DPI, sem jogos
- RAM: calcule a mediana das 60 amostras em cada execução, depois a mediana das cinco execuções. Falhe se ela ultrapassar 200 MiB **ou** piorar mais de 15% sobre a linha de base do mesmo modo
- CPU: preserve mínimo, mediana, máximo e p95. Falhe se a mediana das cinco medianas piorar mais de 15% sobre a linha de base. Se o p95 de uma execução for ≥1%, emita alerta sem bloquear. Não use pico de 1 s como veto. As medianas de CPU deste host são zero pela quantização; zero não significa ausência de trabalho. Por decisão explícita do titular, uma mediana positiva sobre essa base zero bloqueia, sem piso artificial
- Monitor Rust, separado e sem WebView: mantenha **pico de RAM ≤15 MiB e pico de CPU <0,5%**, sem relaxamento por host ocupado
- Dados ausentes, CPU não finita, coleta incompleta, intervalos inválidos ou coorte diferente de cinco execuções são erro. Não descarte execuções nem regrave a linha de base automaticamente

O titular confirmou explicitamente o bloqueio por regressão das medianas de RAM **e CPU**. A comparação de CPU com base zero aceita apenas zero, sem divisão por zero, exceção por host ocupado ou piso artificial. O pico de CPU não bloqueia a interface e o p95 continua somente como alerta.

## Linha de base medida

Fonte versionada: [perf-baseline.json](perf-baseline.json). Dados de cada execução, com identificadores de boot/plano e horários absolutos de amostra removidos: [evidências](history/perf-baseline-0.6.5). O JSON conserva hashes dos originais e das cópias públicas, horários UTC de cada execução e o estado conferido antes das coletas. Nenhuma execução foi descartada. Os códigos do probe antigo nos metadados se referem ao critério de pico substituído, não ao resultado do gate novo.

Host: Windows 11 Pro 25H2, build 26200.9711; AMD Radeon 760M, driver 32.0.21030.2001; WebView2 156.0.4314.58; DPI 96/DPR 1; janela 1100×740, tema claro e pt-BR. Coletor real. Executável de referência do commit `720582e`, versão interna 0.6.0, SHA-256 `73f62e24b8ac6bfcba2d501bed24e62b2c9d757f6067ff175c477dd6871ab7e5`. A referência antecede a atualização de versão/identifier; ela não é apresentada como uma coleta de um binário 0.6.5 ainda não construído.

| Modo | RAM das cinco medianas: mín / med / máx (MiB) | CPU das cinco medianas: mín / med / máx (%) | p95 por execução: mín / med / máx (%) |
| --- | ---: | ---: | ---: |
| Completo | 163,06 / **165,21** / 176,25 | 0 / 0 / 0 | 0,3844 / 0,3906 / 0,3971 |
| Reduzido | 179,03 / **205,50** / 206,77 | 0 / 0 / 0 | 0,2604 / 0,2675 / 0,3877 |

O JSON também conserva extremos observados nas 300 amostras de cada modo. CPU é normalizada pela capacidade total da máquina. Não se confundem esses números com FPS ou com o benchmark de desempenho do Windows, cujo protocolo continua separado.

**Escolha do padrão:** Completo usa menos RAM na mediana desta coorte; será o padrão para novas preferências. O Reduzido tem p95 de CPU menor nesta coorte, permanece disponível e continua apropriado para preferências de transparência, acessibilidade ou GPUs diferentes. Escolhas salvas são preservadas. `prefers-reduced-transparency`, quando suportado, desliga a transparência/desfoque; movimento reduzido e alto contraste continuam respeitados. Não há promessa de menor RAM no Reduzido.

**Resultado da referência:** Completo passa pela política nova. Reduzido excede o teto de 200 MiB em **5,50 MiB**. Nenhum dos modos gera alerta p95 nessa coorte de referência. O monitor de referência passou com pico de **2,47 MiB e 0,1299%**. Os resultados da candidata estão abaixo; a referência não foi substituída.

## Candidata final 0.6.5 — nova medição

Coleta em 2 de novembro de 2026, noite local (3 de novembro UTC), depois dos blocos E/F. Mesmo host, Windows/WebView2/GPU, DPI 96, Início 1100×740, tema claro e pt-BR. O titular fechou o jogo antes da coleta. Não houve builds, testes ou DevTools concorrentes durante as amostras. A conferência prévia, encerrada antes da medição, verificou foco, visibilidade, modo salvo e CSS compilado: três superfícies com blur no Completo e zero no Reduzido. Sem flags experimentais na distribuição.

| Modo | RAM das cinco medianas: mín / med / máx (MiB) | CPU das cinco medianas: mín / med / máx (%) | p95 por execução: mín / med / máx (%) | Gate |
| --- | ---: | ---: | ---: | --- |
| Completo | 156,43 / **163,54** / 181,40 | 0,3253 / **0,3932** / 0,5833 | 1,2959 / 1,3323 / 1,8392 | Reprovado: regressão da mediana de CPU |
| Reduzido | 233,07 / **244,04** / 255,88 | 0,3255 / **0,3886** / 0,6502 | 1,4396 / 1,4557 / 2,3438 | Reprovado: regressão de RAM/CPU e teto de RAM |

Completo: RAM mediana caiu **1,01%** sobre 165,21 MiB; CPU mediana aumentou de zero para 0,3932%. Não há percentual finito de regressão sobre base zero: pelo critério escolhido pelo titular, qualquer valor positivo reprova. Reduzido: RAM mediana aumentou **18,75%** sobre 205,50 MiB e excedeu o teto de 200 MiB em **44,04 MiB**. O p95 em ambos os modos gera alerta; não foi usado como veto adicional. Picos de 1 s da interface não decidem aprovação.

O **Completo permanece padrão** para novas preferências: sua mediana de RAM foi 80,49 MiB menor, com diferença de CPU mediana de apenas 0,0046 ponto percentual. A coorte não demonstra que Reduzido seja melhor no conjunto. Reduzido continua disponível para transparência/acessibilidade, sem promessa de menor consumo. Nenhuma preferência existente é regravada pela política.

Monitor separado, interface fechada, 60 amostras após 30 s: **pico 2,75 MiB / 0,1305% CPU — aprovado** nos limites estritos. A primeira tentativa encerrou no aquecimento por referência incorreta ao recurso do ícone; não produziu amostras nem aprovação. Corrigido somente o identificador Win32 do recurso 1, a nova coleta completou. Não mudou detecção, catálogo ou sessão de jogos.

Evidências públicas: [manifesto com hashes e resultados](history/perf-final-0.6.5/manifest.json), [Completo](history/perf-final-0.6.5/full-evaluation.json), [Reduzido](history/perf-final-0.6.5/reduced-evaluation.json) e [monitor](history/perf-final-0.6.5/monitor-1.json). As onze séries preservam valores numéricos; boot/plano, origem dos horários e nomes alheios são anonimizados. A referência continua intacta. SHA-256 da UI medida: `45016ae0c3ec2b94b05529117ea58aa20e15cfb7c2ad6657059852ff4898f2b3`; monitor corrigido: `0ca0fa512982f8141684a2dd2e6824204e7844a62428231e1428ca71188b62fd`.

**Limitação atual:** o CI funcional verde não equivale a aprovação do gate de consumo da interface. Não há waiver ou relaxamento por host ocupado, e a investigação anterior de GPU/WebView não foi reaberta. A candidata precisa resolver ou ter uma decisão explícita do titular sobre essas regressões antes do aceite de consumo.

## O que a investigação estabeleceu

O bisect com duas execuções por versão localizou o salto maior de RAM em `1067244`, quando as telas adotaram a composição nova. A tag v0.6.0 remedida também variou em CPU; a referência antiga de 132,86 MiB não permite atribuir causalidade a uma atualização específica do Windows/WebView2.

A inspeção separada de processos identificou o processo GPU da WebView como o maior consumidor. O perfil encontrou uma camada de página retida após a animação, superfícies ampliadas por sombras e atualizações de uma tabela fechada. Foram mantidos o fundo limitado à viewport, sombras menores, memoização do Orb com limiar de 1%, montagem da tabela sob demanda e suspensão visual fora de foco. Coletor e benchmark Rust continuam ativos quando necessário.

O CSS minificado mantinha somente `-webkit-backdrop-filter:none` no Reduzido. A WebView continuava aplicando a propriedade padrão, apesar do fundo sólido. Corrigir a ordem das declarações e a precedência em alto contraste desligou o blur de fato. O teste usa **CSS compilado**, além dos testes contra o servidor de desenvolvimento.

Menos camadas e ausência de blur não garantiram menor RAM. As alternativas de achatar o fundo e usar PNG aumentaram consumo e foram descartadas. O Paint flashing não ficou discernível nas imagens capturadas; a evidência de pintura vem dos traces, sem atribuir MiB exatos a um efeito. Paleta, tipografia, Orb e botões foram preservados.

## Reproduzir

```powershell
pwsh -NoProfile -File tools/build-release.ps1
cargo build --release -p ametista-metrics --example budget --locked
# Escolha o modo, tema e tamanho na UI; feche a janela antes do ensaio
pwsh -NoProfile -File tools/test-resource-budget.ps1 -Mode interface -Effects full
pwsh -NoProfile -File tools/test-resource-budget.ps1 -Mode interface -Effects reduced
pwsh -NoProfile -File tools/test-resource-budget.ps1 -Mode game_monitor
```

O harness mede cinco execuções da interface e só aprova após a avaliação da coorte contra o arquivo versionado. O modo de coleta do probe **não** é aprovação da interface. O resultado canônico fica em `work/resource-budget-phase-6/interface.json`; cada ensaio guarda os cinco arquivos numa pasta única. O monitor mede uma execução de 60 s após aquecimento e usa picos.

Para reproduzir o cálculo sem medir o host, execute `budget.exe evaluate docs/perf-baseline.json full <saida.json>` seguido dos cinco arquivos `docs/history/perf-baseline-0.6.5/full-N.json`; para Reduzido, use `reduced` e seus arquivos. A avaliação Reduzido retorna código 1 por teto de RAM; isso é esperado. Código 2 indica erro de dados/configuração, nunca aprovação. O subcomando `baseline <metadados.json> <candidato.json> <dez arquivos>` gera somente um candidato explícito; ele não substitui a linha de base por conta própria.

Resultados deste host não são garantia para outras GPUs ou WebView2. Mudanças futuras devem preservar a referência e explicar qualquer substituição em PR. As regressões da candidata acima são limitações conhecidas, sem waiver escondido. Não houve ajustes reais no host; aceite administrativo, NVDA/Narrador e DPI físico permanecem pendentes dos seus roteiros.
