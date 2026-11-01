# Fase 3 — entrega e validação

## Resultado

CLI de produção `gametune-cli` sobre WindowsSession com análise sem elevação,
simulação padrão, seleção de perfil, aplicação confirmada, listagem, desfazer,
ações Appx separadas e saída JSON/texto em pt-BR/en. Importação aceita somente IDs.
Não adiciona dependências externas; parser fechado testado para os poucos comandos.

Fase 2 recontada antes das mudanças: **51 testes = 26 + 6 + 7 + 2 + 4 + 6**.
Os **2 testes nativos de HKCU estão dentro dos 6 de windows_runtime e dentro dos
51**, sem somar duas vezes. Correção isolada em commit e JSON de contagem histórico.

Fase 3: **65 testes passaram no Windows local**, sem elevação. Distribuição:

| Suíte | Passaram |
| --- | ---: |
| acceptance/foundation | 26 |
| acceptance/manual_apps | 6 |
| acceptance/safety_boundaries | 7 |
| acceptance/scopes_and_sources | 2 |
| cli/tests/cli | 9 |
| core/unitários | 5 |
| core/read_only | 4 |
| windows_runtime/unitários, incluindo os 2 nativos HKCU | 6 |
| **Total** | **65** |

Testes administrativos não integram esses 65: dois de fixtures e um de aceite
real estão ignorados por padrão; foram compilados, não executados. O parser
PowerShell 5.1 verificou todos os scripts e compilou os três helpers C# fixos em
memória, sem registrar serviço nem abrir apps. Linux está configurado no CI;
não foi executado localmente nesta máquina Windows.

Análise real local, sem admin, no Windows 11 Pro build 26200: **88 linhas** (86
descritores + 2 entradas descobertas), 7 selecionadas no Seguro, nenhuma falha de
leitura, saída 0; ProgramData/GameTune não existia e continuou ausente. Tempo
medido de uma execução: aproximadamente 33 segundos. Isso valida leitura local,
não ganho de desempenho nem aceite da matriz Windows.

## Arquivos criados e alterados

- `crates/cli/{Cargo.toml,src/args.rs,src/lib.rs,src/main.rs,src/output.rs,tests/cli.rs}`
- `crates/core/src/inspection.rs`, `crates/core/tests/read_only.rs` e leitura
  sem escrita em journal/removal
- `crates/windows_runtime/scripts/{desktop,real_profiles}.ps1`, módulo
  `src/real_profiles.rs`, exemplo `examples/real_profile_e2e.rs`, teste
  `tests/real_profiles_vm.rs`; inspeção/identidade/transporte/Store/task atualizados
- `locales/cli.pt-BR.json`, `locales/cli.en.json`
- `docs/cli.md`, `docs/windows-real-profiles.md`, `docs/phase-3-plan.md`, este
  relatório e `docs/test-count-phase-3.json`
- README, CHANGELOG, segurança, VM e CI atualizados; workspace/lockfile em 0.3.0

## Comandos de verificação

```powershell
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
cargo test --workspace --locked --offline
cargo test -p gametune-windows-runtime --features vm-tests,real-profile-tests --locked --offline --no-run
cargo build -p gametune-cli --release --locked --offline
target/release/gametune-cli.exe --list --json
target/release/gametune-cli.exe --profile safe --dry-run --json
```

Rodar os harnesses administrativos exige seguir `windows-vm.md` ou
`windows-real-profiles.md` e restaurar snapshot do hipervisor ao terminar.

## Riscos e pendências

- Matriz Home/Pro, elevação por outra conta, Appx real e teste com dois reinícios
  ainda não executados. A entrega não declara o aceite administrativo concluído.
- Smoke test comprova ativação local/janela responsiva e estado básico do
  Defender; requer revisão visual. Não valida transações online nem todos os jogos.
- Sem admin, inventário Appx é só do usuário do processo. Leitura do histórico
  protegido pode exigir admin; analisar o catálogo não exige.
- Undo não restaura Appx, dados de apps, auditoria nem estado transitório do SO.
  Reinstalar abre a Loja explicitamente e pode usar rede pelo aplicativo da Loja.
- Consultas sequenciais podem levar dezenas de segundos. CLI ainda sem auxiliar
  UAC/IPC, UI, assinatura, instalador, métricas ou monitor de jogos.

Próxima fase: **Fase 4**, UI Tauri/React — início, itens, revisão e histórico,
consumindo o mesmo backend, i18n/acessibilidade e elevação somente na ação.
