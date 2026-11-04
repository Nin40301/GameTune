# Regressão visual e inspeção

Referência canônica: Chromium da versão fixada em package-lock, `ubuntu-24.04`, fonte local Plus Jakarta Sans, pt-BR, claro/escuro, 1440/390 px. `aaa-visual.spec.ts` cobre as oito telas e /design-system com página inteira; o horário da última amostra é mascarado apenas na comparação determinística. Os dados são do mock identificado na própria tela, nunca medições reais.

A referência é gerada pelo CI Linux, não por uma captura Windows: executar o workflow **Ametista CI** na branch candidata por workflow_dispatch com `update-visual-baselines=true`. O passo gera PNGs em `apps/desktop/e2e/__screenshots__/aaa-visual.spec.ts/`, roda a suíte e publica as bases junto das evidências. Baixar, inspecionar e commitar essas bases; PR/push normais apenas comparam (`toHaveScreenshot`, limiar por pixel 0,12 e no máximo 0,2% de pixels diferentes). Não atualizar automaticamente para fazer passar. Mudança esperada de fonte/Chromium exige revisão explícita das diferenças.

No Windows, os mesmos ensaios geram imagens e verificam layout/semântica, mas não regravam nem comparam a referência Linux. `aaa-states.spec.ts` captura estados vazios de Início/Apps/Histórico/Jogos/Medições/Itens, análise carregando/com erro e Resultado. O loading vem do estado real do mock; o sucesso executa apenas o mock. Não inventar um estado de carregamento de serviço ou sucesso em produção.

```powershell
npm --prefix apps/desktop run build
npm --prefix apps/desktop run test:e2e
node tools/verify-production-ui.mjs
```

Evidências completas: `outputs/phase-6.7/delivery-2/screenshots/` e `states/`. A matriz AA anterior também registra capturas de viewport e página inteira em screenshots, nos idiomas pt-BR/en e larguras 1440/1100/768/390. Capturas de página inteira preservam barras fixas na posição de viewport do navegador; conferências específicas de fim da lista validam o espaço livre abaixo da última linha. Não confundir uma imagem do documento inteiro com uma única tela física de 1000 px.

Inspeção humana de imagens complementa axe/geometria, não o contrário. Registrar problemas encontrados, até duas rodadas e limitações no relatório. NVDA/Narrador, DPI e zoom físico permanecem no checklist manual. /design-system, pseudo-localização e mock são eliminados do JavaScript de produção e verificados em verify-desktop/CI.
