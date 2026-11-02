# Dependência de PowerShell

## Migração do Bloco E

As duas rotas escolhidas foram **identidade** (antes compilava C# a cada conexão) e **serviços** (várias invocações por análise). WTS/token próprio/pasta conhecida e leitura de hives locais agora usam `windows-sys` + `winreg`; serviços usam SCM com direitos mínimos. Não há WTSQueryUserToken, consulta de conta em domínio, handle em Explorer/jogo, cascata de parada ou encerramento de processos de terceiros. Se o SID interativo não puder ser determinado unicamente, a operação recusa continuar. HKCU continua sendo HKEY_USERS\SID interativo.

Uma execução somente leitura do CLI, mesmo host e catálogo, antes/depois da migração: **4,6120165 s → 2,1937551 s**, 88 linhas JSON idênticas. Build excluído do cronômetro. É uma comparação exploratória de uma execução, com cache/host não controlados; não constitui promessa de desempenho. Os testes de roundtrip existentes continuam compilados, mas escrita/serviço/energia reais exigem a VM e ainda não foram executados neste host.

| Script | Add-Type | Situação e migração possível |
| --- | --- | --- |
| identity.ps1 | Sim, WTS | Retirado do enum de scripts executáveis; substituído por Rust/WTS, token próprio, hive local e known folder |
| service.ps1 | Não | Retirado do enum; substituído por SCM. Preserva Start, ausência/valor exato de DelayedAutoStart e estado Running/Stopped; recusa estados transitórios/dependências ativas |
| desktop.ps1 | Sim, GetWindowThreadProcessId | Só é invocado para abrir Store/diálogo de encerramento no desktop correto. Próxima migração: COM shell com PID/SID via metadados, sem handle em processo |
| power.ps1 | Não; powercfg fixo | Próxima migração: PowerGetActiveScheme/PowerEnumerate/PowerDuplicateScheme/PowerSetActiveScheme/PowerDeleteScheme, preservando GUIDs e snapshot |
| appx.ps1 | Não | Manter cmdlets Appx oficiais, parâmetros JSON, nome do catálogo e consentimento individual |
| appx_inventory.ps1 | Não | Manter inventário em lote, somente leitura e cancelável; distinguir usuário atual de todos os usuários |
| task.ps1 | Não | Migrável para Task Scheduler COM; preservar XML completo e Enabled sem registrar novamente a tarefa |
| storage.ps1 | Não | Migrável para ACL Win32; manter dono Administradores/SYSTEM, recusa de reparse/hardlink e teste de permissões |
| restore_point.ps1 | Não | Manter Checkpoint-Computer/Get-ComputerRestorePoint por enquanto. Migração SRSetRestorePoint exige validar limite diário e criação real |
| store.ps1 | Via desktop.ps1 | Abre somente URI Store criada do PFN registrado/validado; não reinstala automaticamente |
| prelude.ps1 | Não | Envelope, serialização e validações fixas compartilhadas; sem entrada de texto de comando |
| catch.ps1 | Não | Mapeia erros, não devolve mensagens pessoais para logs/diagnóstico |
| parse.ps1 | Não (compila helpers recebidos do próprio teste) | Exclusivo de teste de sintaxe/fontes embutidas, sem origem UI |
| fixture.ps1 | Sim, serviço sintético | Exclusivo de VM opt-in; não distribuir autorização de fixture no release |
| real_profiles.ps1 | Sim, sondagem de janelas | Exclusivo de VM opt-in com snapshot/reinício; não habilitar no host |
| RestartDialog (fonte fixa embutida) | Via desktop.ps1 | Só abre `Shell.Application.ShutdownWindows` no desktop conferido. A pessoa decide/cancela no Windows; nenhum shutdown automático |

## Constrained Language Mode

As duas rotas migradas não iniciam PowerShell. Appx, tarefas, criação de ponto, abertura da Store/diálogo e proteção do armazenamento ainda podem falhar com CLM/políticas corporativas. O app não altera ExecutionPolicy, não tenta escapar de CLM e não enfraquece antivírus/WDAC. A falha é registrada e o lote não vira aprovação. Ensaio específico com CLM continua pendente na VM.

`windows-sys 0.61` já era usado pelo IPC/monitor; o runtime habilita apenas features necessárias às APIs migradas. `sha2` existente verifica o auxiliar. `zip 9.2.2`, sem features de compressão, é a única biblioteca nova de runtime do suporte: evita implementar um formato de arquivo próprio e exporta um ZIP pequeno, offline, por allowlist.

## Fontes primárias

- [WTSQuerySessionInformationW](https://learn.microsoft.com/en-us/windows/win32/api/wtsapi32/nf-wtsapi32-wtsquerysessioninformationw) e [GetTokenInformation](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)
- [QueryServiceConfigW](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-queryserviceconfigw), [QueryServiceStatusEx](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-queryservicestatusex), [ChangeServiceConfigW](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-changeserviceconfigw)
- [System Restore WMI](https://learn.microsoft.com/en-us/windows/win32/sr/configuring-system-restore) e [SRSetRestorePoint](https://learn.microsoft.com/en-us/windows/win32/sr/calling-srsetrestorepoint)

A documentação pública de System Restore não oferece uma consulta conclusiva e não elevada do estado ativo de cada volume. Pontos antigos, VSS e flags de frequência não são prova. A UI mostra `unknown` quando não há prova, ou `disabled_by_policy` quando a política DisableSR está explicitamente ativa. A ferramenta nativa permite conferir; o app nunca ativa proteção automaticamente. A confirmação completa de estado por volume permanece pendente de uma API documentada/ensaio em VM.
