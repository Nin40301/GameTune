# Ensaio de VM: perfis reais, reinício e desfazer

Para uma VM sem Rust/Git/Visual Studio, use o [pacote interno de aceite](vm-acceptance-kit.md). Ele inclui runner, teste final e CLI de produção como arquivos irmãos; o caminho é fixo pelo executável, nunca parâmetro livre. O roteiro abaixo continua disponível para desenvolvimento.

Este ensaio **altera alvos reais do catálogo Seguro e Equilibrado**. É separado
dos testes de fixtures de `windows-vm.md` e exige um segundo opt-in. Nunca rode
no host nem em Windows pessoal. Um filtro de fabricante/modelo recusa máquinas
fora das famílias de VM conhecidas, mas não comprova a existência de snapshot.

Use VM x64 descartável Home/Pro, usuário local sem dados pessoais, desktop
interativo, proteção do sistema/pontos de restauração funcionando, planos de
energia clássicos e os quatro apps instalados. O baseline exige que já abram e
que Defender esteja ativo; não tenta consertar pré-requisitos nem muda segurança.
Instale dependências, desligue a rede e crie snapshot do hipervisor **antes**.
Use o mesmo checkout e a mesma conta/sessão interativa em todas as etapas.

## Preparar e iniciar

PowerShell elevado **dentro da VM**:

```powershell
cargo fetch --locked
# Binário de produção com features padrão, sem autorização de fixtures
cargo build -p ametista-cli --locked --offline
cargo build -p ametista-windows-runtime --example real_profile_e2e --features real-profile-tests --locked --offline
$env:AMETISTA_DISPOSABLE_VM = 'I_HAVE_A_SNAPSHOT'
$env:AMETISTA_REAL_PROFILES = 'I_ACCEPT_REAL_CATALOG_CHANGES_AND_REBOOTS'
target/debug/examples/real_profile_e2e.exe step
```

Não use `--all-features` para compilar a CLI. O harness invoca exatamente
`target/debug/ametista-cli.exe` com perfis reais e flags fixas; não injeta alvos de
teste, não remove Appx e não exporta scripts da UI. Exige histórico sem alterações
ativas. É compilado apenas com `real-profile-tests`; o release padrão o exclui.

1. O primeiro `step` verifica os quatro apps e Defender, captura snapshot do
   perfil Seguro, grava estado `applying` antes da execução, aplica pela CLI,
   salva o estado após aplicar e retorna **10 = reinício necessário**.
2. Reinicie manualmente pelo Windows. Após entrar na mesma conta, abra terminal
   elevado, redefina as duas variáveis e execute `...real_profile_e2e.exe step`.
   O teste exige boot diferente, abre/verifica os apps, confirma ajustes
   persistentes, desfaz Seguro pela CLI e compara snapshot completo. Em seguida
   captura/aplica Equilibrado e retorna 10 novamente.
3. Reinicie outra vez. Redefina as variáveis e execute `step` pela terceira vez.
   Verifica apps/ajustes, desfaz Equilibrado e compara snapshot; só então retorna
   0 com estado `complete`.
4. Execute o teste de aceite:

```powershell
cargo test -p ametista-windows-runtime --features real-profile-tests --test real_profiles_vm --locked --offline -- --ignored --test-threads=1
```

Executar `step` sem reiniciar retorna 10 e não reaplica. O teste de aceite falha
se faltarem etapas, qualquer app, dois boots distintos ou comparação de undo.
Não basta compilar o harness para declarar o ensaio aprovado. O workflow comum
e o workflow de fixtures não executam esse ensaio. Não use `-- --ignored` sem
selecionar explicitamente a suíte desejada.

## Evidência e comparação

`%ProgramData%\AmetistaRealProfileTests\<SID>`, protegido para Administradores e
SYSTEM, guarda estado retomável e, por perfil:

- `*-before.json`, `*-after-apply.json`, `*-after-reboot.json`, `*-restored.json`
- `*-apply.json`, com resultado real da CLI
- `state.json`, com boot anterior/posterior, smoke tests e igualdade dos snapshots

Snapshot inclui valor/tipo/ausência e chaves criadas do executor; valores,
subchaves e ACL da chave de política; configuração completa do serviço pelo
registro, início, execução, conta, caminho, tipo e error control; XML,
habilitação e DACL das tarefas; todos os planos com conteúdo e plano ativo.
Compara estrutura e valores sem depender da ordem das propriedades JSON. Não
inclui PID/uptime, eventos, auditoria, estado transitório do sistema inteiro ou
arquivos pessoais. Apps e StartupApproved não entram em presets automáticos;
o roundtrip de StartupApproved continua coberto na suíte de fixtures dedicada.
Qualquer drift de configuração selecionada após o reinício falha o ensaio.

O smoke test ativa `ms-windows-store://home`, `ms-settings:about`,
`windowsdefender:` e Edge em `about:blank` pelo desktop interativo. Confirma
processo com caminho Windows esperado e SID/sessão corretos, janela visível e
responsiva (incluindo filhos de ApplicationFrameHost no Windows 10), e estado
de serviço/antivírus/proteção em tempo real via Get-MpComputerStatus somente
leitura. FFI fixa inspeciona HWNDs e PIDs dos apps conhecidos; não abre handles
de processos nem interage com jogos. Não encerra apps para forçar o resultado.

Isso cobre ativação local e resposta da interface, **não** compras/downloads na
Loja, navegação de rede, todas as páginas de Configurações ou execução de jogos.
Confirme visualmente que o conteúdo carregou, registre screenshots no relatório
da VM e acrescente Minecraft Launcher/Bedrock na matriz final. Um app com janela
responsiva mas conteúdo quebrado exige reprovação manual, não aceite automático.

## Falhas e recuperação

Falha após aplicar preserva WAL e estado `applying`/`await_reboot`. Não repete
aplicação cegamente. Antes de qualquer tentativa de recovery, guarde a evidência.

```powershell
target/debug/examples/real_profile_e2e.exe recover
```

Esse comando tenta `--undo --apply --confirm` e marca o ensaio abortado, sem
declará-lo aprovado. Em conflito, use a evidência para diagnosticar e restaure o
snapshot do hipervisor. **Sempre restaure o snapshot ao terminar**, mesmo com
sucesso: auditoria do Windows e estado dos apps abertos não são apagados pelo undo.

Repita em Windows 10 22H2 e Windows 11 23H2/24H2+, Home/Pro. A execução desta
matriz ainda está pendente. Nenhum perfil real foi aplicado no host da entrega.

Fontes oficiais para os mecanismos:
[Task Scheduler GetFolder](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskfolder-getfolder),
[ativação de apps](https://learn.microsoft.com/en-us/windows/apps/develop/launch/launch-default-app),
[URIs de Configurações](https://learn.microsoft.com/en-us/windows/apps/develop/launch/launch-settings),
[URIs da Loja](https://learn.microsoft.com/en-us/windows/apps/develop/launch/launch-store-app),
[execução pelo Explorer](https://learn.microsoft.com/en-us/windows/win32/shell/samples-execinexplorer).
