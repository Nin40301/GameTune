# Checklist manual de acessibilidade

Automação não comprova conformidade WCAG. Preencher os resultados abaixo na build candidata, **na VM** para fluxos com escrita. Registrar commit/hash, Windows/edição, WebView2, navegador, tamanho da janela, idioma, tema, efeitos, DPI e tecnologias assistivas. Nesta entrega, **NVDA, Narrador e DPI físico ainda não foram validados**.

Fase 6.7 / Entrega 2: repetir manualmente os atalhos da StatusStrip, switches de Jogos e navegação entre a barra fixa de Itens e a barra móvel. O novo texto secundário tem mínimo de 13,5 px. Os testes de geometria e axe não preenchem os resultados manuais abaixo; o objetivo “AAA” de acabamento não é uma declaração de conformidade WCAG AAA.

| Ensaio | Roteiro e resultado esperado | Resultado do titular |
| --- | --- | --- |
| Somente teclado | Tab/Shift+Tab, Pular para o conteúdo, navegação + Mais no celular, setas/Home/End em Segmented e cartões de perfil, Espaço em Checkbox/Switch, Escape em detalhes, foco de retorno | Pendente |
| Troca de tela | Foco no h1, anúncio da nova página, sem anúncio repetido do mesmo nome em títulos/banners | Pendente |
| NVDA | Ler Início, Itens, Revisão, Apps, Jogos, Medições, Histórico, Configurações e onboarding; nomes/papéis/estados corretos, tabelas com caption e cabeçalhos | Pendente |
| Narrador | Repetir o roteiro anterior; simulação deve anunciar progresso no máximo uma vez a cada 3 s e conclusão/cancelamento, sem ler cada alvo | Pendente |
| Erros e sucesso | Erro usa alert com próximo passo; sucesso usa status. Nenhum aviso informa alteração bem-sucedida quando o helper falhou | Pendente |
| Zoom 200% | Janela 1280 px, zoom real do WebView/navegador para 200%; conteúdo reflui, rótulos e ações continuam acessíveis, sem rolagem horizontal do documento | Pendente |
| Zoom 400% | Repetir em 400% (viewport CSS equivalente a 320 px); tabelas podem ter região própria de rolagem, a página não; barra inferior não cobre o último item | Pendente |
| Texto 200% | Aumentar só texto; nomes longos pt-BR/en quebram sem corte, checkbox e rótulo continuam associados | Pendente |
| DPI 125/150/200% | Windows nas três escalas; UI, UAC, file dialogs e ícone da bandeja nítidos, sem mudar execução/ACLs para o ensaio | Pendente |
| Alto contraste | Temas de contraste do Windows claros/escuros; seleção, risco, foco, checkbox, switch, Orb e gráficos continuam identificáveis sem depender só de cor | Pendente |
| Menos movimento | Ativar preferência do Windows, navegar/analisar; sem animações longas, movimento de progresso não é necessário para entender o estado | Pendente |
| Menos transparência | Preferência suportada e opção Reduzido: vidro sólido, sem blur; não confundir com aprovação de consumo do modo | Pendente |
| Contraste | AA em texto/componente, claro/escuro, hover, foco, selecionado e desabilitado; verificar os casos incompletos reportados pelo axe sobre gradientes | Pendente |
| Alvos de toque | ≥40 px para botões/summary/segmented/switch; Checkbox pode usar todo o rótulo clicável ≥40 px | Pendente |
| UAC/Store/reinício | Só na VM: cancelar UAC não aplica; Appx exige cada consentimento; Desfazer não reinstala; botão de reinício só abre a escolha do Windows | Pendente |

Playwright verifica axe AA nos dois idiomas/temas, foco e layouts 1440/1100/768/390. O ensaio de reflow automatizado em viewports CSS 640/320 representa a geometria de zoom 200/400% sobre 1280 px; **não substitui o zoom real no WebView ou os ensaios de leitores de tela/DPI acima**. Evidências incompletas não são marcadas como aprovação.


## Evidências automatizadas da Entrega 2

A suíte inclui pseudo-localização de pelo menos +40% nos dois idiomas, temas e 1440/390 px, incluindo editor de Jogos e descrições do catálogo; disclosures Agora→Depois; fluxo completo simular→revisar→aplicar **no mock**→resultado→desfazer somente com Tab/Enter/Espaço; reflow equivalente em 640/320 px; axe nos dois temas/idiomas e na galeria de estados. Contraste de ink/base é testado ≥7:1, sem extrapolar esse número para cada pixel do vidro. Bordas de switches preservam o limite visível em ambos os temas.

Não substituir “Pendente” na tabela pelos resultados do axe. Zoom geométrico não mede zoom físico do WebView. Rodar NVDA/Narrador em Itens recolhidos/expandidos, painel de consentimentos e região de progresso; confirmar que os anúncios limitados a 3 s não atrapalham os consentimentos. Verificar foco no menu Mais enquanto a barra de revisão está visível. Efeitos Automático deve acompanhar redução de transparência; Movimento reduzido deve remover também o desenho do visto, sem ocultar o resultado.
