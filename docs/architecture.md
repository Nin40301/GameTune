# Plano de arquitetura — uma página

GameTune será um aplicativo offline Windows 10/11 com UI Tauri 2, React,
TypeScript strict e Tailwind. O primeiro incremento entrega somente a fundação
Rust, sem alterações reais no computador. A UI futura terá pt-BR/en, teclado,
leitores de tela e tema escuro/claro.

**Dependências:** `core` não depende dos adaptadores. `catalog` depende de `core`;
os seis `exec_*` dependem do contrato `Executor` do núcleo. `exec_mock` fornece
um Windows fictício compartilhado. `acceptance` verifica o conjunto. CLI, UI,
métricas e watcher serão consumidores desse núcleo nas fases seguintes.

**Domínio e autorização:** catálogo fechado, embutido e validado por schema,
SHA-256, duplicatas e uma allowlist independente de alvos exatos. O motor recebe
IDs, nunca comandos ou caminhos livres. Perfis automáticos filtram build,
edição, nível, SSD, verificação, proteção e risco. Confirmações manuais não são
serializadas em perfis. Recursos essenciais ficam bloqueados centralmente.

**Execução:** cada adaptador captura o estado sem modificar o sistema, aplica
um alvo autorizado e restaura seu snapshot. Registro preserva tipo e bytes;
serviços preservam início e execução; tarefas preservam habilitação; energia
preserva plano ativo e inventário. Estado ausente é diferente de valor zero.
Appx não admite promessa de reversão exata e fica fora da aplicação automática.

**Transação e recuperação:** `journal.wal` append-only é a fonte durável.
`Prepared` é sincronizado antes de aplicar; `Reverting`, antes de desfazer.
Estados terminais são registrados após a operação. O primeiro original é
imutável até sua reversão. Lock exclusivo impede dois escritores. Sequência e
hash encadeado detectam corrupção. Recuperação privilegia rollback; undo usa a
ordem inversa de aplicação. `journal.json` será uma projeção de leitura/exportação,
não o único arquivo de recuperação. Não há escrita real em ProgramData nesta fase.

**Privilégio mínimo:** a UI futura usa `asInvoker`; apenas um auxiliar de
aplicar/desfazer usa `requireAdministrator`. O auxiliar revalida IDs e confirmações,
protege ProgramData com DACL Administradores/SYSTEM e vincula cada transação ao
SID interativo antes de resolver HKCU em HKEY_USERS. Isto resolve a divergência
entre elevar só na ação e elevar todo o aplicativo. Nada disso é simulado como
implementação de segurança Windows nesta fase.

**Entrega e qualidade:** Rust estável, sem `unsafe`, erros tipados, lockfile e
justificativas das dependências. CI Linux/Windows: formato, Clippy, testes e
build. Fase 2 exige integração real aplicar/desfazer em VM descartável; fases
3–7 acrescentam CLI, UI, métricas, sessões por jogo e empacotamento. Segurança,
reversão e medições precedem qualquer alegação de desempenho.
