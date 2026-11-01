# Manutenção do catálogo

`catalog/items.json` contém os 86 descritores do Apêndice A; não é carregado do
disco em runtime. `include_str!` inclui dados, schema e hashes no binário.
`load_embedded` verifica SHA-256 dos três documentos, valida JSON Schema Draft 7,
desserializa com campos fechados e aplica a proteção semântica de `core`.

Extensões do modelo inicial: `target.kind` discrimina o alvo; `riskNote` é
localizado em pt-BR/en; `verified`, `verificationNote`, `sources`,
`supportedEditions`, `maxBuild` e `requiresSsd` registram as restrições.
`maxBuild: null` significa sem limite superior conhecido, não garante que uma
política seja eficaz em versões futuras.

Dois mapeamentos têm confirmação documental:

| ID | Fonte e limite |
| --- | --- |
| `reg.cortana` | [Microsoft Experience/AllowCortana](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-experience#allowcortana), Pro/Enterprise/Education |
| `reg.game_dvr_policy` | [Microsoft ApplicationManagement/AllowGameDVR](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-applicationmanagement#allowgamedvr), Pro/Enterprise/Education, apenas Windows 10 |

`verified` confirma o mapeamento documental, não ganho de desempenho nem teste
real em todos os builds. Os outros 84 itens permanecem não verificados e fora
dos perfis automáticos. A documentação pode mudar; a Fase 2 deve testar presença,
aplicabilidade e efeito em cada VM suportada.

Para adicionar ou revisar um item:

1. Confirme nome, alvo, valor, edição e builds em fonte primária. Na dúvida,
   mantenha `verified: false`; não invente serviço, pacote ou chave.
2. Escreva nomes, descrição, efeitos adversos e instrução de reversão curtos,
   em pt-BR/en. Remoção Appx tem `revert: manual` e não é reversão exata.
3. Revise a allowlist independente `crates/core/src/approved_targets.json`.
   Alterar apenas o catálogo não concede um alvo novo. Não gere a allowlist
   automaticamente a partir de dados importados. A comparação inclui o valor
   desejado; não aceita caminhos irmãos, prefixos genéricos ou wildcards.
4. Preserve alto risco em GamingServices, XboxIdentityProvider, XblAuthManager,
   XblGameSave, XboxNetApiSvc, XboxGipSvc e bloqueio global de apps em segundo
   plano. `SysMain` exige SSD para pré-seleção; HAGS exige aviso de reinício.
5. A proteção de rede bloqueia `icssvc`, `WMPNetworkSvc` e `XboxNetApiSvc` mesmo
   constando no apêndice e mesmo com confirmação. Inclusão não é autorização.
6. Atualize o enum de IDs em `profile.schema.json` se os IDs mudarem. Perfis
   só carregam versão, nome e IDs. Não podem autorizar alto risco nem código.
7. Rode `python tools/update-catalog-hashes.py`, revise o diff e execute
   `cargo test --workspace --locked --offline` e Clippy antes do commit.

Os dois itens `startup.*` são descritores de descoberta, não autorização para
escrever qualquer entrada de inicialização. A Fase 2 deve emitir identificadores
opacos vinculados a entradas descobertas permitidas, ocultar SecurityHealth e
revalidar a associação no auxiliar elevado. Nenhum executor real existe agora.

`jsonschema` usa `default-features = false`: resolução HTTP e de arquivos
externos não é habilitada. Os documentos contêm apenas referências locais ou
valores inline. Veja a [documentação de resolução](https://docs.rs/jsonschema/0.46.10/jsonschema/).
