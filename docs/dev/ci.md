# CI e release

## Checks obrigatórios sugeridos

Em `main`, exigir `Frontend (Linux)`, `Rust (ubuntu-24.04)`, `Rust (windows-2025)`, `Dependency audits` e `PR dependency review`. PR obrigatório, uma aprovação, conversas resolvidas, branch atualizada, histórico linear, CODEOWNERS e bloqueio de force-push/deleção. O titular configura a proteção no GitHub; esta tarefa não a aplica.

CI separa frontend Linux (formato, ESLint, TS strict/build, componentes, Playwright/axe), Rust Linux/Windows (fmt, clippy com todos os harnesses, testes, release sem fixtures) e auditorias. Testes administrativos ignorados só rodam manualmente no runner de VM com opt-in e snapshot. O gate de consumo exige desktop dedicado interativo e é manual, separado de runners hospedados ocupados.

`cargo audit` falha em vulnerabilidades RustSec sem exceções locais; avisos informativos permanecem visíveis no JSON. `npm audit --omit=dev --audit-level=high` e revisão de dependências bloqueiam vulnerabilidades altas/críticas. A revisão não comenta em PRs e não precisa de permissão de escrita. Consultas de auditoria precisam de rede no CI, nunca no app. Ver [ferramentas](toolchain.md) para os avisos transitivos atuais.

Todos os workflows começam com `permissions: {}` e liberam permissões por job. Jobs comuns têm só `contents: read`; checkout não persiste credenciais. Cache Cargo é separado por SO, compilador e Cargo.lock; cache npm usa package-lock. Não há cache de credenciais, certificados ou dados de usuário. Concurrency cancela CI antigo da mesma referência; release de tag não é cancelado no meio da publicação de rascunho.

## Pins de Actions

Tags oficiais consultadas via API do GitHub em 02/11/2026; workflows usam o SHA completo, não a tag mutável:

| Action | Versão | SHA |
| --- | --- | --- |
| [checkout](https://github.com/actions/checkout/releases/tag/v7.0.1) | 7.0.1 | `3d3c42e5aac5ba805825da76410c181273ba90b1` |
| [setup-node](https://github.com/actions/setup-node/releases/tag/v7.1.0) | 7.1.0 | `949feb2413d6458794dcd2491c4babbbce0c15c1` |
| [cache](https://github.com/actions/cache/releases/tag/v6.1.0) | 6.1.0 | `55cc8345863c7cc4c66a329aec7e433d2d1c52a9` |
| [upload-artifact](https://github.com/actions/upload-artifact/releases/tag/v7.0.2) | 7.0.2 | `cf430e030ddbb5b0abf93d22962f4752f3646cd9` |
| [download-artifact](https://github.com/actions/download-artifact/releases/tag/v8.0.2) | 8.0.2 | `9000827ccba6bdab643e8b6fd33ac0654aef8333` |
| [dependency-review-action](https://github.com/actions/dependency-review-action/releases/tag/v5.0.0) | 5.0.0 | `a1d282b36b6f3519aa1f3fc636f609c47dddb294` |
| [attest-build-provenance](https://github.com/actions/attest-build-provenance/releases/tag/v4.2.2) | 4.2.2 | `4d101475d8b20a2381f78447822ac1eab6504dd8` |

O wrapper de atestado usa `actions/attest` também fixado por SHA. `create-storage-record: false` evita permissão de metadados organizacionais. O job de atestado/rascunho recebe `contents: write`, `id-token: write`, `attestations: write`, mas não faz checkout nem executa código/scripts do projeto. Recebe apenas artefatos do build desta execução. Assinatura Sigstore de procedência **não** é assinatura Authenticode.

## Reprodução

```powershell
cargo install cargo-audit --version 0.22.2 --locked
cargo audit
npm audit --prefix apps/desktop --omit=dev --audit-level=high
pwsh -NoProfile -File tools/build-release.ps1
pwsh -NoProfile -File tools/verify-desktop.ps1
pwsh -NoProfile -File tools/package-portable.ps1 -Destination outputs/portable-local-unique
```

O script compila o auxiliar primeiro e fornece seu SHA-256 ao build da interface, preparando a validação antes do UAC. O empacotamento usa allowlist de arquivos e recusa sobrescrever evidências. Não execute os auxiliares ou testes de escrita no host.

O workflow de release só roda em tag `v*`, repete CI, exige tag igual à versão, gera dois ZIPs, 20 SBOMs Rust, um SBOM frontend, hashes e atestados, e cria **rascunho pré-release**. Nenhuma tag ou publicação real é feita para testar este workflow. O ensaio completo de atestado/rascunho exige uma tag futura do titular; ver [processo de release](../release-process.md).

## Imagens fixadas — 02/11/2026

CI/release usam ubuntu-24.04 e windows-2025 para evitar mudanca automatica de familia do SO sob *-latest e facilitar comparacao de falhas. Nao sao imagens imutaveis: atualizacoes do runner hospedado continuam chegando. Ferramentas e Actions permanecem fixadas; atualizar a familia exige PR e verificacao. Matriz de VM Windows cliente permanece independente: runner Server nao valida Home/Pro.
