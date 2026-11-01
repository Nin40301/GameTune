# GameTune

Aplicativo offline de ajustes do Windows para jogos. **Fase 4: UI Tauri 2,
React/TypeScript strict e Tailwind implementada**, sobre o mesmo backend da CLI.
Aceite UAC/escritas/reinícios na matriz de VM ainda pendente. Licença Apache-2.0.

Leia [como compilar e usar a interface](docs/desktop.md),
[limite de privilégios e IPC](docs/desktop-security.md) e
[relatório da Fase 4](docs/phase-4.md).

## O que está implementado

- Núcleo Rust com IDs fechados, simulação padrão, importação de perfis e WAL
- Registro Windows com tipo/bytes originais e remoção segura de chaves criadas
- Serviços com início, DelayedAutoStart, execução e espera limitada, sem cascata
- Tarefas via COM com habilitação, XML original e detecção de alterações externas
- Energia com plano anterior, inventário, criação quando suportada e undo
- Inicialização via StartupApproved, com IDs por entrada descoberta e snapshot
- Identidade interativa local, HKCU em HKEY_USERS e escopo máquina/SID no journal
- ProgramData protegido, bloqueio global e exclusividade de originais entre usuários
- Ponto de restauração antes de aplicar na API WindowsSession, com limite diário
- Appx **manual separado** com consentimento por app, aviso de ausência de undo,
  inventário AllUsers/provisionado, registro prévio de nome/PFN e reinstalação Store
- `gametune-cli`: análise real sem admin, simulação padrão, JSON/pt-BR/en,
  aplicação confirmada, histórico/desfazer e comandos Appx separados
- Interface com Início, Itens, Revisão, Limpeza de apps, Histórico e Configurações;
  pt-BR/en, claro/escuro, teclado, detalhes, consentimentos e testes axe
- Auxiliar requireAdministrator de uso único; UI asInvoker e pipe local com
  DACL por SID, autenticação de PID/SID, nonce e rejeição de replay/payloads livres
- Análise em background, progresso/cancelamento, quatro workers e inventário
  Appx compartilhado apenas na análise; toda escrita recaptura o estado

## Executar a CLI

```powershell
cargo build -p gametune-cli --locked --offline
target/debug/gametune-cli.exe --profile safe --dry-run --json
target/debug/gametune-cli.exe --list
target/debug/gametune-cli.exe --help --lang en
```

Sem flags é somente leitura. Nenhum ajuste é aplicado sem `--apply --confirm`.
Leia [comandos, Appx e códigos de saída](docs/cli.md) e o
[ensaio com dois reinícios em VM](docs/windows-real-profiles.md).

O catálogo tem 86 descritores e 68 identidades/mapeamentos documentados, incluindo
58 dos 73 itens de nível 1/2. `verified` não significa ganho medido nem recomendação
para jogos. Veja [fontes e limites por item](docs/catalog-sources.md).

## Verificar sem alterar configurações pessoais

Requisitos: Rust estável (testado com 1.98.1), Git e, no Windows, Visual Studio
Build Tools C++ e Windows SDK. Não eleve o terminal para os testes normais.

```powershell
cargo fetch --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo test --workspace --locked --offline
cargo run -p gametune-catalog --example simulate --locked --offline
cargo run -p gametune-catalog --example removed_apps --locked --offline -- outputs/appx-reinstall-demo.html
```

Os exemplos usam mocks, não removem apps e não aplicam o catálogo. O segundo gera
um HTML offline com o aviso e botões de reinstalação. Só clicar no link abre a
Microsoft Store. Os dois testes de integração reais de HKCU agora são ignorados
por padrão e exigem opt-in de VM. Os testes normais não escrevem configurações
do Windows; testes IPC criam apenas pipes efêmeros sem lançar o helper/UAC.

Para compilar, sem executar, a suíte administrativa:

```powershell
cargo test -p gametune-windows-runtime --features vm-tests --locked --offline --no-run
```

Para executá-la, siga [Windows em VM descartável](docs/windows-vm.md). Não use
`--all-features` em releases. O CI comum testa Linux/Windows e só compila os
testes administrativos. Um workflow manual separado aceita runner de VM dedicado.

## Estrutura

```text
catalog/                   catálogo, schemas e hashes embutidos
crates/core/               domínio, motor, WAL, perfis, remoção manual Appx
crates/catalog/            carregador e exemplos sem efeitos reais
crates/exec_mock/           estado fictício e falhas injetadas
crates/exec_{tipo}/         seis adaptadores Windows e mocks
crates/windows_runtime/    identidade, armazenamento, APIs e scripts fixos
  scripts/                 código confiável embutido, parâmetros JSON por stdin
  tests/windows_vm.rs      roundtrip de fixtures, ignorado por padrão
  tests/real_profiles_vm.rs aceite real após dois reinícios, opt-in adicional
crates/cli/                binário de produção, parser fechado e testes
crates/ipc/                protocolo tipado, pipe Windows, FFI e validação do helper
crates/helper/             auxiliar de uma transação com manifesto elevado
crates/desktop_backend/    análise em background e tickets de revisão
apps/desktop/              React/TS/Tailwind, componentes/e2e/axe e host Tauri
locales/                   pt-BR e en para CLI, avisos e botões Appx
tools/                     manutenção de hashes (parser PowerShell nos testes Rust)
docs/                      arquitetura, fontes, segurança, fases e VM
.github/workflows/         CI portátil e integração manual em VM
```

## API compartilhada

`WindowsSession::connect` identifica a sessão e monta o catálogo com entradas
permitidas de inicialização. `simulate` aceita Request com IDs. `apply` exige
elevação, armazenamento protegido e ponto de restauração; `undo` restaura só as
configurações reversíveis. `remove_app` aceita AppRemovalConsent separado;
`reconcile_app_removal` resolve tentativas interrompidas pelo inventário atual.
Nenhum desses métodos recebe código, serviço, comando ou caminho livre da UI.

CLI e UI usam essa API. A UI permanece asInvoker e chama um auxiliar elevado
de uso único após revisão. O IPC aceita somente comandos tipados e IDs. O frontend
não acessa executores diretamente. Ler histórico protegido também pode exigir
autorização explícita, sem abrir a ACL do WAL ao usuário comum.

`preview_app_removal` informa explicitamente `allUsers: false` quando o inventário
é apenas do usuário corrente. O Windows exige elevação para AllUsers/provisionado;
a remoção recaptura esse inventário completo após elevar. O consentimento por app
avisa que a remoção afeta todos os usuários e novos perfis.

Estado de produção:

```text
%ProgramData%\GameTune\machine.lock
%ProgramData%\GameTune\scope-owner.json
%ProgramData%\GameTune\<SID interativo>\journal.wal
%ProgramData%\GameTune\<SID interativo>\journal.json
%ProgramData%\GameTune\<SID interativo>\removed-apps.json
```

Diretórios, arquivos e temporários herdam acesso somente a Administradores/SYSTEM.
O token elevado precisa ter proprietário padrão Administradores/SYSTEM. Um
segundo usuário não pode iniciar alterações globais enquanto o primeiro tiver
entradas reversíveis ativas. Desfazer não apaga registros de auditoria.

## Limites relevantes

- Appx não é automaticamente reversível. Reinstalar pela Store não recupera
  dados nem garante versão, disponibilidade ou reprovisionamento. GamingServices
  e XboxIdentityProvider exigem confirmação reforçada e nunca entram em presets.
- Home exclui políticas de Windows sem suporte documental. Preferências de
  usuário documentadas apenas para leitura continuam manuais até validação funcional.
- Recall está bloqueado: a política pode apagar snapshots, que um DWORD restaurado
  não recupera. SysMain e serviços desaconselhados pela fonte ficam só manuais.
- Não são alterados Defender, Update (exceto o alvo permitido de Delivery
  Optimization), UAC, SmartScreen, firewall, TPM/Secure Boot, VBS/HVCI ou drivers.
- Modern Standby e algumas GPUs/builds podem rejeitar ajustes. O programa registra
  erro e preserva o original; não restaura todos os planos nem força compatibilidade.
- Ainda faltam testes administrativos na matriz Home/Pro, teste UAC com outra conta,
  Appx real, corte de energia, revisão manual com leitor de tela, métricas,
  watcher, assinatura e instalador. O fluxo de elevação/identidade existe, mas
  seu roundtrip com escrita e outra conta ainda precisa de VM.
- Sem promessas de FPS, limpeza de registro, esvaziamento de working sets ou handles
  em jogos. A identificação lê apenas metadados da sessão e do shell canônico.

Leia [resumo da Fase 3](docs/phase-3.md), [Fase 2 corrigida](docs/phase-2.md), [journal](docs/journal.md),
[segurança](docs/security.md) e [manutenção do catálogo](docs/catalog.md).

Assinatura/instaladores entram na Fase 7. Certificado e senha serão segredos
`WINDOWS_CERTIFICATE_BASE64` e `WINDOWS_CERTIFICATE_PASSWORD`; nenhum certificado
é usado pelo pipeline atual. Veja [LICENSE](LICENSE) e [NOTICE](NOTICE).
