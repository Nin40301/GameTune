# Bloco B — versão e identidade estável

## Resumo

Ametista 0.6.5 alfa em Cargo/lockfile, npm/lockfile, Tauri e recursos/manifestos Win32. Identifier `io.github.nin40301.ametista`; UI, monitor e harness usam o mesmo caminho local. Produto, janela, ícone e NOTICE usam Ametista, preservando o titular histórico e Apache-2.0. A tag v0.6.0 permanece em de75a7531faa45600d78818a3391451811f723de.

Sem migração ou remoção de dados. O journal em ProgramData permanece no mesmo diretório; preferências/WebView/jogos/medições ficam no diretório novo do identifier. A documentação orienta encerrar o monitor anterior e concluir recuperação antes de trocar de versão. Não houve alteração de catálogo, IPC, consentimentos ou níveis de elevação.

## Arquivos e comandos

- `dev/application-identity.md`: motivo e efeitos em diretórios, AppUserModelID e futuro instalador
- `../release-process.md`: revisão, tag pelo titular, artefatos, hashes, atestado e assinatura futura
- `tools/verify-version.mjs` e `tools/verify-desktop.ps1`: consistência entre stacks, recursos nativos e hashes públicos da linha de base
- Cargo/npm/Tauri, recursos Win32 e referências de paths atualizados; CHANGELOG 0.6.5

```powershell
node tools/verify-version.mjs
npm run format:check --prefix apps/desktop
npm run lint --prefix apps/desktop
npm run test --prefix apps/desktop
npm run build --prefix apps/desktop
npm run test:e2e --prefix apps/desktop
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
cargo test --workspace --all-features --locked --offline
cargo build --workspace --release --locked --offline
pwsh -NoProfile -File tools/verify-desktop.ps1
```

128 testes Rust aprovados, cinco exclusivos de VM ignorados; 24 componentes e 48 Playwright aprovados. Fmt, clippy, lint, TypeScript/build e verificações de produto/versão/elevação/CSP passaram. A validação encontrou recursos sem a string FileVersion no auxiliar/monitor; foram corrigidos e verificados nos binários finais, sem relaxar o check.

## Riscos e próxima entrega

Não foi instalado ou executado um ajuste no host. Ainda faltam qualificação de atualizações/desinstalação/atalhos e UAC em VM, assinatura e instalador. A referência de consumo continua sendo o executável original identificado no baseline, sem fingir nova medição do binário 0.6.5. Não houve tag, merge ou publicação de release. Próximo bloco: documentação pública, galeria, recuperação e comunidade.
