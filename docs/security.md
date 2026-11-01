# Segurança — implementação da Fase 2

Entradas externas: IDs conhecidos e consentimentos, nunca código ou alvos livres.
Perfis têm limite de 64 KiB e campos fechados; não importam confirmações. O lote
inteiro é autorizado antes da primeira alteração. Alvos são comparados ao
catálogo embutido também no adaptador Windows; fixtures só com feature explícita.

`WindowsSession` é a API de produção para CLI/UI: armazenamento protegido,
escopo, preflight e checkpoint. Os executores de baixo nível são componentes
internos para composição/testes; nenhum comando Tauri deverá expô-los diretamente.
Ainda não há IPC, auxiliar UAC, assinatura, frontend ou instalador.

Todos os crates proíbem unsafe em Rust. winreg encapsula a FFI de registro. Há
uma FFI C# fixa e comentada para WTS, sem manipulação de jogos. Os demais scripts
usam cmdlets/.NET/COM oficiais. O transporte aceita enum interno, gera
EncodedCommand só de código embutido e envia dados JSON por stdin. Não há
Invoke-Expression, parâmetros convertidos em código ou scripts do usuário.

O PowerShell roda oculto, NoProfile/NonInteractive, com caminho obtido do HKLM,
PATH e PSModulePath restritos ao Windows. Há timeout e leitura concorrente de
saída limitada. Erros não imprimem stdout bruto nem dados pessoais na UI.
Um timeout não prova ausência de alteração: o original permanece recuperável.

A identidade usa a sessão WTS do próprio processo. O SID é obtido localmente do
shell canônico explorer.exe da mesma sessão por GetOwnerSid; só esse executável
do Windows é elegível, sem consultar memória ou handles de jogos. Um fallback
compara perfis carregados locais e recusa ambiguidades. Não há resolução de nomes
em domínio nem fallback para o hive da conta administrativa. HKCU é HKEY_USERS/SID.
Sessão 0, SID ausente e hive inacessível falham de forma fechada.

ProgramData é resolvido pela API de pasta conhecida, não por uma pasta livre da
UI. A criação usa DACL protegida Administradores/SYSTEM e proprietário do mesmo
grupo. Diretório existente inseguro, reparse point ou arquivo com vínculo/ACL
inesperados é rejeitado. O token elevado precisa ter proprietário padrão seguro,
para que WAL, locks e temporários de substituição também sejam protegidos.

Há lock exclusivo do WAL, lock global e proprietário de escopo persistente.
Enquanto um SID tiver alterações reversíveis ativas, outro não pode capturar um
segundo original para os mesmos ajustes globais. Outra máquina/SID não abre o
WAL como se fosse seu. Administradores podem alterar quaisquer arquivos; hashes
não são autenticação contra um administrador malicioso.

Appx tem confirmação por app e confirmação adicional para dependências de jogos.
O histórico write-ahead é separado, com nome, PFNs, inventário e resultado. Undo
de configurações não tenta reinstalar. Um relatório HTML offline contém links
Store validados/escapados e CSP sem scripts; somente clique explícito abre Store.
PFN é o formato pedido, ainda suportado mas descontinuado na documentação atual.

Dependências diretas:

| Dependência | Finalidade |
| --- | --- |
| serde / serde_json | Modelos e JSON fechado |
| thiserror | Erros tipados |
| sha2 | Integridade e IDs vinculados |
| time | Instantes RFC 3339 |
| jsonschema sem recursos padrão | Schema local sem resolução externa |
| tempfile | Substituição atômica no mesmo volume e testes isolados |
| winreg, somente Windows | Captura/escrita de tipo e bytes do registro |
| base64 | Codificar scripts constantes UTF-16 para PowerShell |

CI: Actions por SHA, contents read, sem credenciais Git persistidas. Build release
usa features padrão; a suíte administrativa exige workflow manual e runner de VM
dedicado, não PR de terceiros em máquina pessoal.

Pendências: validação administrativa Home/Pro, teste de elevação com outra conta,
ensaios de energia/crash, assinatura, análise de dependências e threat model do
IPC na UI. Um checkpoint é adicional ao journal; não garante recuperar dados de
apps removidos. Não habilitar proteção de sistema ou compatibilidade de energia
silenciosamente para contornar falhas.
