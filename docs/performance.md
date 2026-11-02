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

Fonte versionada: [perf-baseline.json](perf-baseline.json). Dados de cada execução, com identificadores de boot/plano e horários absolutos de amostra removidos: [evidências](history/perf-baseline-0.6.5/). O JSON conserva hashes dos originais e das cópias públicas, horários UTC de cada execução e o estado conferido antes das coletas. Nenhuma execução foi descartada. Os códigos do probe antigo nos metadados se referem ao critério de pico substituído, não ao resultado do gate novo.

Host: Windows 11 Pro 25H2, build 26200.9711; AMD Radeon 760M, driver 32.0.21030.2001; WebView2 156.0.4314.58; DPI 96/DPR 1; janela 1100×740, tema claro e pt-BR. Coletor real. Executável de referência do commit `720582e`, versão interna 0.6.0, SHA-256 `73f62e24b8ac6bfcba2d501bed24e62b2c9d757f6067ff175c477dd6871ab7e5`. A referência antecede a atualização de versão/identifier; ela não é apresentada como uma coleta de um binário 0.6.5 ainda não construído.

| Modo | RAM das cinco medianas: mín / med / máx (MiB) | CPU das cinco medianas: mín / med / máx (%) | p95 por execução: mín / med / máx (%) |
| --- | ---: | ---: | ---: |
| Completo | 163,06 / **165,21** / 176,25 | 0 / 0 / 0 | 0,3844 / 0,3906 / 0,3971 |
| Reduzido | 179,03 / **205,50** / 206,77 | 0 / 0 / 0 | 0,2604 / 0,2675 / 0,3877 |

O JSON também conserva extremos observados nas 300 amostras de cada modo. CPU é normalizada pela capacidade total da máquina. Não se confundem esses números com FPS ou com o benchmark de desempenho do Windows, cujo protocolo continua separado.

**Escolha do padrão:** Completo usa menos RAM na mediana desta coorte; será o padrão para novas preferências. O Reduzido tem p95 de CPU menor nesta coorte, permanece disponível e continua apropriado para preferências de transparência, acessibilidade ou GPUs diferentes. Escolhas salvas são preservadas. `prefers-reduced-transparency`, quando suportado, desliga a transparência/desfoque; movimento reduzido e alto contraste continuam respeitados. Não há promessa de menor RAM no Reduzido.

**Resultado atual:** Completo passa pela política nova. Reduzido excede o teto de 200 MiB em **5,50 MiB** e falha por esse motivo; a aceitação do titular não foi usada para alterar esse teto. Nenhum dos modos gera alerta p95 nesta coorte. O monitor passou com pico de **2,47 MiB e 0,1299%**; seu limite estrito não mudou.

## O que a investigação estabeleceu

O bisect com duas execuções por versão localizou o salto maior de RAM em `1067244`, quando as telas adotaram a composição nova. A tag v0.6.0 remedida também variou em CPU; a referência antiga de 132,86 MiB não permite atribuir causalidade a uma atualização específica do Windows/WebView2.

A inspeção separada de processos identificou o processo GPU da WebView como o maior consumidor. O perfil encontrou uma camada de página retida após a animação, superfícies ampliadas por sombras e atualizações de uma tabela fechada. Foram mantidos o fundo limitado à viewport, sombras menores, memoização do Orb com limiar de 1%, montagem da tabela sob demanda e suspensão visual fora de foco. Coletor e benchmark Rust continuam ativos quando necessário.

O CSS minificado mantinha somente `-webkit-backdrop-filter:none` no Reduzido. A WebView continuava aplicando a propriedade padrão, apesar do fundo sólido. Corrigir a ordem das declarações e a precedência em alto contraste desligou o blur de fato. O teste usa **CSS compilado**, além dos testes contra o servidor de desenvolvimento.

Menos camadas e ausência de blur não garantiram menor RAM. As alternativas de achatar o fundo e usar PNG aumentaram consumo e foram descartadas. O Paint flashing não ficou discernível nas imagens capturadas; a evidência de pintura vem dos traces, sem atribuir MiB exatos a um efeito. Paleta, tipografia, Orb e botões foram preservados.

## Reproduzir

```powershell
cargo build --release -p ametista-desktop -p ametista-monitor --locked
cargo build --release -p ametista-metrics --example budget --locked
# Escolha o modo, tema e tamanho na UI; feche a janela antes do ensaio
pwsh -NoProfile -File tools/test-resource-budget.ps1 -Mode interface -Effects full
pwsh -NoProfile -File tools/test-resource-budget.ps1 -Mode interface -Effects reduced
pwsh -NoProfile -File tools/test-resource-budget.ps1 -Mode game_monitor
```

O harness mede cinco execuções da interface e só aprova após a avaliação da coorte contra o arquivo versionado. O modo de coleta do probe **não** é aprovação da interface. O resultado canônico fica em `work/resource-budget-phase-6/interface.json`; cada ensaio guarda os cinco arquivos numa pasta única. O monitor mede uma execução de 60 s após aquecimento e usa picos.

Para reproduzir o cálculo sem medir o host, execute `budget.exe evaluate docs/perf-baseline.json full <saida.json>` seguido dos cinco arquivos `docs/history/perf-baseline-0.6.5/full-N.json`; para Reduzido, use `reduced` e seus arquivos. A avaliação Reduzido retorna código 1 por teto de RAM; isso é esperado. Código 2 indica erro de dados/configuração, nunca aprovação. O subcomando `baseline <metadados.json> <candidato.json> <dez arquivos>` gera somente um candidato explícito; ele não substitui a linha de base por conta própria.

Resultados deste host não são garantia para outras GPUs ou WebView2. Mudanças futuras devem preservar a referência e explicar qualquer substituição em PR. O modo Reduzido fora do teto é uma limitação conhecida, sem waiver escondido. Não houve ajustes reais no host; aceite administrativo, NVDA/Narrador e DPI físico permanecem pendentes dos seus roteiros.
