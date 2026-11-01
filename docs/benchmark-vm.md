# Aceite do benchmark em VM

Use somente VM descartável com snapshot do hipervisor e siga `desktop-vm.md`.
Desligue rede. UI pelo usuário comum, auxiliar elevado só após revisão. Não use
o roteiro no host. Nenhum ensaio aplicar/reiniciar/desfazer foi autorizado no host.

1. Registre build/edição, CPUs, RAM, GUID de energia, estado completo dos alvos e
   snapshot do hipervisor. Compile release padrão e verifique manifestos/CSP.
2. Confirme que painel consulta valores reais e não pede UAC. Compare com
   ferramentas Windows lembrando a diferença entre memória disponível, working
   set compartilhado e memória privada. Pause/retome a coleta. Rode os dois gates
   de recursos de `benchmark.md`; qualquer violação reprova, sem ignorar filhos.
3. Faça 2 rodadas Antes de 60 s. Feche/reabra: 2 preservadas. Inicie outra rodada,
   simule crash **somente do GameTune** na VM, reabra: parcial descartada e 2 salvas.
   Complete 5; a primeira não deve entrar nas 4 medianas usadas na comparação.
4. Em novo snapshot, interrompa uma rodada suspendendo/retomando a VM. Ela deve
   falhar por lacuna/cobertura, sem virar uma rodada completa. Durante coleta,
   aplicação via bridge deve ser rejeitada, sem lançar UAC.
5. Após 5 Antes, use somente itens permitidos, revisados e reversíveis. Exclua
   o item de plano de energia deste protocolo. Capture antes/depois, aplique na
   VM, confirme IDs efetivamente aplicados. Antes de reiniciar, iniciar Depois
   deve falhar. Reinicie manualmente, reabra e faça 5 Depois com mesmo GUID.
6. Exporte JSON/HTML anonimizados. Confirme 5 por condição, primeira warmup,
   4 medianas usadas, p5/p95/MAD e inconclusivo quando aplicável. Abra HTML offline
   sem rede, verifique ausência de scripts/recursos remotos/nomes e caminhos.
7. Em snapshot separado, mude plano no meio de uma rodada: rejeitar. Depois deve
   exigir GUID original. Não permita troca silenciosa pelo coletor.
8. Opcional: indique instalação PresentMon já fornecida pelo usuário. Importe
   CSV v1 de uma única swap chain e confira fórmulas contra os intervalos.
   CSV misturado, NaN e formato incompatível devem falhar. O app não executa a
   ferramenta nem abre processos do jogo. Teste diálogos cancelados e overwrite.
9. Desfaça na VM pelo fluxo existente, compare snapshot completo e restaure
   snapshot do hipervisor. Registre também Narrador/NVDA, teclado e escalas DPI.

O teste sintético `persisted_full_protocol_survives_reboot_and_excludes_warmup_outliers`
valida persistência/reboot lógico em CI, mas não substitui esse aceite real.
