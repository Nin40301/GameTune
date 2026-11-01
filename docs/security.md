# Limites de confiança e revisão da Fase 1

Entradas externas aceitas: perfil JSON limitado a 64 KiB, versão 1, nome e IDs
conhecidos. O domínio rejeita campos desconhecidos, duplicatas e IDs inválidos;
o pacote de catálogo também oferece validação pelo schema fechado de perfis.
O perfil não carrega confirmações. O lote inteiro é autorizado antes de alterar
o primeiro item. IDs duplicados em uma solicitação são deduplicados.

Catálogos não são importáveis. A allowlist de alvos é independente dos IDs e
compara todos os campos: um ID aparentemente seguro com alvo Defender, Windows
Update, rede, áudio, energia essencial ou autenticação é rejeitado. As únicas
políticas Edge permitidas são StartupBoostEnabled e BackgroundModeEnabled.
DoSvc só aceita início manual, não Disabled. Os três conflitos de rede do
apêndice são bloqueados no motor e na seleção automática.

Não existe spawn de processos, PowerShell, acesso ao registro, WMI, rede,
telemetria, handles de jogos, hook ou FFI no código desta fase. Todo crate usa
`#![forbid(unsafe_code)]`. Executores Windows retornam erro tipado e nunca
substituem silenciosamente a operação por sucesso em mock.

Dependências diretas e finalidade:

| Dependência | Justificativa |
| --- | --- |
| serde / serde_json | Modelos e arquivos JSON com campos fechados |
| thiserror | Erros tipados sem unwrap/expect em produção |
| sha2 | Integridade dos documentos embutidos e encadeamento do WAL |
| time | Timestamp UTC RFC 3339 do journal |
| jsonschema, sem recursos padrão | Validação local Draft 7, sem resolução HTTP |
| tempfile, somente testes | Isolar journals e evitar alterações no host |

O lockfile fixa versões transitivas. A Fase 7 ainda precisa de análise de
dependências, inventário de licenças, assinatura e revisão de supply chain.
Checksums não substituem assinatura do binário. Actions estão fixadas por SHA
e o workflow tem somente `contents: read`, sem credenciais Git persistidas.

Antes de habilitar executores Windows na Fase 2, implementar e verificar:

- DACL restrita Administradores/SYSTEM em ProgramData, rejeição de reparse
  points/caminhos redirecionados e persistência segura antes da primeira ação
- Vinculação do journal a máquina e SID interativo; HKCU resolvido em HKEY_USERS
- Auxiliar elevado com protocolo fechado, autorização de solicitante e sem
  parâmetros de linha de comando derivados livremente da UI
- Scripts PowerShell fixos e embutidos com parâmetros tipados, sem interpolação
- Captura fiel, detecção de conflito externo, timeout e comparação pós-undo
- Ponto de restauração com tratamento do limite diário, sem mascarar falhas reais
- Startup: descoberta permitida, IDs opacos e bloqueio de SecurityHealth

Tauri, CSP, comandos IPC mínimos, i18n e acessibilidade serão implementados na
Fase 4. A UI deve operar sem elevação. O auxiliar revalida tudo independentemente
da UI. Sem rede por padrão; checagem manual de atualização só com opção explícita.
