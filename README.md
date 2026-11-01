# GameTune

Fundação de um aplicativo offline para ajustes transparentes e reversíveis do
Windows voltados a jogos. Nome provisório. **Fase 1 implementada**: biblioteca
Rust, catálogo, simulação com mocks, journal e testes. Ainda não há aplicativo
desktop, CLI de produção ou instalador. Os executores Windows reais retornam
`Unavailable`, inclusive no Windows.

## Executar

Requisitos: Rust estável (testado com 1.98.1), Git e, no Windows, Visual Studio
Build Tools com as ferramentas C++ e Windows SDK. Não use terminal elevado.
Dependências de compilação são baixadas pelo Cargo; a biblioteca não usa rede.

```powershell
cargo fetch --locked
cargo test --workspace --locked --offline
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo run -p gametune-catalog --example simulate --locked --offline
```

O exemplo usa **um Windows 10 Pro fictício**, com valores de registro ausentes.
Imprime JSON de simulação do perfil Seguro sem consultar o PC, criar journal,
abrir processos ou alterar configurações. Não é o `gametune-cli` da Fase 3.

## Estrutura criada

```text
GameTune/
├── .github/workflows/ci.yml
├── Cargo.toml, Cargo.lock, rust-toolchain.toml
├── catalog/
│   ├── items.json                 # 86 descritores do Apêndice A
│   ├── schema.json
│   ├── profile.schema.json
│   └── sha256.json
├── crates/
│   ├── core/src/                  # domínio, motor, journal, perfis, proteção
│   ├── catalog/src/               # carregador de dados embutidos
│   ├── catalog/examples/          # demonstração sem alteração real
│   ├── exec_mock/src/             # estado fictício e injeção de falhas
│   ├── exec_registry/src/
│   ├── exec_service/src/
│   ├── exec_scheduled_task/src/
│   ├── exec_appx/src/
│   ├── exec_startup/src/
│   ├── exec_power/src/
│   └── acceptance/tests/          # testes portáveis entre crates
├── docs/
│   ├── architecture.md
│   ├── catalog.md
│   ├── journal.md
│   ├── security.md
│   └── phase-1.md
├── tools/update-catalog-hashes.py
├── CHANGELOG.md
└── LICENSE                        # escolha pendente
```

Pastas futuras: `apps/desktop` (Tauri 2, React, TypeScript strict, Tailwind e
i18n), `crates/cli`, `crates/metrics`, `crates/watcher` e um auxiliar elevado.
Elas serão criadas nas fases correspondentes, com implementações verificáveis.

## Uso e reversão da biblioteca

1. Carregue o catálogo por `gametune_catalog::load_embedded()`.
2. Selecione IDs com `Preset::select` ou importe um `Profile` validado.
3. Use `Engine::run` para simular. Esse é o caminho padrão e não exige journal.
4. Apenas após revisão, use `Engine::apply` com um `Journal` exclusivo.
5. Use `Engine::undo(&[], ...)` para desfazer tudo, ou passe IDs para desfazer
   itens específicos. Operações incompletas exigem recuperação antes de reaplicar.

Nesta fase, só o estado dos mocks pode mudar. Os exemplos de testes mostram o
fluxo completo. O histórico auditável permanece após desfazer: reversão restaura
configurações, não apaga registros de auditoria.

## Decisões de segurança

- Simulação padrão; catálogos externos e comandos livres não são aceitos.
- Só dois mapeamentos estão documentalmente verificados. Os demais têm
  `verified: false`; não entram automaticamente em nenhum perfil.
- Os perfis Home ficam vazios até haver verificação de políticas aplicáveis.
- Itens de alto risco exigem confirmação por ID a cada solicitação manual.
- Remover Appx não oferece reversão exata: dados e versão original podem se
  perder. Esses itens são manuais, fora dos perfis automáticos e bloqueados no
  motor desta fase. Reinstalação pela Store será uma função distinta na Fase 2.
- `icssvc`, `WMPNetworkSvc` e `XboxNetApiSvc` aparecem no apêndice mas são serviços
  de rede. Permanecem visíveis no catálogo, com execução bloqueada pela regra
  absoluta de proteção, inclusive se houver confirmação manual.
- Game DVR por política tem limite de build para Windows 10, conforme a
  [documentação da Microsoft](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-applicationmanagement#allowgamedvr).

## CI e entrega futura

GitHub Actions executa formato, Clippy, testes offline, build release e simulação
em Linux e Windows. Não foi publicado um repositório remoto nesta fase.
Os artefatos atuais são o catálogo validado e o lockfile; instalador NSIS/MSI e
ZIP portátil executável pertencem à Fase 7.

Na Fase 7, configurar assinatura Tauri via segredos `WINDOWS_CERTIFICATE_BASE64`
e `WINDOWS_CERTIFICATE_PASSWORD`, importação em armazenamento temporário do
runner e seleção pelo thumbprint. **Nenhum certificado é lido nem uma assinatura
é executada no pipeline atual**. Certificados e segredos ficam fora do Git.

## Limites e próximos passos

Ainda faltam executores Windows, DACL em ProgramData, identificação do usuário
interativo, ponto de restauração, UI, métricas, watcher, empacotamento e testes em
VM. A Fase 1 não comprova ganhos de FPS, metas de consumo ou compatibilidade
prática com Minecraft/anti-cheat. A matriz Home/Pro de Windows 10 22H2 e Windows
11 23H2/24H2+ continua pendente. Consulte `docs/phase-1.md`.

Não há limpeza de registro, esvaziamento de working sets, acesso à memória de
jogos, hooks, DLLs ou drivers. Futuras alterações devem ser testadas primeiro em
VM. Um ponto de restauração será uma proteção adicional, não substitui o journal.

A licença está pendente de escolha do titular; veja `LICENSE`.
