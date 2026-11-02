# E se o PC ficar estranho?

Esta é uma alfa. Use uma VM descartável com snapshot anterior ao teste; não use no PC principal. Pare de aplicar novos ajustes enquanto investiga. Não apague o journal nem troque de versão para tentar esconder uma falha.

## 1. O Windows abre: use Desfazer

Abra Ametista, entre em **Histórico** e escolha **Desfaça tudo**. Confira a revisão e confirme. Para um problema isolado, use Desfazer naquele item. A operação restaura os estados registrados; se houver falha ou conflito, preserve os dados e o erro para suporte. Não edite o registro ou o journal manualmente para forçar o resultado.

Se uma sessão temporária ficou interrompida, siga a recuperação oferecida na próxima abertura antes de iniciar outra. Mantenha a conta de usuário que iniciou as alterações. O UAC deve aparecer apenas para a operação autorizada de aplicar/desfazer/remover apps; não execute a interface inteira como administrador.

**Apps removidos não voltam pelo Desfazer.** No histórico de apps, Abra na Loja permite reinstalar quando o aplicativo estiver disponível. A Loja pode precisar de internet; ela não garante recuperar os dados, a mesma versão ou o provisionamento anterior. Isso não torna a remoção automaticamente reversível.

## 2. O problema continua: Restauração do Sistema

Se houver um ponto anterior ao teste, abra **Painel de Controle → Recuperação → Abrir Restauração do Sistema**. Escolha o ponto e confira os programas afetados antes de concluir. O Windows reinicia ao aplicar a restauração. Esse recurso recupera configurações e arquivos do sistema; não substitui um backup pessoal. Consulte as [instruções oficiais de Restauração do Sistema](https://support.microsoft.com/en-us/windows/experience/backup-recovery/system-restore).

A disponibilidade depende da Proteção do Sistema e dos pontos existentes. Ametista não deve ativar essa proteção silenciosamente nem prometer um ponto que o Windows não criou. [Proteção do Sistema](https://support.microsoft.com/en-us/windows/experience/backup-recovery/system-protection).

## 3. O Windows não abre: Ambiente de Recuperação

Use o **Ambiente de Recuperação do Windows (WinRE)**. Quando disponível, selecione **Solução de Problemas → Opções avançadas → Restauração do Sistema** e escolha um ponto anterior. Um dispositivo criptografado pode exigir a chave de recuperação do BitLocker; mantenha-a fora da VM/PC antes do teste. Não publique essa chave em issues. [WinRE e formas de acesso](https://support.microsoft.com/en-us/windows/experience/backup-recovery/windows-recovery-environment).

Se o teste ocorreu em VM, desligue a VM conforme o procedimento do hipervisor e volte ao snapshot anterior. Não use redefinição/formatação como primeiro passo: essas opções têm consequências diferentes e podem remover dados. [Opções oficiais de recuperação](https://support.microsoft.com/en-us/windows/experience/backup-recovery/recovery-options-in-windows).

## Pedir ajuda

Informe versão do Ametista, build/edição do Windows, item selecionado, se houve reinício e o resultado do Desfazer. Revise e anonimize qualquer diagnóstico antes de anexar. Não envie journal bruto, nomes de usuário, caminhos pessoais, chaves, certificados ou tokens. Para vulnerabilidades, use o [canal privado](../../SECURITY.md), não uma issue pública.
