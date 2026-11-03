# Descrição do PR — Ametista 0.6.5 / fechamento 6.7

Ametista apresenta ajustes do Windows com simulação, revisão de riscos e reversão pelo journal, mantendo Appx como remoção manual com consentimento separado. Esta branch integra a identidade liquid glass e o fechamento 6.7 à main; não adiciona ajustes nem relaxa segurança/consentimentos.

## Entrega 1

- **Decisão do titular de 2 de novembro de 2026, política v2:** CPU mediana <1%, alerta p95 >2%; RAM mediana ≤200 MiB e regressão RAM ≤15%. Completo aprovado nas séries preservadas: 163,54 MiB/0,3932%; Reduzido informativo, 244,04 MiB/0,3886%, sem bloquear. Monitor estrito ≤15 MiB/<0,5% preservado e aprovado. V1, base e medições antigas intactas. [Política e avaliações](../performance.md)
- CI em ubuntu-24.04/windows-2025. Pacote interno com cinco executáveis x64/CRT estática, hashes e script Windows PowerShell 5.1: VM/admin/dois opt-ins/snapshot, cópia protegida, resultados e retomada após dois reinícios manuais. Sem Rust/Git/VS na VM. **Não é release**
- Doze defeitos corrigidos: banner/barra de ação, StatusStrip, contagem, voz pt-BR/en, ícones, Orb, sparklines, escala tipográfica e switches/contexto de Jogos. [24 capturas inspecionadas](../assets/screenshots/6.7/manifest.json) e [teste por defeito](../history/phase-6.7/ui-fixes.md)

Mudanças novas são os commits posteriores a b066483; a comparação contra main também contém a implementação 6.5 anterior. Nada foi aplicado no host. Não fazer merge/tag automaticamente. Entrega 2 só parte da main após este merge, em design/aaa; partes 3.2–3.8 não estão declaradas prontas.

## Validação

Local: 136 Rust passaram, cinco VM ignorados; 33 componentes, 74 casos e2e na suíte (reruns não somados); fmt/clippy -D warnings, ESLint, TS strict, i18n, axe, CSS compilado e verify-desktop. O CI executa a suíte completa nas imagens fixadas; conferir o commit dos Checks. [Relatório, pacote/hash e comandos](../history/phase-6.7/delivery-1.md).

## Checklist

- [x] Política v2 e aprovação do titular registradas com data; v1/base/séries preservadas
- [x] Completo aprovado nas séries finais; Reduzido informativo; monitor estrito inalterado
- [x] Investigação Reduzido encerrada em duas tentativas, hipótese documentada sem correção cega
- [x] Pacote VM compilado, hashes/PE/declarações conferidos; nenhum teste administrativo no host
- [x] Doze correções 3.1 com regressões, capturas e inspeção
- [ ] CI completo verde em ubuntu-24.04/windows-2025 no SHA final deste PR
- [ ] Execução real do pacote, ACL/tarefa/retomada e dois reboots na VM descartável
- [ ] Roundtrip completo/UAC/outra conta e Minecraft/Loja/Edge/Configurações/Segurança na matriz Home/Pro
- [ ] Proteção do Sistema confirmada por volume; estado desconhecido continua explicitamente desconhecido
- [ ] NVDA/Narrador, DPI/zoom/alto contraste físicos; preencher checklist manual
- [ ] Primeiro ensaio de release/assinatura/instalador na Fase 7, após os aceites necessários
- [ ] Merge pelo titular; só então criar design/aaa da main para a Entrega 2
