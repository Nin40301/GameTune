# Aceite de perfis de jogos em VM descartável

**Não execute no host.** Use snapshot do hipervisor, desktop interativo de usuário
comum, WebView2 instalado, rede desligada, sem dados pessoais. Restaure o snapshot
ao terminar. Compile release padrão; mantenha UI/helper/monitor na mesma pasta.
O ensaio não cria tarefa/serviço de autostart e não altera políticas de auditoria.

1. Capture estado completo dos alvos selecionados (valor/tipo/ausência/ACL do
   registro e ancestrais, GUIDs/atributos de energia, Start/DelayedAutoStart e
   execução dos serviços) e snapshot da VM. Faça apenas o teste dedicado
   `test-game-events.ps1` primeiro, com processo nosso de 6 s.
2. Adicione `synthetic_game.exe` como jogo. Escolha Modo de Jogo (Win11) e/ou
   Alto Desempenho, leia os avisos, confirme, habilite e salve. Inicie monitor
   em Configurações. **Sem jogo, não deve haver UAC nem mudança de alvo.**
3. Abra o sintético por si mesmo ou adapte sua duração na VM. Cancele UAC na
   primeira vez: nenhum ajuste aplicado, erro visível, sem nova tentativa em
   loop. Pare/reinicie monitor, repita e autorize. Confira o WAL Prepared antes
   do efeito Windows, auxiliar vivo só durante sessão e estado aplicado.
4. Feche a interface mantendo monitor/jogo. A sessão deve continuar. Termine
   o processo sintético pelo seu encerramento normal; compare snapshot completo
   após rollback. Repita com duas instâncias e dois executáveis com ações
   compartilhadas: não restaurar uma ação enquanto o último usuário estiver vivo.
5. Repita com `svc.fax` **somente na VM**, se instalado e seguro parar. Confirme
   que StartMode/DelayedAutoStart não mudam durante a pausa e o estado rodando/
   parado original volta. Dependentes ativos: falhar sem cascata. Recursos
   ausentes: mostrar ignorado, sem abortar itens compatíveis nem inventar sucesso.
6. Durante ação temporária, aplicar/desfazer permanente deve falhar por lock,
   preservando originais. Se ajuste permanente existia antes, desfazer temporário
   deve voltar àquele estado vigente; depois desfazer permanente volta ao inicial.
7. Crash **somente do monitor criado neste ensaio** na VM: helper deve tentar
   reverter após EOF/prazo e encerrar. Reinicie o monitor: aviso/recuperação antes
   de nova sessão. Crash do auxiliar/reboot da VM: WAL persiste e nova execução
   exige UAC para rollback. Compare snapshot completo antes/depois. Não apague WAL.
8. Inspecione SID interativo usando outra conta administrativa no UAC. HKCU
   deve continuar no hive do usuário do desktop. Outro SID não pode usar pipe.
   Replay/mensagem fora da sequência/payload malformado devem provocar rollback.
9. Interrompa WMI temporariamente **apenas na VM** usando mecanismo controlado
   do harness; não desabilite permanentemente serviços. Erro de fonte ou fila
   interrompida deve devolver o estado e bloquear novas aplicações até revisão.
10. Meça consumo separado com `test-resource-budget.ps1`, limites fechados e
    todos os descendentes. No monitor, execute também fase com jogo sintético
    observado (sem ajustes no gate somente leitura), e faça medição separada
    de **auxiliar elevado e filhos PowerShell durante aplicar/reverter**. UAC
    pode quebrar parentesco de processo: some explicitamente o PID autenticado
    do auxiliar e seus filhos em evidência adicional; não omita esse custo nem
    declare que o gate do monitor idle representa a sessão elevada inteira.
11. Validar CPU total incluindo `WmiPrvSE` compartilhado, cadência/latência,
    Narrador/NVDA, bandeja com teclado, reinício do Explorer, idiomas, temas,
    DPI 100/150/200%, caminhos com caracteres internacionais, usuário Home/Pro,
    jogos protegidos com caminho indisponível e Minecraft Launcher/Bedrock.

Automação normal testa detector, concorrência lógica, lock e crash real de um
**executor simulado em arquivo temporário**. Não instala serviço nem toca em
alvos do catálogo. Os itens acima dependem do opt-in e snapshot real de VM;
não foram executados no host e não são substituídos pelos testes sintéticos.
