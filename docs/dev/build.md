> Desenvolvimento: use VM descartÃ¡vel com snapshot para qualquer escrita administrativa. Alfa; nÃ£o use no PC principal.

# Guia tÃ©cnico e build â€” Ametista

Aplicativo offline de ajustes do Windows para jogos. **Ametista 0.6.5 alfa**,
com interface, monitor Rust independente e perfis temporÃ¡rios por jogo.
Aceite UAC/escritas/reinÃ­cios na matriz de VM ainda pendente. LicenÃ§a Apache-2.0.
OrÃ§amentos separados: interface com linha de base de cinco execuÃ§Ãµes, regressÃ£o
das medianas de RAM/CPU >15% e teto mediano de RAM de 200 MiB; p95 como alerta. O monitor mantÃ©m
picos de RAM <=15 MiB e CPU <0,5%. Veja [polÃ­tica e mediÃ§Ãµes](../performance.md).

Leia [como compilar e usar a interface](../user/desktop.md),
[limite de privilÃ©gios e IPC](../security/desktop.md) e
[protocolo de benchmark](../user/benchmark.md), [jogos e monitor](../user/games.md) e
[histÃ³rico tÃ©cnico](../history/phase-6.md).

## O que estÃ¡ implementado

- NÃºcleo Rust com IDs fechados, simulaÃ§Ã£o padrÃ£o, importaÃ§Ã£o de perfis e WAL
- Registro Windows com tipo/bytes originais e remoÃ§Ã£o segura de chaves criadas
- ServiÃ§os com inÃ­cio, DelayedAutoStart, execuÃ§Ã£o e espera limitada, sem cascata
- Tarefas via COM com habilitaÃ§Ã£o, XML original e detecÃ§Ã£o de alteraÃ§Ãµes externas
- Energia com plano anterior, inventÃ¡rio, criaÃ§Ã£o quando suportada e undo
- InicializaÃ§Ã£o via StartupApproved, com IDs por entrada descoberta e snapshot
- Identidade interativa local, HKCU em HKEY_USERS e escopo mÃ¡quina/SID no journal
- ProgramData protegido, bloqueio global e exclusividade de originais entre usuÃ¡rios
- Ponto de restauraÃ§Ã£o antes de aplicar na API WindowsSession, com limite diÃ¡rio
- Appx **manual separado** com consentimento por app, aviso de ausÃªncia de undo,
  inventÃ¡rio AllUsers/provisionado, registro prÃ©vio de nome/PFN e reinstalaÃ§Ã£o Store
- `ametista-cli`: anÃ¡lise real sem admin, simulaÃ§Ã£o padrÃ£o, JSON/pt-BR/en,
  aplicaÃ§Ã£o confirmada, histÃ³rico/desfazer e comandos Appx separados
- Interface com InÃ­cio, Itens, RevisÃ£o, Limpeza de apps, HistÃ³rico e ConfiguraÃ§Ãµes;
  pt-BR/en, claro/escuro, teclado, detalhes, consentimentos e testes axe
- Auxiliar requireAdministrator de uso Ãºnico; UI asInvoker e pipe local com
  DACL por SID, autenticaÃ§Ã£o de PID/SID, nonce e rejeiÃ§Ã£o de replay/payloads livres
- AnÃ¡lise em background, progresso/cancelamento, quatro workers e inventÃ¡rio
  Appx compartilhado apenas na anÃ¡lise; toda escrita recaptura o estado
- Painel com RAM/CPU/processos medidos, top 12 agrupado e consumo incluindo WebView2
- Benchmark com 5 Ã— 60 s por condiÃ§Ã£o, descarte do aquecimento, retomada entre
  reinÃ­cios, plano de energia constante, mediana/p5/p95/MAD e conclusÃ£o inconclusiva
- ImportaÃ§Ã£o opcional de CSV PresentMon v1, sem executar ferramenta nem abrir jogos
- ExportaÃ§Ã£o JSON/HTML offline, anonimizaÃ§Ã£o padrÃ£o e gate real de RAM/CPU
- Monitor Rust asInvoker separado, bandeja, inÃ­cio sob demanda e eventos WMI
- Descoberta por configuraÃ§Ãµes locais dos launchers e adiÃ§Ã£o manual de `.exe`
- Perfis temporÃ¡rios fechados, serviÃ§os apenas pausados, uniÃ£o entre jogos,
  journal prÃ³prio, rollback no fim/EOF e recuperaÃ§Ã£o apÃ³s crash/reboot

## Executar a CLI

```powershell
cargo build -p ametista-cli --locked --offline
target/debug/ametista-cli.exe --profile safe --dry-run --json
target/debug/ametista-cli.exe --list
target/debug/ametista-cli.exe --help --lang en
```

Sem flags Ã© somente leitura. Nenhum ajuste Ã© aplicado sem `--apply --confirm`.
Leia [comandos, Appx e cÃ³digos de saÃ­da](../user/cli.md) e o
[ensaio com dois reinÃ­cios em VM](windows-real-profiles.md).

O catÃ¡logo tem 86 descritores e 68 identidades/mapeamentos documentados, incluindo
58 dos 73 itens de nÃ­vel 1/2. `verified` nÃ£o significa ganho medido nem recomendaÃ§Ã£o
para jogos. Veja [fontes e limites por item](catalog-sources.md).

## Verificar sem alterar configuraÃ§Ãµes pessoais

Requisitos: Rust estÃ¡vel (testado com 1.98.1), Git e, no Windows, Visual Studio
Build Tools C++ e Windows SDK. NÃ£o eleve o terminal para os testes normais.

```powershell
cargo fetch --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo test --workspace --locked --offline
cargo run -p ametista-catalog --example simulate --locked --offline
cargo run -p ametista-catalog --example removed_apps --locked --offline -- outputs/appx-reinstall-demo.html
```

Os exemplos usam mocks, nÃ£o removem apps e nÃ£o aplicam o catÃ¡logo. O segundo gera
um HTML offline com o aviso e botÃµes de reinstalaÃ§Ã£o. SÃ³ clicar no link abre a
Microsoft Store. Os dois testes de integraÃ§Ã£o reais de HKCU agora sÃ£o ignorados
por padrÃ£o e exigem opt-in de VM. Os testes normais nÃ£o escrevem configuraÃ§Ãµes
do Windows; testes IPC criam apenas pipes efÃªmeros sem lanÃ§ar o helper/UAC.

Para compilar, sem executar, a suÃ­te administrativa:

```powershell
cargo test -p ametista-windows-runtime --features vm-tests --locked --offline --no-run
```

Para executÃ¡-la, siga [Windows em VM descartÃ¡vel](windows-vm.md). NÃ£o use
`--all-features` em releases. O CI comum testa Linux/Windows e sÃ³ compila os
testes administrativos. Um workflow manual separado aceita runner de VM dedicado.

## Estrutura

```text
catalog/                   catÃ¡logo, schemas e hashes embutidos
crates/core/               domÃ­nio, motor, WAL, perfis, remoÃ§Ã£o manual Appx
crates/catalog/            carregador e exemplos sem efeitos reais
crates/exec_mock/           estado fictÃ­cio e falhas injetadas
crates/exec_{tipo}/         seis adaptadores Windows e mocks
crates/windows_runtime/    identidade, armazenamento, APIs e scripts fixos
  scripts/                 cÃ³digo confiÃ¡vel embutido, parÃ¢metros JSON por stdin
  tests/windows_vm.rs      roundtrip de fixtures, ignorado por padrÃ£o
  tests/real_profiles_vm.rs aceite real apÃ³s dois reinÃ­cios, opt-in adicional
crates/cli/                binÃ¡rio de produÃ§Ã£o, parser fechado e testes
crates/ipc/                protocolo tipado, pipe Windows, FFI e validaÃ§Ã£o do helper
crates/helper/             auxiliar de uma transaÃ§Ã£o com manifesto elevado
crates/desktop_backend/    anÃ¡lise em background e tickets de revisÃ£o
apps/desktop/              React/TS/Tailwind, componentes/e2e/axe e host Tauri
locales/                   pt-BR e en para CLI, avisos e botÃµes Appx
tools/                     manutenÃ§Ã£o de hashes (parser PowerShell nos testes Rust)
docs/                      arquitetura, fontes, seguranÃ§a, fases e VM
.github/workflows/         CI portÃ¡til e integraÃ§Ã£o manual em VM
```

## API compartilhada

`WindowsSession::connect` identifica a sessÃ£o e monta o catÃ¡logo com entradas
permitidas de inicializaÃ§Ã£o. `simulate` aceita Request com IDs. `apply` exige
elevaÃ§Ã£o, armazenamento protegido e ponto de restauraÃ§Ã£o; `undo` restaura sÃ³ as
configuraÃ§Ãµes reversÃ­veis. `remove_app` aceita AppRemovalConsent separado;
`reconcile_app_removal` resolve tentativas interrompidas pelo inventÃ¡rio atual.
Nenhum desses mÃ©todos recebe cÃ³digo, serviÃ§o, comando ou caminho livre da UI.

CLI e UI usam essa API. A UI permanece asInvoker e chama um auxiliar elevado
de uso Ãºnico apÃ³s revisÃ£o. O IPC aceita somente comandos tipados e IDs. O frontend
nÃ£o acessa executores diretamente. Ler histÃ³rico protegido tambÃ©m pode exigir
autorizaÃ§Ã£o explÃ­cita, sem abrir a ACL do WAL ao usuÃ¡rio comum.

`preview_app_removal` informa explicitamente `allUsers: false` quando o inventÃ¡rio
Ã© apenas do usuÃ¡rio corrente. O Windows exige elevaÃ§Ã£o para AllUsers/provisionado;
a remoÃ§Ã£o recaptura esse inventÃ¡rio completo apÃ³s elevar. O consentimento por app
avisa que a remoÃ§Ã£o afeta todos os usuÃ¡rios e novos perfis.

Estado de produÃ§Ã£o:

```text
%ProgramData%\Ametista\machine.lock
%ProgramData%\Ametista\scope-owner.json
%ProgramData%\Ametista\<SID interativo>\journal.wal
%ProgramData%\Ametista\<SID interativo>\journal.json
%ProgramData%\Ametista\<SID interativo>\removed-apps.json
```

DiretÃ³rios, arquivos e temporÃ¡rios herdam acesso somente a Administradores/SYSTEM.
O token elevado precisa ter proprietÃ¡rio padrÃ£o Administradores/SYSTEM. Um
segundo usuÃ¡rio nÃ£o pode iniciar alteraÃ§Ãµes globais enquanto o primeiro tiver
entradas reversÃ­veis ativas. Desfazer nÃ£o apaga registros de auditoria.

## Limites relevantes

- Appx nÃ£o Ã© automaticamente reversÃ­vel. Reinstalar pela Store nÃ£o recupera
  dados nem garante versÃ£o, disponibilidade ou reprovisionamento. GamingServices
  e XboxIdentityProvider exigem confirmaÃ§Ã£o reforÃ§ada e nunca entram em presets.
- Home exclui polÃ­ticas de Windows sem suporte documental. PreferÃªncias de
  usuÃ¡rio documentadas apenas para leitura continuam manuais atÃ© validaÃ§Ã£o funcional.
- Recall estÃ¡ bloqueado: a polÃ­tica pode apagar snapshots, que um DWORD restaurado
  nÃ£o recupera. SysMain e serviÃ§os desaconselhados pela fonte ficam sÃ³ manuais.
- NÃ£o sÃ£o alterados Defender, Update (exceto o alvo permitido de Delivery
  Optimization), UAC, SmartScreen, firewall, TPM/Secure Boot, VBS/HVCI ou drivers.
- Modern Standby e algumas GPUs/builds podem rejeitar ajustes. O programa registra
  erro e preserva o original; nÃ£o restaura todos os planos nem forÃ§a compatibilidade.
- Ainda faltam testes administrativos na matriz Home/Pro, teste UAC com outra conta,
  Appx real, corte de energia, revisÃ£o manual com leitor de tela, mÃ©tricas,
  watcher, assinatura e instalador. O fluxo de elevaÃ§Ã£o/identidade existe, mas
  seu roundtrip com escrita e outra conta ainda precisa de VM.
- Sem promessas de FPS, limpeza de registro, esvaziamento de working sets ou handles
  em jogos. A identificaÃ§Ã£o lÃª apenas metadados da sessÃ£o e do shell canÃ´nico.

Leia [resumo da Fase 3](../history/phase-3.md), [Fase 2 corrigida](../history/phase-2.md), [journal](journal.md),
[seguranÃ§a](../security/model.md) e [manutenÃ§Ã£o do catÃ¡logo](catalog.md).

Assinatura/instaladores entram na Fase 7. Certificado e senha serÃ£o segredos
`WINDOWS_CERTIFICATE_BASE64` e `WINDOWS_CERTIFICATE_PASSWORD`; nenhum certificado
Ã© usado pelo pipeline atual. Veja [LICENSE](../../LICENSE) e [NOTICE](../../NOTICE).

## Interface

A marca e os binÃ¡rios usam `Ametista`/`ametista`. Tema claro e efeitos completos sÃ£o o padrÃ£o para novas preferÃªncias, com respeito Ã  transparÃªncia reduzida. O titular aceitou a diferenÃ§a de memÃ³ria para fechar a 6.5. A [polÃ­tica atual](../performance.md) usa cinco execuÃ§Ãµes por modo: Completo passa; Reduzido excede o teto mediano de 200 MiB em 5,50 MiB neste host. O monitor continua estrito e passou. A investigaÃ§Ã£o de GPU/WebView foi encerrada; os resultados antigos de pico permanecem como histÃ³rico, sem representar o gate vigente.
