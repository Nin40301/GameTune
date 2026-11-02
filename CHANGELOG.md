# 0.6.5 — Ametista — 2026-11-02

- Identidade Ametista, interface lilás com vidro, pt-BR/en e temas claro/escuro
- Versões Cargo/npm/Tauri/recursos Win32 alinhadas em 0.6.5
- Identificador estável `io.github.nin40301.ametista`; dados anteriores não são apagados nem migrados automaticamente
- Linha de base de cinco execuções por modo; regressão de medianas RAM/CPU >15% e teto mediano de RAM de 200 MiB; p95 de CPU como alerta
- Completo passa e é o padrão para novas preferências; Reduzido permanece opção, mas excede o teto neste host em 5,50 MiB
- Monitor mantém os limites estritos de 15 MiB e CPU <0,5% e passou
- Alfa para testes em VM; sem assinatura ou instalador final nesta etapa
- Tag v0.6.0 preservada como referência; não houve merge automático

# Investigação de consumo da interface — 2026-11-01

- Bisect com duas coletas nativas por versão, hashes e traces de 10 s da WebView
- Fundo limitado à viewport, sombras menores e composição sem animação retida, preservando o redesign
- Orb com limiar acumulado de 1%, tabela fechada montada sob demanda e memoização de sparklines
- Consultas/animações visuais suspensas fora de foco; coletor e benchmark Rust preservados
- Corrigido blur que continuava ativo no Reduzido minificado; regressão testada com CSS de produção, incluindo forced-colors
- Alternativas que aumentaram RAM descartadas e documentadas; gates continuam reprovados
- Veja docs/history/performance-investigation.md para os resultados brutos, limitações e próximos passos

# Polimento da Fase 6.5 — 2026-11-01

- Tracking dos títulos suavizado, hierarquia simplificada e status de análise contextual
- Revisão compacta agrupada por risco, ícones por tipo e Checkbox acessível compartilhado
- Busca/categoria refinadas, espaço reservado para ações e navegação móvel com quatro itens e Mais
- Novas capturas e testes; backend e segurança preservados
- Gates nativos executados sem jogos: Completo 238,29 MiB/3,1209%; Reduzido 240,62 MiB/1,5748%; ambos reprovados, limites inalterados
- Veja docs/history/phase-6.5-polish.md para evidências e pendências

# Fase 6.5 — Renomeação

GameTune passou a se chamar Ametista. O titular histórico permanece GameTune contributors (NOTICE). Não há migração de dados: a pasta antiga em ProgramData bloqueia escritas, sem apagar ou mover conteúdo. A tag v0.6.0 preserva o checkpoint anterior.

# Changelog

## 0.6.0 — Fase 6 — 2026-11-01

- Removidas flags experimentais, inclusive defaults do wry substituídos por vazio
- Orçamentos separados: UI <=150 MiB; monitor <=15 MiB e CPU <0,5%
- Monitor Rust/Win32 asInvoker sem WebView, sob demanda e com ícone na bandeja
- Eventos WMI locais com caminho/PID/CreationDate, sem handles de jogos ou hooks
- Descoberta de configurações Steam/Epic/Battle.net/Riot/Minecraft e `.exe` manual
- Catálogo temporário com hash/schema: Modo de Jogo, energia e três serviços pausáveis
- Serviços pausados sem mudar StartMode; sessões sobrepostas compartilham ações
- WAL separado protegido, lock global, heartbeat/EOF com rollback e recuperação
- Tela Jogos, consentimentos, Configurações, i18n, teclado e testes axe
- Testes de PID reusado, múltiplas instâncias, payloads abusivos e crash real de
  executor simulado; ensaio WMI real somente com nosso processo sintético
- Aceite UAC/escritas/queda/reboot e custo do auxiliar elevado pendentes da VM

## 0.5.0 — Fase 5 — 2026-11-01

- RAM/CPU/processos reais por snapshot global, top 12 agrupado, sem handles de jogos
- Coleta compartilhada em background, 2 s no painel e 1 s durante benchmark
- Cinco rodadas de 60 s por condição, primeira descartada; persistência atômica,
  crash/retomada, lacunas/suspensão e validação de boot/plano de energia
- Medianas por rodada, p5/p95/MAD, pisos conservadores e resultado inconclusivo
- PresentMon por importação CSV v1, sem download, execução ou sessão ETW pelo app
- JSON/HTML offline com CSP, escaping e anonimização por projeção
- Gates reais incluindo WebView2, sem elevar limites ou esvaziar working sets
- Composição da própria janela por software para reduzir custo de GPU/memória
- Testes sintéticos, persistência/reboot, bridge, componentes e Playwright/axe
- Aceite de aplicar/reiniciar/desfazer continua exclusivo da VM

## 0.4.0 — Fase 4 — 2026-11-01

- UI Tauri 2/React/TypeScript strict/Tailwind em pt-BR/en, dark/light e seis telas
- Revisão com resumo por tipo, consentimentos individuais Appx e risco reforçado
- UI asInvoker e auxiliar requireAdministrator de uso único com protocolo tipado
- Named pipe com DACL por SID, PID/SID reais, nonce, prazo, limites e anti-replay
- Análise em worker, progresso/cancelamento, quatro leitores e um inventário Appx
- Cancelamento encerra somente os próprios filhos PowerShell de leitura
- Histórico protegido e reinstalação Store pelo backend existente; sem rede no WebView
- Componentes, Playwright/mock, axe em dois idiomas/temas e testes reais de pipe
- Binários com assets locais, CSP/allowlist mínimas e verificação dos manifestos
- Dois testes legados HKCU agora ignorados por padrão e exclusivos do opt-in de VM
- Reaplicação detecta estado externo divergente sem sobrescrever nem perder o original
- UAC/escritas/Store, outra conta e matriz Windows permanecem sem aceite em VM

## 0.3.0 — Fase 3 — 2026-10-31

- Contagem histórica corrigida: 51 testes na Fase 2, incluindo os 2 nativos
- CLI sobre WindowsSession com --profile, --dry-run padrão, --list e --undo
- Escrita só com --apply --confirm; consentimentos Appx individuais reforçados
- Inspeção real sem admin, existência/valor atual/já aplicado e erros por alvo
- Saída JSON versionada ou legível, mensagens pt-BR/en e códigos documentados
- Leitura de WAL/histórico sem criar arquivos ou reparar dados em simulação
- Reinstalação separada via link PFN e shell do usuário interativo validado
- Correção do caminho com barra final no COM do Agendador de Tarefas
- Transporte de scripts constantes sem exceder limite da linha de comando Windows
- Ensaio adicional opt-in com Seguro/Equilibrado reais, dois reinícios, smoke
  tests dos quatro apps Windows e comparação completa após desfazer
- CI compila/linta os harnesses sem executá-los e publica binário CLI padrão

## 0.2.0 — Fase 2 — 2026-10-31

- Remoção Appx manual separada, consentimento por app e confirmação reforçada
- Histórico prévio nome/PFN e HTML offline com botões de reinstalação Store
- 68/86 itens documentados, incluindo 58/73 de nível 1/2; fontes por ID
- Restrições de Home/Pro, elegibilidade automática separada da verificação
- Bloqueio Recall por exclusão de snapshots; SysMain e itens desaconselhados manuais
- Adaptadores Windows: registro raw, serviços, tarefas COM, energia e StartupApproved
- Identidade interativa local, escopo máquina/SID, DACL e exclusão entre usuários
- Ponto de restauração com tratamento do limite diário na API de produção
- Harness de VM com alvos próprios e snapshots completos; aceite administrativo pendente
- Licença Apache-2.0, NOTICE e CI portátil/manual de VM

## 0.1.0 — Fase 1 — 2026-10-31

- Workspace Rust com núcleo independente e seis limites de executores por tipo
- Catálogo embutido de 86 itens, schemas fechados e SHA-256 em runtime
- Proteção por alvo exato e bloqueio dos conflitos de rede do apêndice
- Perfis por nível, risco, verificação, edição, build e tipo de disco
- Journal write-ahead com lock, sync, checksum, estados e recuperação por undo
- Mocks compartilhados com falhas antes/depois da alteração e na reversão
- Exemplo de simulação, testes portáveis e CI Linux/Windows
- Documentação de arquitetura, manutenção, segurança e limitações por fase

Na versão 0.1.0, executores reais, desktop, CLI, métricas, watcher e instaladores
ainda estavam pendentes.

## Fase 6.5 — Camada visual

- Interface clara branco/lilás e escura violeta-tinta, fonte variável local e ícones SVG
- StatusOrb com RAM medida, componentes acessíveis, navegação inferior e efeitos reduzidos
- Preservados os contratos, consentimentos, catálogo e segurança
- Gates de consumo pós-redesign pendentes a pedido do usuário; sem afirmação de ganho de FPS ou redução de RAM
