# Bloco D — ferramentas, CI, dependências e rascunho de release

## Resumo

Rust 1.98.1, Node 26.13.0 e npm 11.20.0 fixados, com checks de consistência. Frontend Linux independente de Rust Linux/Windows. Auditorias Cargo/npm e revisão de dependências em PR, Actions por SHA, permissões por job, concurrency e caches Cargo/npm. Workflows manuais de VM/consumo preservam seus opt-ins e limiares.

Release em tag futura `v*`: CI, build sem fixtures, dois ZIPs portáteis, SHA256SUMS, 20 SBOMs Rust + um frontend, procedência Sigstore e pré-release **em rascunho**. O job com escrita/OIDC não executa código do projeto. Nenhum certificado, tag ou release foi criado nesta entrega.

## Verificação local

```powershell
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
cargo test --workspace --all-features --locked --offline
cargo audit
npm audit --prefix apps/desktop --omit=dev --audit-level=high
npm run format:check --prefix apps/desktop
npm run lint --prefix apps/desktop
npm test --prefix apps/desktop
npm run build --prefix apps/desktop
pwsh -NoProfile -File tools/build-release.ps1 -SkipFrontend
pwsh -NoProfile -File tools/verify-desktop.ps1
pwsh -NoProfile -File tools/package-portable.ps1 -Destination outputs/block-d-portable
node tools/check-doc-links.mjs
```

128 testes Rust passaram, cinco de VM ignorados; 24 componentes passaram. Actionlint 1.7.12 (download oficial com checksum conferido) e parser YAML validaram os quatro workflows. SBOMs CycloneDX foram gerados localmente para os 20 crates e frontend; os ZIPs de ensaio foram criados por allowlist, sem publicar. Cargo/npm não encontraram vulnerabilidades classificadas; dois avisos informativos transitivos estão detalhados em [ferramentas](../dev/toolchain.md). A UI não mudou; último e2e/axe completo: 48 testes no Bloco B.

## Riscos e pendências

O CI remoto será conferido depois deste commit. O primeiro release de tag ainda precisa validar atestado/rascunho no GitHub, sem simular essa aprovação. Fixar ferramentas não garante builds byte a byte entre SDKs. Proteção de branch, descrição e tópicos são configuração do titular. Nenhum ajuste real foi aplicado ao host. Próximo: Bloco E, endurecimento do auxiliar e suporte/recuperação.
