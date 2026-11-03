# Marca e ícones locais

Fontes vetoriais em `apps/desktop/src/assets/brand`: `logo-light.svg`, `logo-dark.svg`, `app.svg`, `tray.svg`. As quatro faces da gema preservam o desenho do BrandMark da interface. Sem imagens externas, emoji, filtros SVG ou fontes remotas.

```powershell
node tools/generate-icons.mjs
```

Usa o Chromium do Playwright já instalado como ferramenta de desenvolvimento para rasterizar os SVGs locais. Produz `icon.ico` e `tray.ico` com quadros PNG em 16,20,24,32,40,48,64,128,256 px e PNGs para inspeção. O formato ICO é conferido pelo leitor de imagens e pelo compilador de recursos do Windows. O app não inclui Playwright nem renderiza ícones em runtime.

O monitor declara PerMonitorV2 e carrega o tamanho pequeno apropriado ao DPI da sua própria janela (16/20/24/32 para 100/125/150/200%). Recarrega ao receber WM_DPICHANGED, mantém o handle em GWLP_USERDATA e libera o recurso ao trocar/encerrar; não carrega um ícone novo a cada timer. O timer apenas atualiza a bandeja existente. Não há handles ou alterações em jogos.

Geração em vários tamanhos não comprova nitidez física em todas as escalas. A inspeção PNG automatizada/local foi feita; teste de bandeja real em 125/150/200%, alternância de monitores e temas de contraste continua no checklist manual. Não declarar essa matriz validada sem os resultados do titular.
