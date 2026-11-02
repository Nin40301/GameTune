# Ferramentas fixadas

Rust **1.98.1** (`rust-toolchain.toml`) e Node **26.13.0**, npm **11.20.0** (`.nvmrc`, `apps/desktop/.nvmrc`, `engines` e `.npmrc`) reproduzem as ferramentas usadas nesta alfa. `stable` e versões maiores soltas mudariam compilador, lint e resolução sem revisão. Rust é instalado explicitamente com a mesma versão em todos os jobs. `rust-version = 1.98` continua sendo o mínimo declarado, não a seleção do compilador.

Use `rustup show`, `rustc -Vv`, `node --version` e `npm --version` antes de reproduzir. NVM para Windows aceita `nvm install 26.13.0` e `nvm use 26.13.0`; não altere preferências do sistema por script. CI usa setup-node e `npm ci`, sem atualizar o lockfile. Ferramentas fixadas não garantem binários byte a byte idênticos entre Windows SDKs; hashes identificam cada artefato real.

Ferramentas de desenvolvimento, sem dependência em runtime:

| Ferramenta | Versão | Motivo |
| --- | --- | --- |
| [cargo-audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit) | 0.22.2 | Consultar RustSec contra Cargo.lock; vulnerabilidades bloqueiam, avisos informativos ficam no relatório |
| [cargo-cyclonedx](https://github.com/CycloneDX/cyclonedx-rust-cargo) | 0.5.9 | Gerar CycloneDX 1.5 por crate, incluindo dependências transitivas do alvo Windows |
| npm audit / npm sbom | 11.20.0 | Auditar produção e gerar inventário do frontend a partir do lockfile |

Atualize versões em PR dedicado, junto com lockfiles, CI, documentação e checks. Dependabot propõe atualizações semanais; não há auto-merge. Auditorias consultam serviços externos **no desenvolvimento/CI**; o app distribuído continua offline.

Na auditoria de 02/11/2026: zero vulnerabilidades classificadas pelo cargo-audit e zero no npm de produção. Há avisos RustSec RUSTSEC-2024-0370 (`proc-macro-error`, sem manutenção) e RUSTSEC-2024-0429 (`glib`, unsoundness), transitivos de GTK/Tauri no lockfile. Não são compilados no alvo Windows desta aplicação; não foram suprimidos. Acompanhar correção upstream e repetir auditoria antes de release. Um resultado datado não garante ausência de vulnerabilidades futuras.
