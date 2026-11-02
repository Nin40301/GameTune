# Jogos e monitor independente

## Usar

1. Em **Jogos**, descubra candidatos dos launchers ou escolha um `.exe` pelo
   seletor nativo. Confira que é o executável do jogo, não apenas seu launcher.
2. Selecione o cadastro, escolha as ações temporárias, leia as descrições e
   confirme o consentimento. Habilite e salve o perfil. Nada é aplicado ao salvar.
3. Em **Configurações**, confirme e inicie o monitor. Ele aparece na bandeja;
   a interface pode ser fechada. Não há autostart, tarefa, serviço ou driver
   instalado pelo monitor.
4. Abra o jogo **depois** de iniciar o monitor. Ao detectar o caminho exato,
   ele pede UAC para um auxiliar da sessão. Cancelar UAC impede a aplicação.
   Jogos já abertos são ignorados; é preciso reabri-los para ter eventos novos.
5. Ao fechar o último jogo que usa uma ação, o auxiliar restaura seu snapshot.
   Use **Pare e restaure** na bandeja ou em Configurações para encerrar. Edite
   a biblioteca só depois de o monitor liberar o lock.

O menu da bandeja é acessível pelo teclado do Windows e tem **Abra o Ametista**
e **Pare e restaure**. O tooltip indica observação, sessão, autorização,
recuperação ou falha. Um erro fica na bandeja até o usuário parar o monitor;
não há repetição automática de UAC em loop.

## Ações permitidas

O catálogo complementar embutido `catalog/game-actions.json`, seu schema e
hash limitam os perfis a:

| ID | Durante o jogo | Ao terminar |
| --- | --- | --- |
| `reg.game_mode` | Ativar Modo de Jogo, quando compatível | Restaurar tipo/bytes/ausência originais |
| `power.high_performance` | Selecionar/criar Alto Desempenho quando suportado | Restaurar GUID/inventário original |
| `svc.diagtrack` | Parar, sem alterar StartMode/DelayedAutoStart | Restaurar execução e configuração originais |
| `svc.fax` | Mesma pausa; pode interromper envio/recepção de fax | Mesma restauração |
| `svc.retaildemo` | Mesma pausa; interrompe demonstração de loja | Mesma restauração |

As identidades/fontes dos cinco itens já constam em `catalog-sources.md`. O
subconjunto rejeita itens não verificados, risco alto, Appx, tarefas, inicialização,
itens protegidos ou que exigem reinício. Serviço pausável é uma marca explícita
nesse catálogo, não uma autorização para pausar qualquer serviço. O executor
Windows revalida a marca. Serviços com dependentes ativos falham sem cascata;
nenhum processo é terminado à força. [Contrato de parada do SCM](https://learn.microsoft.com/en-us/windows/win32/services/stopping-a-service).

Um gatilho do Windows pode reiniciar um serviço durante o jogo. O monitor não
força paradas repetidas nem muda sua inicialização para impedir isso; a pausa
é uma solicitação pontual, registrada e restaurável, sem promessa de economia.

O Modo de Jogo do catálogo está verificado a partir de build 22000. A UI o
desabilita quando incompatível e o helper verifica novamente. Ausência de um
recurso gera resultado ignorado. Não há garantia de redução de RAM, CPU ou FPS.

## Arquitetura e recuperação

`ametista-monitor.exe` é Rust/Win32, **asInvoker**, sem Tauri/WebView. Um worker
COM recebe eventos WMI; outro cuida do detector e do protocolo. Não há processo
PowerShell de monitoramento ou polling que abra processos. A thread da bandeja
continua responsiva durante UAC/aplicação/restauração. O cliente usa runtime
Tokio de uma thread, o mesmo pipe autenticado da interface e um helper irmão fixo.

O helper **requireAdministrator** fica vivo somente durante uma sessão ativa:
revalida IDs, confirmação, SID/PID, nonce/hash e sequência. Quadros aceitam
apenas `set { ids }`, `ping {}` e `finish {}`. A união autorizada na abertura
limita todos os `set`. Heartbeat a cada ~3 s; EOF ou prazo de 10 s tenta rollback
de todas as ações e encerra. Payload malformado/replay também encerra e reverte.
O prazo de execução de uma operação Windows é separado do heartbeat. Depois
da resposta, o cliente drena eventos acumulados durante UAC antes de aplicar.

O journal `game-session.wal` fica em `%ProgramData%\Ametista\<SID>`, protegido
como o journal permanente. O mesmo motor grava original **antes** da alteração,
e guarda status/hash/transições. Um lock de máquina fica retido durante a sessão
e impede aplicar/desfazer permanentemente enquanto ações temporárias estão
ativas. Mesmo depois de um crash, uma dívida temporária impede mudanças
permanentes ou de outro usuário até a recuperação. Não reutiliza o original
antigo de um ajuste permanente: o temporário captura o estado vigente e restaura
esse estado, preservando o journal permanente independente.

Jogos simultâneos compartilham ações pela união dos perfis; a última instância
deve fechar para liberar sua parte. PID **mais CreationDate** previnem reuso;
eventos repetidos ou atrasados não ressuscitam uma sessão terminada. Um processo
não cadastrado que reutilize um PID remove o estado antigo, sem ativar perfil.

Se só o monitor falhar, o helper detecta desconexão/prazo e tenta restaurar. Se
helper/PC falhar, o WAL mantém Prepared/Applied/Reverting. Na próxima execução
do monitor, uma indicação local gravada **antes** do UAC pede recuperação; o
helper sempre restaura o WAL temporário anterior antes de aceitar nova sessão,
mesmo se a indicação local estiver ausente. Recuperação precisa de UAC: o app não
instala um serviço privilegiado para evitá-lo. Mudança externa conflitante não
é sobrescrita; dívida/erro permanecem para revisão na VM.

Ao reabrir a interface com indicação de recuperação, Início oferece acesso a
Configurações, sem iniciar UAC automaticamente. Uma biblioteca editável corrompida
não impede restaurar o WAL protegido: o monitor usa biblioteca vazia apenas para
recuperação, preservando o arquivo corrompido e sem autorizar novos perfis.

Se o WMI deixar de responder por mais de 10 s, a fila de 128 eventos encher,
faltarem dados para encerrar um PID ativo ou o canal cair, a sessão falha fechada
e tenta restauração. Um RPC WMI travado pode atrasar o encerramento do worker;
o pipe é encerrado antes de aguardar a thread, preservando rollback pelo helper.

## Descoberta local e limites

Somente arquivos de configuração são lidos. Não consulta processos de launchers,
registro para localizar bibliotecas, rede, tokens de autenticação na saída ou
árvores de executáveis. Limites: 1 MiB por arquivo, 32 bibliotecas Steam,
512 manifests por pasta, 256 candidatos, 128 jogos cadastrados. Caminhos de
configuração em UNC, unidades mapeadas de rede e junction/symlink são rejeitados antes de ler descendentes.

| Launcher | Arquivo lido | O que pode sugerir |
| --- | --- | --- |
| Steam | `%ProgramFiles(x86)%\Steam\steamapps\libraryfolders.vdf` e `appmanifest_*.acf` nas bibliotecas | Nome e diretório; `.exe` precisa ser escolhido |
| Epic | `%ProgramData%\Epic\EpicGamesLauncher\Data\Manifests\*.item` | Nome e `InstallLocation` + `LaunchExecutable`, se explícitos |
| Battle.net | `%APPDATA%\Battle.net\Battle.net.config` | `Games/*/InstallPath`, quando presente; `.exe` manual |
| Riot | `%ProgramData%\Riot Games\RiotClientInstalls.json` | Chaves de `associated_client`; confira se é jogo ou launcher |
| Minecraft Launcher | `%APPDATA%\.minecraft\launcher_profiles[ _microsoft_store].json` | Perfis/diretório e Java explícito, se presente; não infere Bedrock |

Esses formatos de arquivos **não são APIs estáveis documentadas pelos fabricantes**.
Os parsers foram validados com fixtures, não com todas as versões dos launchers.
Layouts desconhecidos são ignorados. Não há selo de compatibilidade para os
formatos. Steam instalado fora do diretório padrão pode precisar de adição manual;
as bibliotecas adicionais são lidas quando seu arquivo padrão é encontrado.
`LaunchCommand`, autenticação, comandos de instalação e launch options nunca
são executados ou enviados ao auxiliar. Sugestões começam desabilitadas.

Para Java, selecionar `javaw.exe` observa qualquer jogo que use **aquele mesmo
caminho**. O monitor não lê argumentos/memória para distinguir mundos ou versões.
O launcher da Store não identifica automaticamente o processo Bedrock. Se o
Windows não fornece o caminho do jogo, o monitor não aplica por nome apenas.

## Eventos: fontes e limites

A consulta local fixa é `__InstanceOperationEvent WITHIN 1`, filtrada para
criação/deleção de `Win32_Process`, apenas da sessão Windows do monitor. Os
eventos fornecem PID, CreationDate e ExecutablePath. Não se habilita SeDebug,
auditoria, provedores ETW ou qualquer política do Windows.

Fontes oficiais: [eventos de criação e intervalo](https://learn.microsoft.com/en-us/windows/win32/wmisdk/--instancecreationevent),
[consulta semissíncrona](https://learn.microsoft.com/en-us/windows/win32/api/wbemcli/nf-wbemcli-iwbemservices-execnotificationquery),
[propriedades de processo e restrições de caminho](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-process),
[bandeja e acessibilidade versão 4](https://learn.microsoft.com/en-us/windows/win32/shell/notification-area).

`WITHIN 1` é intervalo de polling **do provedor WMI**, não promessa de latência
de 1 s. Processos muito curtos podem não produzir par observado. O monitor não
tem handles dos jogos; o provedor do Windows é responsável pela origem dos
dados. Seu custo compartilhado fora da árvore não é atribuído ao monitor pelo
gate e precisa de medição de sistema na VM. A falha de fonte nunca justifica
abrir o jogo por outra API.

## Dados e execução

Biblioteca em `%LOCALAPPDATA%\io.github.nin40301.ametista\games\games.json`, formato
fechado/validado, escrita atômica e lock. Não aceita comandos, alvos Windows ou
IDs fora do catálogo temporário. Caminhos ficam locais; biblioteca/status não
são exportados no benchmark anonimizado. `monitor-status.json` é uma projeção,
nunca prova de autorização. `recovery-pending.json` é só indicação conservadora;
`monitor-stop.json` contém nonce de uma instância para pedir encerramento, nunca
PID a terminar. Uma instância mantém lock; PID antigo não significa “executando”.

```powershell
cargo build --workspace --release --locked --offline
cargo build -p ametista-metrics --example budget --release --locked --offline
./tools/verify-desktop.ps1
./tools/test-resource-budget.ps1 -Mode interface
./tools/test-resource-budget.ps1 -Mode game_monitor
cargo build -p ametista-watcher --example synthetic_game --example wmi_probe --release --locked --offline
./tools/test-game-events.ps1
```

O último comando cria somente nosso processo sintético, espera encerrar e valida
eventos; não chama executores. O gate usa `--readonly`, que desabilita toda
aplicação/recuperação, com WMI e bandeja reais. O binário normal é iniciado pela UI.
Para recuperar ajustes reais/testar sessões, use somente `games-vm.md` na VM.
