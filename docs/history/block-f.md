# Bloco F — interface, fluxos e evidências

## Resumo

App.tsx passou a compor provider/modelo e shell; páginas, hooks e estado ficam separados, usando os mesmos comandos tipados do backend. CSS consolidado em quatro layers; Tailwind sem utilitários usados foi removido. Preservados gema, paleta, Plus Jakarta Sans local, Orb e botões. Novos fluxos: onboarding em três passos com idioma inicial do navegador, resultado após operação, filtro de histórico, suporte/diagnóstico e orientação para dados antigos. Estado, erros e próxima ação usam InlineStatus; foco no h1, skip link e progresso limitado a um anúncio por 3 s.

Arquivos principais criados: `apps/desktop/src/{pages,hooks,state}`, componentes compartilhados, `i18n-format.ts`, testes onboarding/results/accessibility, `tools/check-i18n.mjs`, `tools/generate-icons.mjs`, SVGs de marca e ICOs multiquadro, docs de frontend/ativos/checklist e galeria. `polish.css` e CSS redundante foram incorporados. A licença OFL da fonte acompanha NOTICE e os pacotes portáteis; o titular Apache-2.0 permanece.

O monitor usa somente sua janela Win32 para escolher/cachear/liberar o ícone no DPI corrente. A coleta nativa detectou uma referência de recurso incorreta: dangling<u16> representa endereço 2, não MAKEINTRESOURCE(1). Corrigida para `without_provenance(1)`; nova coleta completou. Nenhuma regra de detecção/sessão foi alterada. Inventário auditado: **92 linhas de fronteira unsafe/extern**, com função, motivo e hash em [unsafe-inventory](../security/unsafe-inventory.md); unwrap/expect seguem proibidos.

## Verificação

```powershell
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
cargo test --workspace --all-features --locked --offline
npm run format:check --prefix apps/desktop
npm run lint --prefix apps/desktop
npm test --prefix apps/desktop
npm run build --prefix apps/desktop
npm run test:e2e --prefix apps/desktop
node tools/check-i18n.mjs
node tools/audit-unsafe.mjs
pwsh -NoProfile -File tools/verify-desktop.ps1
```

Rust: **134 passaram, zero falharam, cinco administrativos ignorados**. Componentes: **30/30 em sete arquivos**. Suíte e2e tem **66 casos únicos**. As seleções afetadas foram reexecutadas após correções: dez fluxos, treze visuais/performance e dezesseis onboarding/resultados; não somar essas execuções sobrepostas como testes distintos. O CI final executará a suíte completa. A primeira rodada teve sete falhas de seletor ambíguo entre live region e texto visível; seletores exatos corrigidos, sem remover anúncios. Strict TS, ESLint, format, clippy, catálogo, versão/manifestos/CSP e dez permissões passaram localmente. As 390 chaves pt-BR/en coincidem; 30 arquivos JSX sem prosa fixa.

Axe AA automatizado cobre oito telas × dois idiomas × dois temas × quatro larguras; onboarding/resultados nos dois idiomas/temas em 1440/390. Também há geometria de reflow em 640/320, foco, toque, teclado, movimento/transparência reduzidos e forced-colors. Isso não certifica conformidade manual.

## Imagens e consumo

[48 capturas e hashes](../assets/screenshots/0.6.5/manifest.json): todas as oito telas, claro/escuro, 1440/390, página inteira; três passos iniciais e resultado. Cada imagem foi inspecionada nos painéis locais, incluindo faixas da lista longa. A galeria usa dados explícitos do backend de testes. Rótulos longos e ações permanecem visíveis; teste separado confere a última linha sob a barra fixa.

Gate real inalterado, cinco execuções por modo, aquecimento 30 s + 60 amostras de 1 s; mesmo Início 1100×740, luz, idioma e DPI; sem jogos, builds ou DevTools durante a coleta. [Resultados e séries](perf-final-0.6.5/manifest.json), [comparação detalhada](../performance.md).

| Modo | RAM mediana antes → depois (MiB) | CPU mediana antes → depois (%) | Resultado |
| --- | ---: | ---: | --- |
| Completo | 165,21 → 163,54 | 0 → 0,3932 | Reprova regressão de CPU |
| Reduzido | 205,50 → 244,04 | 0 → 0,3886 | Reprova RAM/CPU; 44,04 MiB acima do teto |
| Monitor, picos | 2,47 → 2,75 | 0,1299 → 0,1305 | Passa ≤15 MiB e <0,5% |

Não foi regravada a linha de base nem criado piso para CPU zero. p95 da interface gera alerta, nunca veto por pico. Completo continua padrão novo por consumir 80,49 MiB menos que Reduzido nesta coorte. A escolha não aprova a regressão de CPU. Investigação anterior de GPU/WebView encerrada, identidade preservada.

## Riscos e próximo bloco

UI funcional verde não equivale a gate de consumo aprovado. VM administrativa, NVDA/Narrador, DPI físico, zoom real 200/400%, troca de monitor e alto contraste físico continuam pendentes no [roteiro manual](../dev/accessibility-checklist.md). Nenhum ajuste real foi aplicado no host. Próximo: Bloco G, transparência do catálogo, backlog, descrição/checklist de PR e relatório consolidado; sem merge/tag automáticos.
