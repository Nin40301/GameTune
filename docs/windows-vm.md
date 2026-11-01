# Integração em Windows descartável

Os testes normais nunca aplicam o catálogo real. Os testes administrativos são
ignorados por padrão e só existem com a feature `vm-tests`. Essa feature não deve
ser usada em releases. O runner comum do GitHub apenas compila essa suíte.

O ensaio novo de perfis **reais** é outra suíte, outra feature e outro opt-in:
veja [dois reinícios e smoke tests](windows-real-profiles.md). As restrições de
alvos dedicados deste documento continuam valendo para `windows_vm.rs`.

## Preparar

1. Crie uma VM descartável x64 com Windows e um usuário de teste. Instale Rust
   estável, Git, Windows SDK e Visual Studio Build Tools C++. Use desktop
   interativo; serviço em sessão 0 é rejeitado para não adivinhar o usuário.
2. Execute `cargo fetch --locked` enquanto a VM ainda tem rede. Depois desligue
   a rede da VM. Não instale jogos, contas pessoais ou dados de produção.
3. Desligue a VM e crie um checkpoint/snapshot no hipervisor. Nome sugerido:
   `GameTune-before-phase2`. Reinicie a partir desse estado.
4. Dentro da VM, abra um PowerShell **como administrador**, no checkout. A VM
   precisa oferecer planos de energia clássicos para validar o roundtrip de
   criação e ativação de um plano. Modern Standby pode rejeitar essa operação;
   registre isso como incompatibilidade, sem forçar recursos de energia.

## Executar

```powershell
$env:GAMETUNE_DISPOSABLE_VM = 'I_HAVE_A_SNAPSHOT'
cargo test -p gametune-windows-runtime --features vm-tests --test windows_vm --locked --offline -- --ignored --test-threads=1
```

O opt-in registra que existe um snapshot, não detecta nem cria um snapshot do
hipervisor. Não execute o comando em uma instalação pessoal ou no host.
Não use `--all-features` em uma compilação de produção.

O teste cria somente:

- `HKU\<SID interativo>\Software\GameTuneTest\V<pid>_<nonce>` e valores dedicados
- Serviço `GameTuneTest_V<pid>_<nonce>`, executando um serviço .NET compilado
  pelo teste, com início automático atrasado para verificar a restauração
- Tarefa `\GameTuneTest\GameTuneTest_V<pid>_<nonce>`, com helper próprio e data
  futura, sem executar aplicações reais
- Uma entrada de teste em Run/StartupApproved, identificada pelo mesmo nome,
  apontando para o helper que não executa ações fora do SCM
- Plano clonado com GUID iniciado por `47544e54-`; o único plano que pode ser
  excluído pelo executor no teste é o que ele criou
- `%ProgramData%\GameTuneTestFixtures\<nonce>` e, no teste de ACL, um diretório
  separado `%ProgramData%\GameTuneTestStore\<SID>`

Registro normal, serviço, tarefa e inicialização usam uma allowlist de fixtures
compilada somente com `test-targets`/`vm-tests`. Não há substituição de IDs do
catálogo por caminhos livres e o teste não aplica Copilot, DiagTrack, Xbox ou
qualquer outro alvo real do catálogo.

## Comparação e evidência

Para cada execução, em `crates/windows_runtime/work/windows-vm/`, o teste grava:

- `before.json`: valores, tipos, subchaves e DACL do registro dedicado;
  configuração completa do serviço via registro e estado SCM; XML e DACL da
  tarefa; valor Run/StartupApproved dedicado; inventário e consulta de cada
  plano, incluindo o plano ativo
- `after.json`: o mesmo snapshot após aplicar; os cinco grupos precisam mudar
- `restored.json`: o mesmo snapshot após desfazer; deve ser idêntico a before
- `journal.wal`: original e estados de preparação/aplicação/reversão

O teste verifica que simular mantém o snapshot inteiro idêntico. Fecha e reabre
o journal antes de desfazer. Verifica também que repetir undo não modifica nada.
Não compara PID do serviço, timestamps de auditoria ou eventos do Windows:
restaurar a configuração não apaga logs nem recria uma identidade de processo.

Há tentativa de limpar os recursos criados mesmo se uma asserção falhar. Em
caso de crash, falha de setup ou queda de energia, podem ficar fixtures. **Sempre
restaure o checkpoint do hipervisor ao terminar**, com sucesso ou falha. A limpeza
de arquivos não usa remoção recursiva: só remove helper e marcador conhecidos.
Os relatórios permanecem no checkout para revisão.

## Ponto de restauração e identidade

`WindowsContext::create_restore_point` usa Checkpoint-Computer e retorna Created
ou Recent. O segundo caso só é aceito quando o histórico confirma um ponto nas
últimas 24 horas. O código não altera a frequência nem habilita proteção de
sistema silenciosamente. Proteção indisponível gera erro de preflight e impede
o lote na API `WindowsSession`. Esses caminhos precisam de teste funcional
manual na VM com Proteção do Sistema configurada pelo operador; não são chamados
pela suíte automática para evitar alterações fora dos alvos dedicados.

Valide também, dentro da VM, a elevação usando outra conta administrativa:
compare o SID interativo antes e depois. `WindowsContext::detect` usa WTS na
sessão do processo e resolve localmente o SID do shell canônico dessa sessão.
O fallback usa perfis carregados e recusa ambiguidades. Não consulta domínio
para resolver nomes nem usa o token administrativo como dono de HKCU.
HKCU é resolvido em HKEY_USERS desse SID. Não haverá fallback para o hive da
conta elevada. Sessão sem usuário ou hive não carregado gera erro.

## Matriz a registrar

| Sistema | Home | Pro |
| --- | --- | --- |
| Windows 10 22H2 / 19045 | Pendente | Pendente |
| Windows 11 23H2 / 22631 | Pendente | Pendente |
| Windows 11 24H2 / 26100 e posteriores | Pendente | Pendente |

O host desta entrega executou os testes dedicados de HKCU sem elevação. Isso
não substitui a matriz administrativa. Appx real não é removido nos testes:
o fluxo irreversível é testado com mocks, inventários e falhas injetadas.

O workflow manual `windows-vm.yml` usa exclusivamente runner próprio com label
`gametune-disposable`, iniciado em desktop interativo elevado dentro da VM.
Nunca associe esse label a uma máquina pessoal ou runner de produção. A execução
precisa ser disparada manualmente com a confirmação de snapshot.
