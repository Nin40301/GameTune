# Bloco G — transparência e fechamento documental

Gerador `tools/generate-catalog-docs.mjs` e páginas `docs/user/adjustments.md`/`adjustments.en.md` cobrem os 86 descritores, com nome, efeito, risco/consequência, reversão, nível, builds/edições e fontes. O gerador não calcula nem altera seleção automática; o backend continua sendo a autoridade. CI recusa páginas desatualizadas.

Criados `docs/dev/adding-a-tweak.md`, `docs/user/diagnostics.md`, `docs/dev/pr-0.6.5.md`, `docs/backlog.md` e o [relatório consolidado](release-0.6.5-report.md). Índices, guia de uso, processo de release e changelog alinhados. P2 pesquisado apenas em fontes oficiais: cada proposta permanece verified:false, sem novo alvo/execução. P1 continua explícito, inclusive o gate de CPU/RAM da interface e os aceites de VM/AX.

```powershell
node tools/generate-catalog-docs.mjs --check
node tools/check-doc-links.mjs
node tools/verify-version.mjs
node tools/audit-unsafe.mjs
node tools/check-i18n.mjs
cargo audit
npm audit --prefix apps/desktop --omit=dev --audit-level=high
```

Gerador/links/versão/inventário/i18n passaram. Cargo e npm saíram com código zero; zero vulnerabilidades classificadas, dois avisos transitivos informativos do grafo GTK preservados. Workflows verificados por actionlint; nenhuma execução de release/tag foi disparada. Testes funcionais Rust/componentes/TS/lint/axe do Bloco F continuam a referência local; o push final repete o CI completo Linux/Windows.

Risco: documentação gerada descreve alvos, não garante presença, suporte ou benefício em todo Windows. Portátil não eleva de pasta gravável. Proteção por volume, matriz de VM, leitores de tela/DPI/zoom reais, primeira tag e consumo da interface continuam pendentes; não transformar documentação em aceite. Próximo: revisão do titular, pendências P1 e depois Fase 7. Nenhum merge/tag automático.
