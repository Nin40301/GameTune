# Journal — formato 2 e recuperação

`journal.wal` é JSON Lines append-only: versão 2, escopo máquina/SID, sequência,
hash anterior, entrada e SHA-256 canônico. A entrada guarda alvo, ID, tipo/bytes ou
estado completo relevante, timestamp e resultado. XML de tarefa é guardado para
detectar mudanças fora da habilitação. `journal.json` é só uma projeção de leitura.

```text
capturar -> Prepared + sync_all -> aplicar -> Applied ou Failed + sync_all
original ativo -> Reverting + sync_all -> restaurar -> Reverted ou UndoFailed
```

O primeiro original fica imutável até Reverted. Reaplicar Applied não substitui
o original. Undo usa a ordem inversa de preparação e pode ser repetido. Uma
falha parcial preserva o original e exige recuperação antes de novo lote.
Cauda incompleta é descartada após validar o prefixo; corrupção completa bloqueia
abertura. Falha de escrita envenena o escritor e impede a ação subsequente.

Executores Windows exigem `open_scoped`, com o mesmo escopo do contexto. A API de
produção abre isso exclusivamente no armazenamento protegido. Um lock global e
`scope-owner.json` bloqueiam outro usuário enquanto houver originais ativos.
O usuário anterior precisa desfazer antes de outra sessão assumir ajustes globais.

O formato 1 da fundação não tem migração automática para o formato 2. Nenhuma
operação Windows era habilitada naquela versão. Preserve journals antigos de
testes para análise; não os edite/renomeie como journal de produção. Migrações
futuras precisam ser explícitas e testadas, principalmente se um alvo mudar.

Registro: captura raw do winreg, distingue ausente/zero e guarda chaves ausentes.
Undo remove só valores criados e só exclui chaves criadas se não houver valores
nem subchaves. Conteúdo de terceiros gera conflito; não é apagado. Inicialização
preserva os bytes exatos e exige que o vínculo com Run não tenha mudado.

Serviços: não parar dependentes nem iniciar dependências silenciosamente.
Tarefas: alterar somente Enabled; XML divergente impede reativação. Energia:
restaurar o plano anterior e remover apenas o plano criado pela ação. Mudanças
externas observáveis geram Conflict; não restaurar todos os planos do Windows.

Appx usa **outro histórico**, `removed-apps.json`: nome/PFN e Prepared são escritos
e sincronizados antes de remover. Arquivo é substituído atomicamente no mesmo
volume, com lock próprio. Removed/Failed são persistidos depois. Se houver crash
em Prepared, reconciliar pelo inventário atual; não chamar isso de undo.
Uma tentativa parcialmente removida continua oferecendo os PFNs capturados.

Limites: sync_all depende do sistema de arquivos e do armazenamento. Nenhum teste
de corte elétrico foi realizado nesta entrega. Checksum não autentica contra
administrador. Ainda não há compactação/migração de histórico, retomada para
concluir lote nem política de sobrescrever mudanças externas. Restaurar valores
não apaga logs do Windows nem preserva timestamps de auditoria.
