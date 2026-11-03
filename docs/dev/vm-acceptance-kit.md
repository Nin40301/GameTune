# Pacote interno autossuficiente de aceite

O pacote é **somente para VM, nunca release**. Na máquina de desenvolvimento Windows com ferramentas fixadas e dependências já baixadas:

```powershell
pwsh -NoProfile -File tools/build-vm-acceptance.ps1 -Destination outputs/vm-acceptance-local
node tools/verify-vm-package.mjs outputs/vm-acceptance-local
Get-FileHash -Algorithm SHA256 outputs/vm-acceptance-local.zip
```

Gera cinco executáveis x64 com CRT estática, manifesto de nove arquivos e hashes, runner Windows PowerShell 5.1 e [README para a VM](../../tools/vm-acceptance/README.md). A compilação usa target isolado em work, duas invocações explícitas de features de teste e CLI separada com features padrão. Verificação PE recusa dependência do runtime redistribuível MSVC; --list só enumera os cinco testes, não os executa.

Transfira o ZIP e seu SHA-256 para a VM pelo canal de teste; **a VM não precisa de Rust, Git ou Visual Studio**. Windows/.NET Framework/PowerShell locais são necessários; Add-Type de fixture/smoke usa compilação fornecida pelo Windows e pode ser bloqueado por CLM/WDAC. Não há bypass, download ou mudança de política/segurança. Runner de CI Server só compila/inspeciona: não aprova a matriz cliente Home/Pro.

O runner recusa fabricante/modelo fora das famílias de VM e exige administrador em sessão interativa. Antes da primeira escrita exige dois opt-ins e confirmação literal de snapshot. Snapshot não pode ser provado pelo filtro. Copia somente arquivos fixos para ProgramData com ACL Admin/SYSTEM; grava estado antes de executar, usa lock exclusivo e tarefa de logon temporária na mesma conta. Retomada verifica conta/UUID/boot, ACLs, hashes e consentimento protegido; não usa senha/autologon. Reinícios são manuais. Falha/crash não repete aplicação: exige recovery explícito ou snapshot. Um erro ao instalar a tarefa ainda permite retomada manual pelo roteiro; nunca mudar segurança para fazê-la rodar.

Em `results/` ficam logs dos executáveis, snapshots dedicados sob `work/windows-vm`, cópias da evidência de perfis reais e estado do runner. Logs podem conter SID/caminhos: revisar antes de compartilhar. O script remove somente a tarefa criada por ele ao finalizar/abortar/falhar; conserva pacote/evidências/WAL. Restaurar snapshot ao terminar, mesmo com sucesso.

CI Windows publica artefato temporário **INTERNAL-VM-ACCEPTANCE-NOT-A-RELEASE**, com sete dias de retenção. O workflow de release não o anexa aos assets. A geração local e os hashes foram conferidos; execução administrativa, tarefa/ACL e dois reboots **ainda não foram validados** nesta entrega. [Roteiro dos perfis reais](windows-real-profiles.md) · [fixtures](windows-vm.md).
