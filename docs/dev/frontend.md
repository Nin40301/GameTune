# Organização da interface

`App.tsx` só compõe provider/modelo e shell. `state/useAppModel.ts` mantém análise, seleção, revisão e respostas do mesmo backend; nenhuma regra de execução foi copiada do Rust. `state/AppContext.tsx` oferece esse estado às páginas. Preferências ficam em `hooks/usePreferences.ts`; atividade da janela e anúncios limitados têm hooks separados.

As oito telas ficam em `pages/`, com uma tela de resultados e o onboarding. Os painéis de jogos/medições continuam componentes reutilizados em Início e Configurações; medição e perfis permanecem no backend original. Diálogos, itens, revisão, estado e controles ficam em `components/`. Trocar a página foca o h1 e anuncia o título; a análise é cancelável e não anuncia nomes de alvos.

CSS usa `@layer tokens, base, components, screens`: paleta e aliases; reset/tipografia; controles e superfícies; composição de telas e preferências de acessibilidade. `polish.css` foi incorporado e removido. Tailwind 4 só fornecia reset/build, sem utilitários usados nos componentes; foi removido, junto com o plugin Vite. O reset local cobre os elementos usados, e as capturas/axe verificam a preservação da aparência. A regra local `sr-only` é explícita.

`InlineStatus` padroniza mensagens e ação seguinte. Números/plurais/datas usam Intl; `tools/check-i18n.mjs` exige chaves iguais pt-BR/en, verifica referências estáticas e recusa prosa fixa em JSX/atributos de acessibilidade. Strings internas de protocolo, IDs e classes não são textos de interface. O verificador não prova a qualidade da tradução e não substitui revisão humana.

```powershell
node tools/check-i18n.mjs
npm test --prefix apps/desktop
npm run test:e2e --prefix apps/desktop
```

O backend de produção não contém fixtures. Os testes antigos usam uma fixture explícita de pessoa que já concluiu onboarding; os novos testes de primeira execução não usam esse atalho. As capturas de documentação são identificadas como dados simulados, nunca medições reais do host. Ver [checklist manual](accessibility-checklist.md).
