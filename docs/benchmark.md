# Métricas e benchmark

O painel usa leituras reais do Windows, sem administrador. RAM disponível é
`ullAvailPhys` (inclui memória reutilizável), não apenas páginas zeradas. CPU é
o delta ocupado dividido pelo delta total de `GetSystemTimes`; a primeira
leitura não tem CPU. Processos vêm de um snapshot global, agrupados por nome
sem diferenciar maiúsculas. O top 12 soma working sets: páginas compartilhadas
podem aparecer em mais de um processo. Não é uma previsão de economia.

Não há OpenProcess, injeção, hook, esvaziamento de working set ou encerramento
de processos no coletor. A FFI fica isolada em `metrics/src/native.rs`. Usa o
layout NT de Windows 10/11; um retorno incompatível falha sem valores fictícios
ou fallback que abra processos. Uma consulta global exige coletar recursos de
todos os processos; a classe Basic, mais leve, não fornece memória/CPU e não
existe em todos os Windows suportados. Plano e boot são consultas somente leitura.

Fontes: [NtQuerySystemInformation](https://learn.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntquerysysteminformation),
[memória disponível](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-globalmemorystatusex),
[tempos da CPU](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getsystemtimes),
[layout NT de processos](https://github.com/winsiderss/systeminformer/blob/master/phnt/include/ntexapi.h).

## Protocolo guiado

1. Feche outros apps/tarefas, espere estabilizar e esfriar, mantenha carga,
   tomada/bateria e **o mesmo GUID de plano de energia**. Confirme cada condição
   antes de cada rodada. Temperatura não é medida e nenhuma tarefa externa é parada.
2. Complete **5 rodadas de pelo menos 60 s Antes**. A primeira é aquecimento e
   fica registrada, mas é descartada da estatística. Há uma amostra por segundo
   durante a rodada; no painel, uma a cada 2 s. Cancelamento descarta só a rodada.
3. Abra Itens/Revisão para aplicar, pelo fluxo já existente. Somente resultados
   `applied`/`already_applied` confirmados pelo auxiliar registram a intervenção;
   itens ausentes ou com falha não entram. O benchmark não aplica nada sozinho.
4. Reinicie manualmente, reabra e complete **5 rodadas Depois**, descartando a
   primeira. O coletor confere boot diferente, uptime menor que no registro da
   aplicação e plano original. Ajustar o relógio sozinho não comprova reboot.
   Inicie a primeira rodada Depois antes de o uptime alcançar o valor registrado
   ao aplicar; se esperar demais, reinicie novamente e comece logo ao abrir.
   Essa regra conservadora prefere recusar uma prova ambígua. Se testar um
   plano diferente, este protocolo não compara: restaure o plano original com
   o fluxo de reversão ou faça um experimento separado. Nunca há troca automática.
5. Compare RAM disponível, CPU e processos. Não existe escore ou ganho de FPS.

Dados ficam em `%LOCALAPPDATA%\local.gametune.desktop\measurements\benchmark.json`.
Estado versionado, checksum, limite de tamanho, lock exclusivo e substituição
atômica com sync protegem contra corrupção/instâncias simultâneas. Esse arquivo
não concede autorização ao motor e não contém comandos. Uma rodada em andamento
é marcada **antes** da coleta; após crash/reabertura, é descartada e as anteriores
são preservadas. Não há retomada de uma rodada pela metade. Suspensão, salto de
relógio, lacuna >2 s, mudança de plano/boot ou cobertura insuficiente invalidam
a rodada. Janela de conclusão: 60–62 s, pelo menos 60 amostras. Sessenta pontos
a 1 Hz cobrem aproximadamente 59 s entre timestamps; a validação desse span
permite 500 ms de jitter das consultas (mínimo 58,5 s), sem encurtar a duração
real de 60 s medida por relógio monotônico.

Alterações são bloqueadas durante uma rodada, inclusive por IPC; uma reserva
também impede começar a rodada enquanto o auxiliar está ativo. Se o app cair
entre a aplicação e o registro da intervenção, ele não presume sucesso: confira
o journal e refaça a revisão antes de prosseguir. Um erro salvando o benchmark
é mostrado separadamente, preservando o resultado real da aplicação.

## Estatística e limitações

Cada rodada válida gera uma mediana temporal. As **4 medianas de rodadas** por
condição são as unidades da comparação; as 240 amostras não são tratadas como
240 experimentos independentes. Relatamos mediana, p5/p95 por interpolação linear,
mínimo/máximo, MAD e cada mediana de rodada. Com apenas 4 rodadas, os percentis
são descrições da dispersão, **não intervalos de confiança de 95%**.

Marque **inconclusivo** se as faixas p5/p95 se sobrepõem OU se a diferença das
medianas não supera `max(largura p95-p5 Antes, largura Depois, piso)`. Pisos
conservadores: 16 MiB de RAM, 0,5 ponto percentual de CPU, 1 processo. Isso não
é teste de significância, prova causal nem garantia de desempenho. Faixas
separadas apenas recebem “diferença observada”. Tarefas externas podem dominar.

`GetSystemTimes` está sujeito à semântica de grupos do Windows acima de 64 CPUs
lógicas; essa topologia precisa de validação específica antes de interpretar
CPU. Amostragem não captura processos que nascem e morrem entre dois snapshots.

## PresentMon opcional, sem anexar ao jogo

O usuário instala sua ferramenta e indica um executável PresentMon existente
(nome/extensão e cabeçalho PE; isso **não autentica** o fornecedor). O GameTune
nunca executa esse arquivo, baixa arquivos ou cria sessão ETW. O console oficial
[abre handles para metadados](https://github.com/GameTechDev/PresentMon/blob/69dafa12b9804b1ea0a5d7595cba8eaf3c127e68/PresentMon/OutputThread.cpp#L303).
Por isso a integração é **importação offline**, não captura automática.

Importe CSV de uma captura externa com métricas **v1** (`--v1_metrics`), conforme
o [contrato oficial](https://github.com/GameTechDev/PresentMon/blob/main/README-ConsoleApplication.md).
Exige `Application,ProcessID,SwapChainAddress,TimeInSeconds,MsBetweenPresents`,
um processo/swap chain, timestamps crescentes, valores finitos e >=100 intervalos
após descartar a primeira linha. Rejeita captura ambígua, métrica GPU como frame
time, CSV >32 MiB ou >1 milhão de linhas. Nenhum PID é usado para abrir processo.

FPS médio = 1000 / média dos intervalos em ms. 1% low = 1000 / média dos
`ceil(n/100)` intervalos mais lentos. Frametime: mediana, p5/p95/p99. São
intervalos entre apresentações, incluindo apresentações não exibidas; não há
promessa de FPS visto na tela. A UI só mostra capturas se a instalação indicada
continua presente. Após reabrir, indique novamente a instalação para exibi-las.

## Exportar e anonimizar

Exporte JSON ou HTML pelo seletor nativo. A UI não fornece caminhos livres;
o backend restringe a extensão ao formato escolhido. HTML tem CSP `default-src
'none'`, não usa script, CSS inline, links ativos ou recursos remotos, e escapa
todos os dados. JSON inclui protocolo, rodadas e método para auditoria.

Anonimizar vem marcado: remove listas/nomes de todos os processos, nome do app
da captura, uptime e identificadores de boot. É uma projeção, não substituição por regex
de um nome de usuário. Caminhos de processos e do PresentMon não são coletados
no protocolo. IDs do catálogo, GUID do plano e horários permanecem. Arquivos
locais do experimento não são anonimizados retroativamente pela exportação.

## Orçamentos separados — atualizados na Fase 6

```powershell
cargo build -p gametune-desktop -p gametune-helper -p gametune-monitor --release --locked --offline
cargo build -p gametune-metrics --example budget --release --locked --offline
./tools/test-resource-budget.ps1 -Mode interface
./tools/test-resource-budget.ps1 -Mode game_monitor
```

Desktop interativo sem administrador, sem outra instância. A interface permanece
aberta e visível, com o dashboard real. O monitor é outro processo Rust, sem
WebView, em modo somente leitura para esse ensaio. Ambos usam o mesmo coletor/gate,
30 s de aquecimento e 60 s de coleta. RAM é working set privado residente somado
da árvore inteira; CPU é o **máximo de amostras de 1 s**, normalizado por toda a
capacidade da máquina. Nenhum working set é esvaziado e nenhum processo externo
é encerrado. Páginas compartilhadas não são contadas várias vezes.

| Perfil fechado do gate | RAM privada máxima | CPU máxima de 1 s |
| --- | --- | --- |
| Interface aberta | <=150 MiB | <1%, meta anterior preservada |
| Monitor de jogos Rust | <=15 MiB | <0,5% |

Limites não são ajustados por host ocupado. Coleta incompleta, valores ausentes,
picos fora do limite ou gaps excessivos reprovam. O teste fecha somente a janela
que criou ou pede parada pelo nonce da instância do monitor; não termina jogos.
Evidência em `work/resource-budget-phase-6/`. Recursos WMI compartilhados fora da
árvore e o auxiliar lançado via UAC precisam de ensaio adicional em VM; o gate
somente leitura não representa o custo de aplicar/reverter uma sessão elevada.

Nenhuma flag experimental é configurada, seguindo a
[orientação da Microsoft](https://learn.microsoft.com/en-us/microsoft-edge/webview2/concepts/webview-features-flags).
`additional_browser_args("")` envia a propriedade suportada vazia para substituir
os switches experimentais padrão do wry. Não desativa GPU, SmartScreen, sandbox
ou isolamento. O perfil de navegador `webview-standard` isola dados das antigas
flags; as preferências visuais podem reiniciar, sem tocar no WAL ou medições.
O monitor não tem navegador. Os resultados da Fase 5 continuam históricos,
medidos com orçamento/configuração anteriores; veja os resultados atuais em
`phase-6.md` e `test-count-phase-6.json`.
