# Harness manual da interface em VM descartável

Este roteiro é exclusivo da VM. Crie snapshot do hipervisor, instale WebView2,
prepare proteção do sistema/checkpoint, desligue rede e use usuário sem dados
pessoais. Não faça o ensaio no host. Restaure o snapshot ao terminar, com sucesso
ou falha. O código não confirma a existência de snapshot do hipervisor.

1. Compile/copie UI e helper de produção com features padrão, na mesma pasta.
   Rode `tools/verify-desktop.ps1` no checkout para conferir manifestos e assets.
2. Abra **ametista.exe pelo usuário comum**. Confirme ausência de UAC na abertura,
   funcionamento de Simule, progresso e cancelamento. Registre edição/build,
   usuário/SID de teste, screenshot e resultados. Não use dados pessoais.
3. Selecione somente `reg.edge_preload` em Itens. Antes, capture no PowerShell
   da VM valores/tipos/subchaves/ACL de
   `HKLM\SOFTWARE\Policies\Microsoft\Edge` e ausência de ancestrais se for o caso.
   Guarde a evidência completa junto do snapshot do hipervisor.
4. Revise, confirme e aplique. Deve surgir UAC apenas ao autorizar a operação.
   Teste primeiro cancelando UAC: nenhum ajuste deve mudar, e o ticket precisa
   de nova revisão. Depois autorize e confira DWORD StartupBoostEnabled=0,
   journal original e término do helper.
5. Desfaça o item por Histórico, com nova revisão/UAC. Compare snapshot completo
   do registro com o anterior, incluindo ausência/tipos/ACL e outras entradas.
6. Repita usando credenciais de **outra conta administrativa** no UAC. HKCU,
   quando testado com um item permitido de usuário, deve continuar no SID do
   desktop comum. Pipe de outro SID deve ser negado; use também a suíte IPC normal.
7. Teste fechar a janela durante espera: não deve haver retry automático; abra
   novamente, carregue histórico e recupere. Nunca apague WAL para contornar falhas.
8. Em snapshot separado, selecione apps reais instalados do catálogo. Confirme
   consentimento separado de cada um e avisos de Minecraft/Game Pass para risco
   alto. Registre nome/PFN antes de remover e inventário AllUsers/provisionado
   depois. Desfazer deve rejeitar a promessa de reinstalar. Reinstalar só abre a
   página validada da Store após revisão. Para concluir instalação, rede só na VM
   e por solicitação explícita; restaure o snapshot para recuperar dados originais.
9. Teste teclado, leitor de tela NVDA/Narrador, escala 100/150/200%, alto contraste,
   dark/light e pt-BR/en. Axe automatizado complementa e não substitui essa revisão.

Testes de fixtures continuam separados:

```powershell
$env:AMETISTA_DISPOSABLE_VM='I_HAVE_A_SNAPSHOT'
cargo test -p ametista-windows-runtime --features vm-tests --test windows_vm --locked --offline -- --ignored --test-threads=1
cargo test -p ametista-windows-runtime --features vm-tests --lib dedicated_hkcu --locked --offline -- --ignored --test-threads=1
```

O ensaio Safe/Balanced com dois reinícios continua em windows-real-profiles.md.
Não execute globalmente todos os testes ignorados sem escolher a suíte.

**Situação da entrega:** compilação e testes IPC não elevados executados; roteiro
UAC/escritas/Store, outra conta e matriz Windows Home/Pro ainda não executados.
