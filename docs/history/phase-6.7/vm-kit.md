# Entrega 1 / bloco CI e pacote VM

CI/release fixados em ubuntu-24.04/windows-2025 para evitar troca automática de família do SO. Ferramentas, SHAs, permissões e opt-ins preservados. A imagem ainda recebe patches; runner Server não prova suporte cliente Home/Pro.

Pacote local compilado com cinco executáveis x64, CRT estática, nove hashes, README pt-BR e runner PowerShell 5.1. Build isolado de features; CLI padrão separada. O runner exige VM/admin/opt-ins/snapshot declarado; cópia protegida, tarefa temporária de logon, estado antes da ação, lock e validação de conta/UUID/boot/hash/ACL para retomar. Pede dois reinícios manuais, salva resultados e não reaplica após crash. Recovery é explícito, nunca aprovação.

Comandos: `tools/build-vm-acceptance.ps1`, `node tools/verify-vm-package.mjs <pasta>`, actionlint, parser PowerShell, cargo fmt/clippy/test --workspace --all-features. Rust: 136 passaram, cinco VM ignorados. Verificação de pacote confirmou 2+2+1 declarações de teste, hashes e ausência de runtime redistribuível MSVC; nenhum teste administrativo foi executado. Guarda de VM recusou o host em CheckOnly antes de escrever. Windows PowerShell local também recusou a execução pela política de scripts: foi respeitada, sem bypass. A retomada/tarefa/ACL/reboots só podem ser aceitos na VM.

O primeiro pacote gerado foi inspecionado; o pacote final é regenerado após o bloco visual para amarrar commit/hash. Não é release. Roteiro: [kit](../../dev/vm-acceptance-kit.md). Próximo: doze correções 3.1 com testes/capturas, sem iniciar a Entrega 2 antes do merge.
