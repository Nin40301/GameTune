# Sistema visual Ametista

Paleta lilás/branco, gema e Plus Jakarta Sans variável local permanecem a identidade. CSS declara layers `tokens, base, components, screens`; tokens não contêm layout. Preferências resolvidas em html: `data-theme=light|dark`, `data-effects=full|reduced`, `data-motion` e tamanho de texto em passos de 10%. Sem estilo livre vindo de perfil e sem relaxar CSP.

| Escala | Tokens e valores |
| --- | --- |
| Espaço | space-1…7: 4, 8, 12, 16, 24, 32, 48 px |
| Texto | fs-xs 12 px (selos), fs-sm 13,5 px, fs-md 15 px, fs-lg 17 px, fs-xl 20 px, fs-2xl 26 px, fs-3xl 34 px |
| Raios | r-sm 10, r-md 16, r-lg 24 px; pill 999 px |
| Elevação | elev-1 controles/linhas, elev-2 vidro, elev-3 Orb/diálogo; nenhuma sombra fora desses três níveis |
| Movimento | dur-1…4: 120, 200, 320, 480 ms; ease-out (.22,1,.36,1), ease-spring (.34,1.56,.64,1) |

Texto principal usa ink #1F1A33 no claro e #F4F1FF no escuro; procurar contraste ≥7:1 em superfícies reais, AA mínimo em todos os controles. Roxo de botão #7C3AED→#6D28D9 com branco; #8B5CF6 nunca é fundo para texto branco. Não afirmar AAA sem conferir o estado renderizado e o roteiro manual.

No máximo três superfícies com backdrop-filter por tela, sem blur aninhado. Listas usam fundo, sem blur. Efeitos reduzidos/transparência reduzida e forced-colors tornam o vidro sólido. Movimento interpola só transform/opacity; visto de sucesso tem exceção pontual de traço. Sem animação contínua após carregar. Redução de movimento prevalece sobre entrada, mola e brilho.

`/design-system` é referência de desenvolvimento com controles/estados e dois temas. O import é guardado por import.meta.env.DEV; build de produção remove rota/chunk. `tools/verify-production-ui.mjs` e verify-desktop recusam marcadores de desenvolvimento, pseudo-localização e mock nos assets. Testes visuais Linux geram a referência canônica; fonte nunca vem de CDN.
