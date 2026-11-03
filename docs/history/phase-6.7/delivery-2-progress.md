# Entrega 2 — verificações por bloco

## Shell e fluxo principal

Navegação agrupada, chips de status e segurança com estado desconhecido honesto; perfis com contagem/risco; CPU/RAM em janela real de 60 s e três processos de maior memória; filtros e grupos que não pré-selecionam risco alto/não verificados. Revisão apresenta os valores disponíveis e o alvo fixo do catálogo, sem executar nem autorizar pela projeção visual. Consentimentos continuam por item e ticket no backend. Resultado tem desenho finito do visto e desfazer.

Verificações: 38 testes de componentes, ESLint, TypeScript strict/build; teste observado 1–12 pt-BR/claro/1440 e matriz de oito telas desse cenário com axe sem violações. Primeira inspeção do Início corrigiu expansão excessiva do painel e separação de nome/memória. Matriz completa, capturas finais e consumo ficam no bloco de qualidade. Nenhum teste administrativo executado.

## Demais telas

Apps usa iniciais como fallback do inventário disponível e barra de revisão separada; somente a remoção final é destrutiva. Histórico tem agrupamento diário e filtros existentes. Medições apresenta os intervalos p5–p95 e medianas calculados pelo Rust e uma prévia antes do diálogo de exportação; nenhuma inferência de FPS. Configurações agrupa aparência, idioma, monitor, privacidade, suporte e textos legais locais, conferidos contra as fontes. Jogos apresenta o monitor com os mesmos consentimentos existentes, sessão e ícones locais com fallback.

Extensão nativa limitada à leitura de ícone de arquivo cadastrado, por variante tipada sem argumentos. Sem alteração de catálogo, execução, journal ou IPC elevado. Inventário agora lista 94 linhas unsafe (duas novas, Shell/GDI, recursos próprios). `png 0.18.1`, já transitivo, vira dependência direta Windows para essa imagem. `cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings` e testes Rust passaram; testes administrativos continuam ignorados sem opt-in de VM.
