# Descrição proposta do PR

Título: **Prepare Ametista 0.6.5 alfa com UI acessível, auxiliar protegido e entrega auditável**

Base: `main` · Origem: `design/liquid-glass`. [Abra a comparação](https://github.com/Nin40301/GameTune/compare/main...design/liquid-glass). Não fazer merge/tag antes da revisão e dos aceites pendentes.

## Problema e resultado

A página pública ainda apresenta o checkpoint anterior, enquanto a interface Ametista já possui identidade e fluxos novos. Esta mudança alinha produto/identifier/versão, organiza a UI e a documentação e prepara builds auditáveis. Simulação continua padrão; aplicação e Appx continuam exigindo revisão, consentimento e auxiliar isolado. Nenhum novo ajuste entrou no catálogo.

O auxiliar só eleva de instalação local protegida, com SHA-256 embutido, recusa de diretórios graváveis por usuário comum e validação Authenticode offline quando há assinatura. Identidade e serviços usam WTS/SCM; logs tipados, diagnóstico com prévia e orientação de recuperação não abrem permissões de dados protegidos.

UI preserva lilás, gema, Plus Jakarta Sans, vidro, Orb e botões. Agora tem páginas/hooks/estado separados, onboarding, resultado, filtros e foco/anúncios acessíveis. CSS em layers substitui arquivos de polimento e Tailwind não utilizado. [48 capturas](../assets/screenshots/0.6.5/manifest.json) mostram todas as telas em claro/escuro e 1440/390, com dados de testes.

CI separa frontend Linux e Rust Linux/Windows, fixa ferramentas e Actions, audita dependências e prepara release de tag em rascunho com dois ZIPs, hashes, CycloneDX e procedência. Assinatura/instalador final ficam para a Fase 7.

## Validação e limitações

Rust: 134 passaram, cinco administrativos ignorados. Componentes: 30/30. Suíte Playwright: 66 casos, incluindo axe nos dois idiomas/temas e reflow geométrico. Fmt, clippy -D warnings, ESLint, TS strict, versão/CSP/manifestos, i18n e documentação gerada verificados. O CI final e seu commit são registrados no [relatório](../history/release-0.6.5-report.md).

**O gate de consumo da interface ainda reprova.** Cinco execuções por modo: Completo 163,54 MiB/0,3932% CPU mediana; Reduzido 244,04 MiB/0,3886%. CPU de referência zero continua bloqueando qualquer mediana positiva; Reduzido excede o teto de RAM em 44,04 MiB. Monitor passa com picos 2,75 MiB/0,1305%. [Evidências e comparação](../performance.md). Não se alterou baseline, limiares ou harness para aprovar. CI funcional verde não substitui esse gate.

## Checklist de revisão

- [x] Versão 0.6.5 e identifier `io.github.nin40301.ametista`; tag v0.6.0 intacta
- [x] Catálogo e regras de proteção preservados; diff de backend limitado às autorizações de consumo/segurança/suporte
- [x] Auxiliar protegido; nenhum bypass de UAC/ACL/CLM; comandos/IDs fechados
- [x] README, recuperação, guias de ajustes, comunidade, pipeline de release e backlog
- [x] Automação local funcional, screenshots e auditorias; inventário de 92 linhas unsafe/extern com justificativa
- [ ] CI obrigatório verde no commit final do PR (conferir link no relatório)
- [ ] Gate da interface aprovado contra a referência ou decisão explícita do titular; não presumir waiver
- [ ] Aceite em VM descartável: UAC/outra conta/ACL/assinatura, apply/reboot/undo/snapshot e apps protegidos/jogos
- [ ] Proteção do Sistema por volume confirmada; estado desconhecido não pode ser apresentado como ativo
- [ ] NVDA/Narrador, zoom real, DPI físico 125/150/200%, alto contraste e bandeja: preencher roteiro manual
- [ ] Primeiro ensaio de tag/SBOM/procedência e revisão do rascunho pelo titular
- [ ] Titular aplica descrição/tópicos e proteção de main; nenhuma alteração automática dessas configurações

Rascunho recomendado enquanto os bloqueios acima estiverem abertos. [Backlog](../backlog.md) separa pendências P1 das propostas P2 sem implementação. Este texto é uma proposta para o titular; nenhuma mensagem/PR público foi enviado automaticamente.
