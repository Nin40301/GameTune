# Manutenção do catálogo

O catálogo contém 86 descritores embutidos, dos quais 68 têm identidade/mapeamento
documentado. A revisão de nível 1/2 cobre 58 dos 73 itens. Fontes por ID e limites
estão em [catalog-sources.md](catalog-sources.md), sem depender de rede em runtime.

`load_embedded` verifica SHA-256 de items/schema/profile.schema, valida Draft 7,
desserializa com campos fechados e aplica uma allowlist independente de alvos.
`verified` exige fonte; não implica seleção automática nem melhora de desempenho.

Metadados adicionais ao modelo inicial: `target.kind`, `riskNote` localizado,
`verified`, `verificationNote`, `sources`, `supportedEditions`, `maxBuild`,
`requiresSsd` e `automatic`. Este último permite confirmar um alvo documentalmente
sem recomendar sua aplicação automática. Ficam manuais SysMain, serviços que a
fonte desaconselha, Appx e preferências cujo documento só descreve leitura.

Para adicionar/revisar:

1. Confirme nome/caminho/valor e limites em documentação primária. No caso VDOT,
   mantenha a cadeia Microsoft Learn → arquivo referenciado e o commit fixado.
   Não use fóruns/Q&A como prova de API suportada. Na dúvida, verified false.
2. Documente nome, ação, risco e reversão em pt-BR/en. Não confunda remover um
   valor de política com recuperar dados que essa política apagou.
3. Revise `core/src/approved_targets.json` separadamente. O alvo inclui valor e
   modo desejados; mudar só o catálogo não concede outra capacidade.
4. Preserve alto risco de dependências Xbox e bloqueio global de apps. Appx
   continua revert manual e fora de todos os presets. O motor reversível não o
   executa; use a ação manual separada, com consentimento por app.
5. Preserve bloqueios absolutos de rede e Recall. DoSvc só aceita Manual, nunca
   Disabled. Verificação não elimina esses bloqueios.
6. Home só recebe política comprovadamente aplicável. As preferências HKCU de
   Modo de Jogo/recomendações, documentadas para leitura, permanecem manuais.
7. Atualize o schema de perfis e `docs/dev/catalog-sources.md`. Rode
   `python tools/update-catalog-hashes.py` e revise o diff antes de testar/commitar.

Os descritores startup.hkcu/hklm não autorizam gravação. O backend descobre apenas
nomes e executáveis permitidos e emite IDs `startup.hkcu.<hash>` ou
`startup.hklm.<hash>`, vinculados a máquina/SID, nome e bytes de Run. O schema
aceita esse formato; o domínio exige que o ID exista no catálogo descoberto.
Um arquivo importado não pode adicionar uma associação. Run não é apagado nem
executado; apenas o valor StartupApproved daquele nome é alterado. Entrada
essencial, variável de ambiente ambígua ou comando inesperado fica oculto.

O layout 02/03 de StartupApproved não é uma API pública documentada pela Microsoft:
continua experimental, não verificado e somente manual, com restauração dos bytes.

`test-targets` e `vm-tests` acrescentam somente namespaces AmetistaTest fixos para
testes. Nunca habilite essas features no binário distribuído. Não importe
catálogos, scripts ou targets de arquivos do usuário. A validação de schema usa
jsonschema sem recursos de resolução HTTP/arquivos externos.
