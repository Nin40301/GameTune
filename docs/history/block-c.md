# Bloco C — documentação pública e comunidade

## Resumo

README curto em pt-BR e README.en.md, alfa para VM e sem uso no PC principal. Galeria versionada de Início, Itens, Revisão e Jogos em claro/escuro, com dados de demonstração identificados; as oito imagens foram examinadas. Instalação, uso básico, limites e recuperação ficaram acessíveis sem o antigo texto técnico longo.

Documentos organizados em user/dev/security/history; índices de documentação e histórico. Links relativos e referências de teste/workflow acompanham os caminhos novos. O teste de fontes do catálogo mudou somente o caminho include_str, sem alterar assertions, catálogo ou capacidades.

SECURITY.md, CONTRIBUTING.md, CODE_OF_CONDUCT.md, modelos de bug/ajuste/ajuda e PR, CODEOWNERS e Dependabot entregues. O canal privado de vulnerabilidades estava desabilitado e foi habilitado, conferido pela API como enabled true. Nenhum relato, mensagem, merge ou release foi enviado. Descrição e tópicos ficaram somente em dev/github-metadata.md para o titular aplicar.

## Comandos e validação

```powershell
node tools/check-doc-links.mjs
node tools/verify-version.mjs
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
cargo test --workspace --all-features --locked --offline
npm run format:check --prefix apps/desktop
npm run lint --prefix apps/desktop
npm run test --prefix apps/desktop
pwsh -NoProfile -File tools/verify-desktop.ps1
```

Rust 128 aprovados, cinco exclusivos de VM ignorados; componentes 24 aprovados. Clippy/fmt, lint, manifesto/CSP/versões e hashes públicos passaram. Os oito YAMLs foram carregados por parser. O verificador offline confere caminhos e caixa dos links locais; não testa URLs externas ou anchors. A UI não mudou neste bloco: a suíte e2e/axe de 48 testes do Bloco B segue como último ensaio completo, sem repetição redundante. Os guias de recuperação usam fontes oficiais da Microsoft e o relato privado segue a documentação do GitHub.

## Riscos e próxima entrega

O repositório em main ainda depende de PR/revisão do titular; esta branch não foi mesclada. Não há instalador final assinado. Guia de recuperação não equivale a teste administrativo; NVDA/Narrador, DPI alto e matriz de VM continuam pendentes. O resultado de consumo do Bloco A foi preservado, incluindo o Reduzido acima do teto. Próximo bloco: reprodutibilidade, dependências, CI e pipeline de release em rascunho.
