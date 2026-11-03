# Logs e diagnóstico offline

Em Configurações, a seção Suporte e recuperação permite conferir a Proteção do Sistema, abrir a pasta de logs e revisar um diagnóstico. Nenhuma dessas ações aplica um ajuste.

## Confira a proteção

O app pode identificar uma política que desativa a proteção, mas a leitura sem elevação nem sempre prova que ela está ativa em cada volume. Nesse caso, mostra **não foi possível confirmar**, sem deduzir que pontos antigos ou VSS garantem proteção. Confira na ferramenta oficial do Windows pelo botão. O app abre somente `SystemPropertiesProtection.exe` do diretório do sistema e não ativa nada sozinho. Antes de aplicar, o motor continua exigindo a criação/verificação do ponto de restauração.

## Abra os logs

Os logs ficam em `%ProgramData%\Ametista\logs`, com ACL de Administradores/SYSTEM. A rotação limita cada arquivo a 256 KiB, mantém no máximo cinco arquivos e remove arquivos com mais de 14 dias quando ocorre uma nova escrita. Cada registro tem horário, evento tipado e sucesso/falha. Não registra nome de usuário, caminho de app, comando livre ou valores originais do registro. Falha de log não impede recuperação pelo journal.

A pasta pode não existir antes de uma operação administrativa. Um usuário sem permissão pode não conseguir abrir/ler esses arquivos. O app não altera a ACL nem pede elevação só para exportar diagnóstico. O diagnóstico informa o que ficou indisponível.

## Revise antes de salvar

1. Selecione **Revise o diagnóstico**
2. Leia a prévia: versão, build/edição do Windows, estado conhecido da proteção, última análise concluída, resumo do journal e logs disponíveis
3. Confirme o conteúdo e escolha onde salvar o ZIP na janela nativa

A prévia é um snapshot limitado, com token de uso único e validade curta. Para atualizar o conteúdo, gere outra prévia. O app não sobrescreve um arquivo existente e não envia nada pela rede.

O ZIP exclui nomes de usuário, caminhos e valores originais. Journal vira um resumo por ID/status; análise contém apenas campos permitidos; logs só incluem o formato tipado reconhecido. Dados protegidos sem acesso e registros fora desse formato não são copiados. **Não inclui dumps de memória, lista de processos de terceiros, certificados, tokens ou o journal bruto.** Confira também o ZIP antes de compartilhá-lo; nenhuma anonimização deve ser tratada como autorização automática para publicar.

O último resultado de análise pode estar ausente se a coleta foi cancelada ou nunca terminou. Isso é declarado na prévia, sem inventar uma análise. A exportação não substitui backup/snapshot ou recuperação do Windows.

## Dados do nome antigo

Se Ametista encontrar `%ProgramData%\GameTune`, mostra uma tela de orientação e recusa iniciar escritas. Não renomeie, apague, mescle ou edite journals para contornar esse bloqueio. Preserve os dados, anote a versão que os criou e consulte o [guia de recuperação](recovery.md). Não há migração automática nesta alfa.

[Volte ao guia da interface](desktop.md) · [Relate vulnerabilidade em privado](../../SECURITY.md)
