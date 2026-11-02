# Contribuir com Ametista

Abra uma proposta antes de ampliar as capacidades do app. Use a branch de trabalho e um PR pequeno, com problema, mudança, risco e validação. Ametista é alfa: escreva no sistema somente dentro do harness de VM descartável com snapshot. Não teste ajustes reais no seu host ou em jogos.

- Preserve simulação padrão, allowlist fechada, consentimentos, write-ahead journal e restauração exata
- Não adicione scripts/comandos livres, rede por padrão, coleta de dados pessoais, hooks, injeção, handles em jogos, limpeza de registro ou esvaziamento de working sets
- Não altere itens protegidos para obter desempenho. Appx permanece manual, com aviso de que Desfazer não reinstala
- Novo ajuste precisa de fonte oficial, suporte por build/edição, risco, reversão e teste. Na dúvida, `verified: false` e fora de perfis automáticos. Veja [catálogo](docs/dev/catalog.md)
- Strings da UI pertencem aos arquivos pt-BR/en. Preserve teclado, leitor de tela, contraste e preferências de acessibilidade
- Não substitua a linha de base para fazer o gate passar. Mostre resultados e justifique a mudança em PR

Requisitos e comandos estão no [guia de desenvolvimento](docs/dev/build.md). Antes de enviar: fmt, clippy -D warnings, testes Rust, ESLint, TypeScript strict/build, componentes, Playwright/axe e verificação desktop quando em Windows. Declare os testes de VM/NVDA/DPI que não executou; compilação do harness não é aceite funcional.

Contribuições são sob Apache-2.0, conforme [LICENSE](LICENSE). Preserve o titular no [NOTICE](NOTICE) e atribuições de dependências. Siga o [Código de Conduta](CODE_OF_CONDUCT.md). Vulnerabilidades seguem o [canal privado](SECURITY.md).
