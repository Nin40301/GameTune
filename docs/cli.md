# Ametista CLI — Fase 3

Execute a partir de um terminal comum. A CLI usa WindowsSession e não realiza
elevação automática. Sem flags, seleciona Seguro e consulta o Windows somente
para leitura. `--dry-run` explicita esse padrão. Para alterar configurações,
abra um terminal elevado na mesma sessão e use **ambos** `--apply --confirm`.
`--apply` e `--dry-run` juntos são erro, sem executar ações.

```powershell
cargo build -p ametista-cli --locked --offline
target/debug/ametista-cli.exe --profile safe
target/debug/ametista-cli.exe --profile balanced --dry-run --json > analise.json
target/debug/ametista-cli.exe --list --json
target/debug/ametista-cli.exe --help --lang en
```

`--profile` aceita `safe`/`seguro`, `balanced`/`equilibrado`,
`aggressive`/`agressivo` ou caminho para um JSON com schema fechado:

```json
{"version":1,"name":"Meu perfil","itemIds":["reg.edge_preload"]}
```

O arquivo tem limite de 64 KiB. Só IDs já presentes no catálogo da sessão são
aceitos. Perfil não concede consentimentos nem transporta scripts, comandos ou
alvos. Os presets excluem Appx, risco alto, alvos protegidos, não verificados e
políticas incompatíveis com a edição/build. Itens manuais reversíveis podem
exigir `--ack-risk ID` e `--ack-unverified ID`, além de compatibilidade real.

Após revisar a simulação, **em VM descartável para esta versão em validação**:

```powershell
target/debug/ametista-cli.exe --profile safe --apply --confirm --json
target/debug/ametista-cli.exe --undo
target/debug/ametista-cli.exe --undo all --apply --confirm --json
target/debug/ametista-cli.exe --undo reg.edge_preload --apply --confirm --json
```

Undo sem `--apply` apenas lê o histórico. Arquivos em ProgramData continuam
restritos a Administradores/SYSTEM: ler histórico protegido pode retornar código
5 em terminal comum; a análise de alvos não depende desse histórico e funciona
sem admin. Ler WAL não cria locks auxiliares, não trunca cauda incompleta e não
repara arquivos. Uma cauda incompleta é indicada em `tornTail`/`needsRecovery`.

## Appx: ações separadas, um app por comando

```powershell
target/debug/ametista-cli.exe appx remove appx.microsoft.copilot --json
target/debug/ametista-cli.exe appx remove appx.microsoft.copilot --apply --confirm --ack-no-undo --json
target/debug/ametista-cli.exe appx reinstall appx.microsoft.copilot --json
target/debug/ametista-cli.exe appx reinstall appx.microsoft.copilot --apply --confirm --json
```

Remover exige elevação e `--ack-no-undo`. Registra nome, PackageFamilyName e
inventário **antes** de remover para todos os usuários e novos perfis.
**Desfazer NÃO reinstala apps nem recupera seus dados**. `--undo ID` rejeita Appx.
GamingServices/XboxIdentityProvider também exigem `--ack-gaming-risk`; itens
não verificados exigem `--ack-unverified ID`. Leia o risco específico na análise.

Reinstalar sem `--apply` somente mostra links. Com `--apply --confirm`, abre a
página validada `ms-windows-store://pdp/?PFN=...` pelo shell do usuário interativo,
inclusive quando a CLI estiver elevada. Não instala automaticamente. Se o app
já foi removido, o PFN vem do histórico protegido; execute elevado para lê-lo.
Se ainda instalado, pode vir do inventário somente leitura. Não aceita URI livre.
Abrir a Loja exige ação explícita e a Loja pode usar rede. Disponibilidade,
versão e dados originais não são garantidos. PFN é um formato legado da Store.

`appx reconcile ID` simula o inventário de uma tentativa interrompida;
`appx reconcile ID --apply --confirm` atualiza o histórico a partir do inventário
atual, sem repetir remoção ou reinstalar.

## Relatórios e limites de leitura

`--text` é o padrão e `--json` produz exatamente um documento JSON em stdout,
inclusive em erros. Texto de erro vai para stderr no modo legível. `--lang pt-BR`
é o padrão; `--lang en` traduz mensagens, nomes e riscos. Códigos/nomes de campos
são estáveis; detalhes técnicos devolvidos pelo Windows preservam o idioma do SO.

A análise consulta **todos** os descritores e entradas de inicialização
permitidas descobertas. `selected` indica o perfil; ler um alvo protegido não
concede permissão para alterá-lo. O relatório contém compatibilidade, bloqueio,
risco, reinício e observação antes de qualquer aplicação. Em modo aplicar,
`items[].observation` continua sendo o estado **anterior**; `result.report`
contém o resultado das alterações. Itens já ajustados não são recapturados como
originais artificiais nem ganham entradas no journal.

| Campo/estado | Significado |
| --- | --- |
| `exists` / `missing` | Alvo existe / não existe no escopo consultado |
| `already_applied` | O estado atual coincide com o ajuste proposto |
| `unknown` | Acesso negado, timeout ou outra falha; `exists` é null |
| `discovery` | Descritor de Run; entradas elegíveis aparecem como itens separados |
| `current` / `currentDword` | Snapshot tipado e valor DWORD decodificado |
| `keyExists` | Existência da chave de registro, distinta da existência do valor |
| `scope` | Máquina, usuário interativo ou cobertura da consulta Appx |

Appx sem admin consulta somente os pacotes do usuário **do processo**. Ausência
nesse escopo não afirma ausência para outros usuários ou no provisionamento;
`current_user_only` comunica essa limitação. Elevado consulta AllUsers/provisionado.
Serviços/tarefas inexistentes são ausência normal. Consultas negadas ficam
desconhecidas, sem abortar os demais itens. A descoberta de inicialização é
deliberadamente fechada, oculta entradas essenciais e não aceita executáveis livres.

Não cria ProgramData, WAL, checkpoint ou logs de aplicação durante simulação;
não inicia a Loja, não troca energia e não altera registro, tarefas ou serviços.
Processos PowerShell de leitura podem aparecer nos logs normais do Windows. Não
há promessa de apagar auditoria do SO. Redirecionar stdout grava somente o arquivo
explicitamente escolhido pelo usuário; relatórios podem conter nomes/caminhos
locais e devem ser compartilhados apenas após revisão.

`--list` lista metadados embutidos, sem consultar o Windows, e funciona no Linux.
Análise real exige Windows com desktop interativo; Linux recebe código 7.
Não há opção de executar mocks no binário de produção. Não há chamadas de rede,
telemetria, atualização, processos de jogos ou métricas nesta fase. Uma análise
completa pode levar dezenas de segundos pelas consultas PowerShell sequenciais.

## Códigos de saída

| Código | Resultado |
| --- | --- |
| 0 | Consulta/operação concluída; leia resultados por item |
| 2 | Argumentos conflitantes, perfil/schema/ID inválido |
| 3 | Relatório parcial: falha de leitura ou item aplicado com falha/ignorado |
| 4 | Falta confirmação/consentimento, alvo protegido/manual ou incompatível |
| 5 | Privilégio ou acesso insuficiente, inclusive ao histórico protegido |
| 6 | Journal bloqueado/corrompido, recuperação ou conflito explícito |
| 7 | Plataforma/sessão indisponível, arquivo ou preflight/operação falhou |

Código 3 não equivale a rollback global. Consulte `result.report.items` e
desfaça se necessário. Erros individuais de aplicação ficam no relatório com 3;
erros que impedem iniciar a operação usam 4–7. `--apply` explícito pode alterar
parcialmente o sistema; o WAL mantém os originais. O código 10 pertence somente
ao harness de VM, para solicitar reinício manual, e não à CLI de produção.
