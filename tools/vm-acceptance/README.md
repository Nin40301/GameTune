# Ametista — pacote interno de aceite em VM

**NAO E RELEASE. Nunca execute no PC principal. Altera perfis reais Seguro/Equilibrado e pede dois reinicios.** Use Windows x64 Home/Pro descartavel com desktop interativo, mesma conta administradora, apps Loja/Edge/Configuracoes/Seguranca instalados e Protecao do Sistema funcionando. Nao usa Rust, Git ou Visual Studio na VM. Usa Windows PowerShell 5.1/.NET Framework e componentes Windows ja presentes; CLM/WDAC podem recusar scripts, sem bypass.

1. Confira origem e SHA-256 do ZIP pelo canal de entrega; hashes do manifesto detectam corrupcao, nao substituem assinatura. Extraia em pasta local da VM. Desligue a rede e **crie snapshot do hipervisor antes**. O filtro fabricante/modelo so detecta familias conhecidas de VM, nao prova snapshot.
2. Abra Windows PowerShell como administrador na VM, na mesma conta interativa. A politica local deve permitir o script revisado; o pacote nao muda ExecutionPolicy, nao usa Bypass e nao desliga protecoes.
3. Defina os DOIS opt-ins e execute:

```powershell
$env:AMETISTA_DISPOSABLE_VM = 'I_HAVE_A_SNAPSHOT'
$env:AMETISTA_REAL_PROFILES = 'I_ACCEPT_REAL_CATALOG_CHANGES_AND_REBOOTS'
.\Run-Acceptance.ps1
```

Digite SNAPSHOT quando solicitado. O runner confere VM/admin/hashes, copia o pacote para `%ProgramData%\AmetistaVmAcceptance\package` com escrita apenas Administradores/SYSTEM e grava em `results\` no mesmo root. Executa fixtures dedicadas (registro bruto, servico/tarefa criados pelo teste, StartupApproved e energia), depois perfis reais. A CLI embarcada tem somente features de producao; testes nunca entram no release normal.

Ao avisar **REINICIO NECESSARIO** (codigo 10), reinicie manualmente e entre na mesma conta. Uma tarefa temporaria, somente dessa conta e VM, retoma pelo script protegido apos logon. Nao reinicia, salva senha ou ativa logon automatico. A tarefa e removida ao concluir/falhar/abortar. A retomada usa o consentimento salvo protegido, confere conta/VM/boot e hashes; nao aceita caminhos/comandos livres. Se a janela fechar, confira resultados; para retomar manualmente em terminal elevado:

```powershell
& "$env:ProgramData\AmetistaVmAcceptance\package\Run-Acceptance.ps1" -Resume
```

Codigo 0 so indica comparacoes automatizadas concluidas; confira visualmente os quatro apps e Minecraft Launcher/Bedrock e preencha a matriz Home/Pro. Codigo 10 pede boot diferente. Outra falha interrompe sem reaplicar cegamente. Preserve `results\` (logs, snapshots completos e estado) e WAL; podem conter caminhos/SID e **nao devem ser publicados sem revisao**. O script nao coleta nem envia pela rede.

Para tentar desfazer explicitamente apos falha, ainda na mesma VM/conta e com snapshot preservado:

```powershell
& "$env:ProgramData\AmetistaVmAcceptance\package\Run-Acceptance.ps1" -Recover
```

Recuperar marca **abortado**, nunca aprovado. Em conflito, restaure o snapshot. **Restaure o snapshot ao terminar, mesmo com sucesso**: undo nao apaga eventos/auditoria/apps abertos. Nao apague journals para forcar aceite. `-CheckOnly` confere VM/admin/hashes sem iniciar testes/tarefas. O pacote foi compilado e inspecionado, mas o ensaio real com reboot permanece pendente ate executado na VM.
