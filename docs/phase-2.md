# Fase 2 — implementação e limites de validação

Data da entrega: 31/10/2026 (America/Sao_Paulo). Base aprovada: commit d733991.
Licença escolhida pelo titular e aplicada: **Apache-2.0**, com texto oficial em
LICENSE, metadados Cargo e NOTICE.

## Resumo

O backend Windows está implementado para registro, serviços, tarefas, energia,
StartupApproved, ponto de restauração, identidade interativa e armazenamento
protegido. A arquitetura aprovada foi mantida, com infraestrutura compartilhada
em windows-runtime e os seis adaptadores por tipo.

Appx deixou de ser somente um item bloqueado: há uma **ação manual separada**,
com consentimento por app, aviso de que undo não reinstala e confirmação adicional
para dependências de jogos. O motor reversível continua excluindo Appx; a remoção
usa remove_app, com inventário instalado/provisionado e removed-apps.json
sincronizado antes de remover. O histórico guarda nome, PackageFamilyName,
pacotes e resultado, inclusive falhas/tentativas interrompidas. Um HTML offline
tem aviso e botões reais para ms-windows-store://pdp/?PFN=..., sem abrir a Store
automaticamente. A integração na tela desktop pertence à Fase 4.

A revisão documental chegou a **68/86 itens**, incluindo **58/73 de nível 1/2**.
Fontes e limites estão em catalog-sources.md. Há separação entre identidade
documentada, recomendação automática, edição/build e efeito medido. Os exemplos
VDOT citados têm cadeia de referência Microsoft Learn e commit fixo; servem para
confirmar nomes, não como recomendação de otimização para jogos.

Decisões conservadoras:

- Home não recebe políticas restritas a Pro/Enterprise/Education
- SysMain e serviços desaconselhados pela fonte ficam só manuais
- Políticas de diagnóstico zero/experiências de consumidor respeitam Enterprise/Education
- Recall está bloqueado porque a política pode apagar dados que um DWORD não recupera
- Preferências documentadas apenas como leitura ficam manuais até validação funcional
- StartupApproved é experimental/manual; só nomes, executáveis e vínculos conhecidos
- Os três conflitos de serviços de rede do apêndice continuam bloqueados

## Implementação e recuperação

- Registro usa winreg raw; ausência é distinta de zero. Só remove chaves criadas
  se ainda estiverem vazias, preservando dados de terceiros
- Serviços preservam Start, DelayedAutoStart e execução, com timeout; nunca
  desativam dependentes nem iniciam dependências silenciosamente
- Tarefas usam COM e alteram só Enabled. XML é guardado e mudanças externas bloqueiam undo
- Energia guarda plano ativo/inventário e só remove o plano criado pela ação
- Startup conserva bytes originais e associa IDs a máquina/SID, nome e bytes de Run
- Scripts fixos recebem JSON por stdin; nenhum texto da UI vira código/comando
- Identidade é local e vinculada à sessão WTS, inclusive sob outra conta elevada;
  não há consulta de nomes a domínio nem fallback ao HKCU do administrador
- ProgramData usa DACL Administradores/SYSTEM, recusa links/reparse points e
  mantém lock global; outro SID não captura segundo original enquanto houver
  alterações ativas, mesmo se o arquivo de proprietário de escopo se perder
- Journal v2 é vinculado à máquina/SID e sincronizado antes da ação. O formato v1
  experimental não é migrado silenciosamente
- WindowsSession cria/verifica ponto de restauração antes de aplicar e trata
  limite diário com evidência do histórico; não mascara proteção indisponível

## Verificação local

Host: Windows 11 Pro x64 build 26200, Rust/Cargo 1.98.1, terminal sem elevação.

| Checagem | Situação |
| --- | --- |
| Testes normais | 51 testes passaram |
| Registro nativo dedicado | 2 testes incluídos nos 51, somente HKCU\Software\AmetistaTest |
| fmt e Clippy padrão | Passaram sem avisos |
| Clippy com feature vm-tests | Passou sem avisos |
| Build release, features padrão | Passou |
| Scripts PowerShell | Parser Windows PowerShell 5.1 e compilação em memória do helper, sem registrar serviço |
| Harness de VM | Compila; não executado neste host |
| Integração administrativa Home/Pro e UAC com outra conta | Pendente |
| Execução remota dos workflows GitHub | Não realizada |

Recontagem após a entrega: **51 = 26 + 6 + 7 + 2 + 4 + 6**, respectivamente
foundation, manual_apps, safety_boundaries, scopes_and_sources, core e
windows-runtime. Os dois testes nativos integram os seis do runtime; não são
testes adicionais. Esse total é do Windows, sem habilitar os testes ignorados
de VM. A referência anterior a “dentro dos 50” era um erro de edição.

Os testes novos cobrem consentimento por app, confirmação reforçada, gravação de
PFN antes da ação, falha parcial, crash de remoção, reconciliação, botão Store,
URI maliciosa, separação de undo, fonte/edição, journal por SID e exclusão entre
usuários. O teste nativo compara a árvore completa de valores/tipos/subchaves do
alvo dedicado, antes/após leitura, após alteração e após restauração, incluindo
tipos binário, string, expand string, multi-string, DWORD, QWORD e valor ausente.

## Comandos

```powershell
cargo fetch --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo test --workspace --locked --offline
cargo clippy -p ametista-windows-runtime --features vm-tests --all-targets --locked --offline -- -D warnings
cargo test -p ametista-windows-runtime --features vm-tests --locked --offline --no-run
cargo test -p ametista-windows-runtime embedded_powershell_sources_parse --locked --offline
cargo build --workspace --release --locked --offline
cargo run -p ametista-catalog --example simulate --locked --offline
cargo run -p ametista-catalog --example removed_apps --locked --offline -- outputs/appx-reinstall-demo.html
```

Para rodar a suíte administrativa, siga windows-vm.md: VM descartável, snapshot
do hipervisor, desktop interativo elevado e opt-in explícito. Ela usa somente
serviço/tarefa/helper próprios, Run dedicado, AmetistaTest e plano de teste.
Compara XML/DACL/configurações completas e preserva before/after/restored e WAL.
Não aplica alvos reais do catálogo. Appx real não é removido pela suíte.

## Riscos e aceite restante

**A implementação não equivale ao aceite final da Fase 2.** Falta executar a
suíte administrativa e a matriz em VM descartável, validar ponto de restauração
em proteção habilitada, UAC com outra conta e remoção Appx com pacote dedicado.
Não há evidência prática de Minecraft/Game Pass após aplicar perfis reais.

Outros limites: PFN ainda é suportado pela Store, mas está descontinuado; apps
retirados podem não ser reinstaláveis. PowerShell pode ser bloqueado por política
corporativa. Modern Standby pode impedir criação/ativação de plano clássico.
Não houve corte de energia físico. Estado criado por terceiros resulta em
conflito, não sobrescrita silenciosa. DACL/escopo dependem do Windows suportado e
o token precisa ter proprietário padrão seguro. Não distribuir uma build com
vm-tests ou test-targets.

UI, auxiliar de elevação/IPC, CLI, métricas, watcher, logs com rotação, instalador
e assinatura continuam nas próximas fases. O ZIP desta entrega contém fontes,
não é o ZIP portátil de um aplicativo final.

## Próximo passo

Concluir o aceite administrativo conforme windows-vm.md e registrar a matriz.
Depois, Fase 3: ametista-cli sobre WindowsSession, com --profile, --dry-run,
--undo, --list e JSON, incluindo comandos separados de remoção/reinstalação Appx.

## Arquivos criados e alterados

Criados:

```text
.github/workflows/windows-vm.yml
NOTICE
crates/acceptance/tests/manual_apps.rs
crates/acceptance/tests/scopes_and_sources.rs
crates/catalog/examples/removed_apps.rs
crates/core/src/removal.rs
crates/windows_runtime/Cargo.toml
crates/windows_runtime/scripts/appx.ps1
crates/windows_runtime/scripts/catch.ps1
crates/windows_runtime/scripts/fixture.ps1
crates/windows_runtime/scripts/identity.ps1
crates/windows_runtime/scripts/parse.ps1
crates/windows_runtime/scripts/power.ps1
crates/windows_runtime/scripts/prelude.ps1
crates/windows_runtime/scripts/restore_point.ps1
crates/windows_runtime/scripts/service.ps1
crates/windows_runtime/scripts/storage.ps1
crates/windows_runtime/scripts/store.ps1
crates/windows_runtime/scripts/task.ps1
crates/windows_runtime/src/fixture.rs
crates/windows_runtime/src/lib.rs
crates/windows_runtime/src/native_tests.rs
crates/windows_runtime/src/platform.rs
crates/windows_runtime/src/registry.rs
crates/windows_runtime/src/startup.rs
crates/windows_runtime/src/storage.rs
crates/windows_runtime/src/transport.rs
crates/windows_runtime/tests/windows_vm.rs
docs/catalog-sources.md
docs/phase-2-plan.md
docs/phase-2.md
docs/windows-vm.md
locales/en.json
locales/pt-BR.json
```

Alterados:

```text
.github/workflows/ci.yml
.gitignore
CHANGELOG.md
Cargo.lock
Cargo.toml
LICENSE
README.md
catalog/items.json
catalog/profile.schema.json
catalog/schema.json
catalog/sha256.json
crates/acceptance/Cargo.toml
crates/acceptance/tests/foundation.rs
crates/acceptance/tests/safety_boundaries.rs
crates/catalog/Cargo.toml
crates/catalog/examples/simulate.rs
crates/core/Cargo.toml
crates/core/src/engine.rs
crates/core/src/executor.rs
crates/core/src/journal.rs
crates/core/src/lib.rs
crates/core/src/model.rs
crates/core/src/profile.rs
crates/core/src/protection.rs
crates/exec_appx/Cargo.toml
crates/exec_appx/src/lib.rs
crates/exec_mock/Cargo.toml
crates/exec_mock/src/lib.rs
crates/exec_power/Cargo.toml
crates/exec_power/src/lib.rs
crates/exec_registry/Cargo.toml
crates/exec_registry/src/lib.rs
crates/exec_scheduled_task/Cargo.toml
crates/exec_scheduled_task/src/lib.rs
crates/exec_service/Cargo.toml
crates/exec_service/src/lib.rs
crates/exec_startup/Cargo.toml
crates/exec_startup/src/lib.rs
docs/catalog.md
docs/journal.md
docs/security.md
```
