# Journal e recuperação

O formato canônico de escrita é `journal.wal`, uma sequência JSON Lines UTF-8.
Cada registro contém versão, sequência, hash anterior, entrada completa e
SHA-256 do payload canônico. As entradas contêm chave normalizada, ID, alvo,
snapshot tipado, instante RFC 3339 UTC, status e erro tipado opcional.
`Journal::export_json` gera a projeção padrão JSON do modelo pedido para
`journal.json`. A projeção não participa da recuperação. Manter um WAL separado
evita reescrever um array inteiro no caminho crítico de cada alteração.

Fluxo de aplicação:

```text
capturar estado -> Prepared + sync_all -> alterar -> Applied + sync_all
                                                -> Failed + sync_all
```

Fluxo de reversão:

```text
Prepared/Applied/Failed/UndoFailed -> Reverting + sync_all -> restaurar
Reverting -> Reverted + sync_all | UndoFailed + sync_all
```

Uma falha pode ocorrer após uma alteração parcial. Por isso `Failed` continua
com o original e exige recuperação. Reaplicar um item `Applied` não grava nem
substitui o original. Após `Reverted`, uma nova aplicação captura um novo
original. Desfazer novamente não altera nada. A ordem de rollback é a inversa
da preparação, inclusive após reabrir o arquivo.

`Journal::open` adquire lock exclusivo do sistema operacional antes de ler.
Valida sequência, checksum, identidade do original e transições. Uma linha
completa inválida bloqueia abertura e mantém o arquivo. Uma cauda sem newline é
uma gravação interrompida: o prefixo completo é validado antes de truncar a
cauda e sincronizar. A UI futura deve mostrar `discarded_torn_tail` ao usuário.
Falhas de escrita envenenam o escritor; ele deve ser reaberto antes de continuar.

O motor não confia em alvos do journal: resolve o ID no catálogo embutido e
compara o alvo. Mudanças de alvo em uma atualização exigem migração revisada.
Checksum não é autenticação contra um administrador malicioso; a DACL da Fase 2
é obrigatória. Não importe journals recebidos de terceiros.

Limites desta fase:

- Testes reproduzem janelas de crash truncando registros e mantendo o estado
  fictício já alterado. Não equivalem a testes de corte de energia.
- `sync_all` depende das garantias do sistema de arquivos e do armazenamento.
  A durabilidade da criação do diretório/arquivo precisa ser verificada no
  adaptador Windows antes de habilitar ações reais.
- Não há migração entre versões, journal por SID, ACL, backup de recuperação,
  retomada para concluir o lote ou limite/compactação do WAL nesta fase.
- O chamador fornece um diretório de teste existente. Nenhum teste modifica
  `%ProgramData%`, registro, serviços, tarefas ou energia do host.
- Estado original de registro inclui a lista de chaves ausentes antes da ação;
  o executor real deverá remover só chaves criadas e ainda vazias, sem apagar
  dados criados por terceiros. Alterações concorrentes do Windows devem gerar
  conflito visível, em vez de alegar restauração exata sem comparação.

Referência: [Rust File::sync_all e locking](https://doc.rust-lang.org/std/fs/struct.File.html).
