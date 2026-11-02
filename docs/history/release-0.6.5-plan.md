# Plano de execução — fechamento 0.6.5

Escopo autorizado: fechar a identidade Ametista sem regressão de segurança; nenhuma aplicação de ajustes no host. Um commit por bloco, com testes pertinentes antes de cada commit. Sem merge ou tag automática. A tag v0.6.0 é preservada.

| Ordem | Bloco | Entrega |
| --- | --- | --- |
| A / P0 | Consumo | Cinco execuções por modo, linha de base versionada, mediana/15% e teto de 200 MiB na UI; monitor mantém picos de 15 MiB/0,5%; escolha do padrão pelos dados; encerrar investigação |
| B / P0 | Identidade | Versão 0.6.5, identificador reverso estável, referências públicas e processo de release |
| C / P0 | Documentação | README pt-BR/en, galeria, recuperação, docs organizadas e arquivos de comunidade |
| D / P1 | CI | Ferramentas fixadas, auditorias, revisão de dependências, caches, permissões mínimas, release em rascunho com hashes/SBOM/procedência; preparar PR sem merge |
| E / P1 | Segurança e suporte | Pasta/hash do auxiliar, auditoria de PowerShell e duas migrações, dados antigos, logs, diagnóstico com prévia, Proteção do Sistema e inventário de unsafe |
| F / P1 | UI e AX | Páginas/hooks/state, camadas CSS, fluxos ausentes, acessibilidade, mensagens, ícones e capturas revisadas sem trocar a identidade |
| G | Transparência | Documentação dos ajustes gerada do catálogo, guia de contribuição, backlog e relatório final |

Riscos: variação entre execuções, mediana de CPU quantizada em zero, custo da WebView, troca de identificador/caminhos, instalação portátil gravável e migrações de FFI. Não há descarte por host ocupado nem atualização automática da linha de base. Os diffs de backend ficam limitados às autorizações explícitas do bloco E e à política de teste de consumo do bloco A.

P2 vai para docs/backlog.md: sugestões de ajustes e reparo SFC/DISM guiado, sem implementar novos alvos nesta entrega. Assinatura e instalador final permanecem na Fase 7. Escritas administrativas/roundtrips reais ficam no harness da VM. NVDA/Narrador, DPI físico e matriz Home/Pro exigem resultados manuais; testes automatizados não equivalem a conformidade de acessibilidade.
