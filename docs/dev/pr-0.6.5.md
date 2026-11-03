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

**Decisão do titular em 2 de novembro de 2026 — política v2.** CPU mediana <1%, p95 >2% como alerta, RAM ≤200 MiB e regressão RAM ≤15%. Completo **aprovado** com 163,54 MiB/0,3932%; Reduzido informativo, 244,04 MiB/0,3886%, sem bloquear o PR. Monitor mantém picos estritos ≤15 MiB/<0,5% e passa com 2,75 MiB/0,1305%. Política v1, baseline e séries preservadas; [decisão e avaliações](../performance.md). Não são novas coletas ou aceite de VM.

## Checklist de revisão

- [x] Versão 0.6.5 e identifier `io.github.nin40301.ametista`; tag v0.6.0 intacta
- [x] Catálogo e regras de proteção preservados; diff de backend limitado às autorizações de consumo/segurança/suporte
- [x] Auxiliar protegido; nenhum bypass de UAC/ACL/CLM; comandos/IDs fechados
- [x] README, recuperação, guias de ajustes, comunidade, pipeline de release e backlog
- [x] Automação local funcional, screenshots e auditorias; inventário de 92 linhas unsafe/extern com justificativa
- [ ] CI obrigatório verde no commit final do PR (conferir link no relatório)
- [x] Gate v2 Completo aprovado pelo titular em 02/11/2026; Reduzido informativo; monitor estrito preservado
- [ ] Aceite em VM descartável: UAC/outra conta/ACL/assinatura, apply/reboot/undo/snapshot e apps protegidos/jogos
- [ ] Proteção do Sistema por volume confirmada; estado desconhecido não pode ser apresentado como ativo
- [ ] NVDA/Narrador, zoom real, DPI físico 125/150/200%, alto contraste e bandeja: preencher roteiro manual
- [ ] Primeiro ensaio de tag/SBOM/procedência e revisão do rascunho pelo titular
- [ ] Titular aplica descrição/tópicos e proteção de main; nenhuma alteração automática dessas configurações

Rascunho recomendado enquanto os aceites de VM/AX estiverem abertos. [Backlog](../backlog.md) separa pendências P1 das propostas P2 sem implementação. Este texto é uma proposta para o titular; nenhuma mensagem/PR público foi enviado automaticamente.
