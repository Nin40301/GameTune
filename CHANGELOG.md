# Changelog

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
