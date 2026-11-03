# Entrega 1 / bloco visual 3.1

Identidade preservada: lilás/branco, gema, Plus Jakarta Sans local, vidro, Orb e botões. Não implementa as partes 3.2–3.8, não adiciona preferências ou backend. A versão do produto continua 0.6.5; 6.7 é a fase de fechamento.

| Defeito | Correção | Regressão automatizada |
| --- | --- | --- |
| 1 Banner colado | Classe explícita flex/gap, contador sem encolher, texto quebra e suporta expansão | observed-defects: texto 5× maior e distância dos retângulos |
| 2 Faixa vazia | StatusStrip com Proteção do Sistema e privacidade; atalhos para Configurações; desconhecido não vira ativo | observed-defects: lista/atalhos reais; axe de todas as telas |
| 3 Barra flutuante/vazamento | ActionBar em portal fora do transform da página, fixa na borda; máscara acima, superfície opaca, padding de leitura; no celular respeita a navegação inferior | observed-defects: posição/opacity/última linha; design: quatro larguras |
| 4 Contagem repetida | Removida do cabeçalho, mantida na barra de Itens | observed-defects: sem contador no cabeçalho |
| 5 Voz | Botões pt-BR no infinitivo, opções de catálogo/jogos com nomes substantivos na camada i18n, inglês revisado e menu da bandeja alinhado | VisualHistory: rótulos dos 86 itens/cinco ações/idiomas e bandeja; check-i18n |
| 6 Lupa em Simular | ScanEye para visualização; busca conserva Search | observed-defects: ícone específico do botão |
| 7 Orb arredondado | Intl com uma casa decimal, espaço entre unidade e valor e legenda disponíveis de total | observed-defects: nomes acessíveis pt/en; componentes de métricas |
| 8 CPU/alturas | Janela visual de 60 s por timestamp, sem limitar a 30 amostras; cards com alturas iguais/stretched | VisualHistory: 60 amostras em 1 Hz, pausa/lacuna e duplicata; observed-defects: SVG e alturas |
| 9 Slogan órfão | Windows em foco, balanceado | observed-defects: balance; inspeção 1440/390 |
| 10 Letras pequenas | Corpo 15 px, texto secundário ≥13,5 px, sem texto abaixo de 12 px; perfil móvel em duas colunas para acomodar rótulos | observed-defects: tamanho de descrições, axe/reflow; capturas |
| 11 Coleta como texto solto | Button ghost com Pause/Play | observed-defects: variante e ícone; teste de pausa sem operação |
| 12 Jogos sem contexto/checkboxes | Switches de ação/perfil, checkbox só de consentimento; estado do monitor, três passos e atalho na biblioteca | observed-defects e Games: papéis/estado/consentimento; axe |

As descrições/IDs/valores/risco do catálogo permanecem intactos; os novos nomes são apenas apresentação. Startup dinâmico mantém fallback de nome fornecido pelo backend. Nenhuma seleção, autorização, aplicação ou regra de perfil foi copiada para a UI. A fixture de testes agora avança timestamp para permitir sparkline; o código de produção não contém essa fixture. Não foram inventadas amostras para preencher o gráfico.

## Testes e imagens

33 componentes em oito arquivos passaram, com 487 chaves iguais pt-BR/en e 32 JSX sem prosa fixa. Strict TS/build, ESLint, format, versão e verificação desktop passaram. Rust do bloco anterior: 136 passaram, cinco VM ignorados; nenhuma operação administrativa foi rodada. Suite e2e tem 74 casos únicos. Primeira rodada: 72 passaram e dois seletores em inglês ainda usavam o texto antigo de anonimização; corrigidos, 23 casos afetados passaram novamente. Oito regressões dos doze defeitos passaram também após as capturas finais. CI executará a suíte completa, sem somar reruns como testes distintos.

[Capturas e hashes](../../assets/screenshots/6.7/manifest.json): Início/Itens/Revisão/Jogos, claro/escuro, 1440/390, páginas inteiras; Itens também em viewport inicial e final para provar a barra encostada. As imagens completas conservam a posição inicial das barras fixas, como o Playwright as capturou; não foram retocadas. A prova de última linha usa scroll e screenshot separados. Dados de demonstração identificados na UI, nunca medição do host.

Inspeção cobriu cada imagem e todas as faixas das páginas longas. Segunda rodada: sparkline faltava porque a fixture repetia o mesmo timestamp; corrigida só no mock, capturas aguardam o segundo ponto. Espaço da unidade do Orb foi aberto. Fragilidades reservadas à Entrega 2: cluster de selos denso, telas longas no celular e hierarquia do editor de Jogos. Não declarar “AAA” ou conformidade manual nesta entrega.

## Comandos e pendências

```powershell
npm run format:check --prefix apps/desktop
npm run lint --prefix apps/desktop
npm test --prefix apps/desktop
npm run build --prefix apps/desktop
npm run test:e2e --prefix apps/desktop
node tools/check-i18n.mjs
pwsh -NoProfile -File tools/build-release.ps1 -SkipFrontend
pwsh -NoProfile -File tools/verify-desktop.ps1
```

Consumo: aprovação v2 reavalia as séries preservadas; nenhuma nova coleta nativa deste polimento é apresentada como realizada. A Entrega 2 exige nova coorte. Monitor permanece no orçamento estrito. NVDA/Narrador, DPI/zoom físicos, alto contraste físico, UAC e reboots/roundtrip reais permanecem pendentes. Próximo: CI fixado e revisão do PR; somente após merge pelo titular, criar design/aaa da main.
