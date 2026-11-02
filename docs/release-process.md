# Processo de release

0.6.5 Ã© uma **alfa para VM descartÃ¡vel**, sem assinatura ou instalador final. NÃ£o use no PC principal. A tag v0.6.0 permanece intacta como referÃªncia de cÃ³digo anterior; voltar a um executÃ¡vel antigo nÃ£o substitui Desfazer ou um snapshot de VM.

## Antes de criar a tag

1. Trabalhe em branch e abra PR para `main`. Exija revisÃ£o, CI obrigatÃ³rio e histÃ³rico linear. NÃ£o faÃ§a merge sÃ³ porque o build compilou
2. Confira a mesma versÃ£o em Cargo, lockfile, npm/lockfile, Tauri e recursos/manifestos Win32. Confira o identificador `io.github.nin40301.ametista` e o CHANGELOG
3. Rode os checks abaixo. NÃ£o rode testes com escrita no host; os ensaios administrativos continuam opt-in de VM com snapshot
4. Revise a linha de base de consumo sem substituÃ­-la automaticamente. Completo Ã© o padrÃ£o; Reduzido ainda excede o teto mediano de 200 MiB neste host. NÃ£o transforme esse resultado em aprovaÃ§Ã£o
5. Confira a documentaÃ§Ã£o de recuperaÃ§Ã£o, as limitaÃ§Ãµes do portÃ¡til e os itens pendentes do aceite em VM

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
pwsh -NoProfile -File tools/build-release.ps1
pwsh -NoProfile -File tools/verify-desktop.ps1
```

NÃ£o hÃ¡ autorizaÃ§Ã£o para merge ou tag automÃ¡tica nesta tarefa. Depois do PR aprovado, do merge pelo titular e dos checks verdes no commit escolhido:

```powershell
git switch main
git pull --ff-only
git status --porcelain
# A saÃ­da anterior precisa estar vazia; confira o commit que serÃ¡ marcado
git log -1 --oneline
git tag -a v0.6.5 -m "Ametista 0.6.5 alpha"
git push origin v0.6.5
```

Uma tag existente nunca deve ser movida para corrigir um artefato. FaÃ§a nova versÃ£o/tag. NÃ£o use `git push --force` ou recrie `v0.6.0`.

## Artefatos e hashes

O pipeline de release do Bloco D deverÃ¡ criar um **rascunho**, com ZIP portÃ¡til Windows x64, `SHA256SUMS.txt`, SBOM CycloneDX e atestado de procedÃªncia. AtÃ© esse workflow estar validado, nÃ£o apresente um ZIP local como release publicado. O executÃ¡vel da UI e o auxiliar precisam vir do mesmo build; o endurecimento do Bloco E recusarÃ¡ elevaÃ§Ã£o a partir de pasta gravÃ¡vel por usuÃ¡rio comum.

Para conferir um arquivo baixado:

```powershell
Get-FileHash -Algorithm SHA256 -LiteralPath .\ametista-0.6.5-windows-x64-portable.zip
# Compare todos os 64 caracteres com a entrada correspondente em SHA256SUMS.txt
gh attestation verify .\ametista-0.6.5-windows-x64-portable.zip --repo Nin40301/GameTune
```

O hash detecta alteraÃ§Ã£o em relaÃ§Ã£o ao arquivo listado; nÃ£o Ã© assinatura de cÃ³digo. O atestado deve identificar este repositÃ³rio e o workflow/commit esperado. Confira tambÃ©m o manifesto dentro do ZIP, versÃµes dos quatro executÃ¡veis e ausÃªncia de `.env`, certificados, tokens, dados de usuÃ¡rios, `node_modules` ou executÃ¡veis de fixture. Teste o pacote extraÃ­do somente na VM descartÃ¡vel.

Revisar/publicar o rascunho Ã© uma aÃ§Ã£o do titular. NÃ£o declarar instalador assinado, SmartScreen sem aviso, conformidade de acessibilidade ou aceite de jogos sem suas evidÃªncias. NÃ£o contorne avisos de seguranÃ§a do Windows para testar no PC principal.

## Assinatura futura e proteÃ§Ã£o de main

Preparar somente os nomes de segredos `WINDOWS_CERTIFICATE_BASE64` e `WINDOWS_CERTIFICATE_PASSWORD`; nÃ£o adicionar valores em arquivos/artefatos/logs e nÃ£o usar certificado nesta etapa. A Fase 7 definirÃ¡ importaÃ§Ã£o temporÃ¡ria, Authenticode, timestamp e verificaÃ§Ã£o sem exportar chaves privadas.

ConfiguraÃ§Ã£o recomendada para `main`: PR obrigatÃ³rio, pelo menos uma aprovaÃ§Ã£o, checks CI obrigatÃ³rios e atualizados, conversas resolvidas, histÃ³rico linear, force-push/deleÃ§Ã£o bloqueados e revisÃ£o de CODEOWNERS para seguranÃ§a/workflows. O titular aplica as regras no GitHub; elas nÃ£o foram alteradas por este documento.
