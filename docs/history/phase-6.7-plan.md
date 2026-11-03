# Fase 6.7 — planos de execução

Decisão e escopo do titular: 2 de novembro de 2026. Nenhum ajuste real no host. Um commit por bloco, testes pertinentes verdes. Sem merge automático; preservar referência, política antiga e medições.

## Entrega 1 — design/liquid-glass

1. Consumo: política v2 com CPU mediana <1%, p95 >2% como alerta, RAM ≤200 MiB e regressão ≤15%. Preservar v1/base/séries, registrar aprovação do Completo pelo titular e Reduzido informativo. Monitor estrito inalterado. Investigar Reduzido em no máximo duas tentativas, sem otimização às cegas.
2. CI/VM: imagens ubuntu-24.04/windows-2025. Pacote interno autossuficiente com CLI de produção, testes compilados, hashes, README pt-BR e runner PowerShell; exigir VM, admin, opt-ins e snapshot declarado; gravar resultados e retomar após reboot sem exigir toolchain na VM. Nunca tratar geração do pacote como aceite dos testes administrativos.
3. UI: corrigir os doze defeitos de 3.1 preservando identidade/backend, com teste para cada correção. Botões pt-BR em infinitivo, opções em substantivo, faixa de status útil, banner/espaçamento, barra sem vazamento, métricas/Orb/sparkline, switches e estado do monitor. Inspecionar capturas claras/escuras em desktop/celular.
4. Fechamento: relatório, comandos, capturas e limitações; CI nas imagens fixadas, descrição/checklist do PR atualizado. Sem marcar VM/AX manual como aprovado.

Riscos: segurança de retomada elevada, duas reinicializações e snapshot real dependem da VM; variabilidade WebView; mudanças extensas de texto podem afetar seletores. Não regravar baseline nem relaxar monitor. Portátil gravável continua sem elevação para uso normal; harness de VM é separado e nunca release.

## Entrega 2 — design/aaa, somente após merge da Entrega 1

1. Criar branch a partir da main integrada pelo titular. Formalizar tokens/escalas/voz e rota /design-system de desenvolvimento, excluída e verificada no build de produção.
2. Refinar shell e telas por fluxo (Início/Itens/Revisão/resultado, Apps/Histórico, Medições/Jogos/Configurações), com uma ação primária, estados completos e consentimentos preservados. Depois integrar movimento coerente e preferências.
3. Adicionar bases visuais geradas no CI Linux, pseudo-localização +40%, teclado ponta a ponta, axe, reflow e inspeção de todas as telas/estados em dois temas e 1440/390, até duas rodadas de ajustes.
4. Repetir consumo pela política v2 e relatar comparação. Atualizar checklist manual de leitores de tela, DPI/zoom/alto contraste, sem declarar conformidade sem evidência.

Adiado: experimentos 3.8 desligados e fora do escopo inicial; só entram com medição e aprovação. NVDA/Narrador, DPI físico, VM e primeira release continuam dependentes de aceite manual. O merge é dependência explícita: não criar design/aaa antecipadamente ou contornar com branch da branch.
