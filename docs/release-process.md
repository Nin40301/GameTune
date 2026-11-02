# Processo de release

0.6.5 é uma **alfa para VM descartável**, sem assinatura ou instalador final. Não use no PC principal. A tag v0.6.0 permanece intacta como referência de código anterior; voltar a um executável antigo não substitui Desfazer ou um snapshot de VM.

## Antes de criar a tag

1. Trabalhe em branch e abra PR para `main`. Exija revisão, CI obrigatório e histórico linear. Não faça merge só porque o build compilou
2. Confira a mesma versão em Cargo, lockfile, npm/lockfile, Tauri e recursos/manifestos Win32. Confira o identificador `io.github.nin40301.ametista` e o CHANGELOG
3. Rode os checks abaixo. Não rode testes com escrita no host; os ensaios administrativos continuam opt-in de VM com snapshot
4. Revise a linha de base de consumo sem substituí-la automaticamente. Completo é o padrão; Reduzido ainda excede o teto mediano de 200 MiB neste host. Não transforme esse resultado em aprovação
5. Confira a documentação de recuperação, as limitações do portátil e os itens pendentes do aceite em VM

```powershell
npm ci --prefix apps/desktop
npm run format:check --prefix apps/desktop
npm run lint --prefix apps/desktop
npm run test --prefix apps/desktop
npm run build --prefix apps/desktop
npm run test:e2e --prefix apps/desktop
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
cargo build --workspace --release --locked
pwsh -NoProfile -File tools/verify-desktop.ps1
```

Não há autorização para merge ou tag automática nesta tarefa. Depois do PR aprovado, do merge pelo titular e dos checks verdes no commit escolhido:

```powershell
git switch main
git pull --ff-only
git status --porcelain
# A saída anterior precisa estar vazia; confira o commit que será marcado
git log -1 --oneline
git tag -a v0.6.5 -m "Ametista 0.6.5 alpha"
git push origin v0.6.5
```

Uma tag existente nunca deve ser movida para corrigir um artefato. Faça nova versão/tag. Não use `git push --force` ou recrie `v0.6.0`.

## Artefatos e hashes

O pipeline de release do Bloco D deverá criar um **rascunho**, com ZIP portátil Windows x64, `SHA256SUMS.txt`, SBOM CycloneDX e atestado de procedência. Até esse workflow estar validado, não apresente um ZIP local como release publicado. O executável da UI e o auxiliar precisam vir do mesmo build; o endurecimento do Bloco E recusará elevação a partir de pasta gravável por usuário comum.

Para conferir um arquivo baixado:

```powershell
Get-FileHash -Algorithm SHA256 -LiteralPath .\ametista-0.6.5-windows-x64-portable.zip
# Compare todos os 64 caracteres com a entrada correspondente em SHA256SUMS.txt
gh attestation verify .\ametista-0.6.5-windows-x64-portable.zip --repo Nin40301/GameTune
```

O hash detecta alteração em relação ao arquivo listado; não é assinatura de código. O atestado deve identificar este repositório e o workflow/commit esperado. Confira também o manifesto dentro do ZIP, versões dos quatro executáveis e ausência de `.env`, certificados, tokens, dados de usuários, `node_modules` ou executáveis de fixture. Teste o pacote extraído somente na VM descartável.

Revisar/publicar o rascunho é uma ação do titular. Não declarar instalador assinado, SmartScreen sem aviso, conformidade de acessibilidade ou aceite de jogos sem suas evidências. Não contorne avisos de segurança do Windows para testar no PC principal.

## Assinatura futura e proteção de main

Preparar somente os nomes de segredos `WINDOWS_CERTIFICATE_BASE64` e `WINDOWS_CERTIFICATE_PASSWORD`; não adicionar valores em arquivos/artefatos/logs e não usar certificado nesta etapa. A Fase 7 definirá importação temporária, Authenticode, timestamp e verificação sem exportar chaves privadas.

Configuração recomendada para `main`: PR obrigatório, pelo menos uma aprovação, checks CI obrigatórios e atualizados, conversas resolvidas, histórico linear, force-push/deleção bloqueados e revisão de CODEOWNERS para segurança/workflows. O titular aplica as regras no GitHub; elas não foram alteradas por este documento.
