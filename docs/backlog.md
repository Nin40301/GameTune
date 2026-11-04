# Backlog

As propostas P2 abaixo não foram adicionadas ao catálogo ou implementadas. `verified: false` é o estado de **cada proposta completa**, mesmo quando uma fonte oficial confirma a existência da função. Antes de entrar: alvo/valores/edições confirmados, risco explicado, reversão exata pelo journal, testes mock e roundtrip na VM. Não há autorização automática por estar nesta lista. P1 lista pendências de aceite de recursos já entregues ou limites ainda não resolvidos.

## P1 — antes do aceite de VM/Fase 7

- Entrega 2: Completo v2 aprovado; repetir Reduzido após interrupção por jogo reaberto e medir monitor. PR fica em rascunho até esses gates e validação manual
- Consumo: política v2 por decisão do titular em 02/11/2026 aprova o Completo nas séries finais preservadas; Reduzido é informativo. Investigação limitada encerrada em duas tentativas, sem correção comprovada. Na Entrega 2, Completo passou (156,93 MiB; CPU mediana 0%; p95 3,00% como alerta). Reduzido ficou interrompido; monitor ainda não medido. Não usar a série parcial. Ver [decisão/evidências](performance.md)
- Validar UAC em instalação protegida, hash/ACL/Authenticode, outra conta, SCM, tarefas, startup, energia, Appx, reinícios e abertura de Minecraft/Loja/Edge/Configurações/Segurança na matriz Home/Pro
- Confirmar Proteção do Sistema por volume por uma API documentada ou na ferramenta nativa. A leitura não elevada atual informa desconhecido quando não há prova; não inferir ativação de pontos antigos/VSS/frequência
- NVDA/Narrador, DPI 125/150/200%, zoom real 200/400%, alto contraste e troca de monitor: preencher [checklist](dev/accessibility-checklist.md)
- Primeiro ensaio de tag pelo titular: procedência/rascunho/SBOM/hashes, sem publicar antes de revisar. Instalador protegido e assinatura pertencem à Fase 7
- Acompanhar avisos transitivos RustSec de GTK/Tauri no lockfile; não suprimir. Repetir auditorias antes de release

## P2 — propostas de ajustes, sem implementação

| Proposta | Evidência oficial e decisão | Reversão/prova faltante | Estado |
| --- | --- | --- | --- |
| Aceleração do mouse, manual | [Configuração do mouse](https://support.microsoft.com/en-us/windows/hardware/input-devices/change-mouse-settings) e [SystemParametersInfoW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfow) documentam parâmetros. Preferência de uso, sem promessa de mira/FPS | Capturar limiares, aceleração e velocidade exatos do usuário; notificação/aplicação por sessão, comportamento com Raw Input e teste de journal | `verified: false` |
| Power Throttling/EcoQoS | [SetProcessInformation](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessinformation) e [EcoQoS](https://devblogs.microsoft.com/performance-diagnostics/introducing-ecoqos/) são APIs por processo/thread. Considerar só trabalho do próprio app; nunca abrir handle em jogo/terceiros ou desativar globalmente por suposição | Escopo e estado original documentados; energia/ruído térmico podem importar mais que velocidade. Não é um tweak global aprovado | `verified: false` |
| Suspensão seletiva USB | [Microsoft recomenda não desabilitar globalmente](https://learn.microsoft.com/en-us/windows-hardware/drivers/usbcon/usb-selective-suspend). Não propor “desative tudo”; no máximo orientação para diagnóstico específico | Identificação sem alterar drivers, AC/DC/estado por dispositivo e reversão exata; efeitos em bateria/dispositivos. Nenhum perfil automático | `verified: false` |
| Energia do adaptador de rede | [NetAdapter power management](https://learn.microsoft.com/en-us/powershell/module/netadapter/set-netadapterpowermanagement?view=windowsserver2025-ps) confirma propriedades, não um benefício em jogos. Rede e drivers permanecem protegidos; proposta apenas para estudo/guia manual compatível com esse limite | Persistência de cada propriedade, reconexão/Wake-on-LAN/Modern Standby e reversão por driver ainda não demonstradas. Sem tocar serviços de rede | `verified: false` |
| PCIe/ASPM | [Diretrizes de energia NVMe](https://learn.microsoft.com/en-us/windows-hardware/design/component-guidelines/power-management-for-storage-hardware-devices-nvme) documentam ASPM e estados de energia. Não justifica desativá-los para todos os PCs | Journal teria de preservar AC/DC e cada subconfiguração, além do GUID ativo; snapshot atual de GUIDs não basta. Sem BIOS/drivers | `verified: false` |
| Hibernação, manual | [powercfg](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options) e [hibernação](https://support.microsoft.com/en-us/windows/experience/power-battery/shut-down-sleep-or-hibernate-your-pc) explicam a função | Reativar a opção não recupera um arquivo/imagem de hibernação descartado. Não chamar essa reversão de exata; só estudar ação separada com consentimento e efeitos em Inicialização Rápida | `verified: false` |
| Otimizações de tela cheia por jogo | [Explicação oficial de Fullscreen Optimizations](https://devblogs.microsoft.com/directx/demystifying-full-screen-optimizations/) não confirma toda chave de compatibilidade encontrada em tutoriais. Não recomendar desativação geral | Falta fonte para alvo/valor suportado e reversão por usuário/executável. Sem alterar arquivos do jogo, hooks ou injeção | `verified: false` |
| Reparo SFC/DISM, ferramenta separada e guiada | [Microsoft Support](https://support.microsoft.com/en-us/windows/experience/backup-recovery/use-the-system-file-checker-tool-to-repair-missing-or-corrupted-system-files) descreve reparo de componentes, não otimização. Não executar no host nem incluir em perfil | Não é reversão exata por journal e pode exigir Windows Update/rede/fonte de imagem. Manter separado, explicar efeitos e deixar a pessoa decidir no Windows | `verified: false` |

## P2 — manutenção

- Migrar os scripts remanescentes conforme [auditoria PowerShell](dev/powershell-dependency.md): energia via APIs Power*, tarefas por COM, ACL de armazenamento por Win32 e shell por COM. Appx permanece nos cmdlets oficiais enquanto necessário
- Ensaiar CLM/WDAC sem bypass; melhorar mensagens quando a política corporativa recusa uma operação
- Revisão independente do novo FFI e dos limites de integridade; revogação offline e pin de editor quando houver assinatura definida
- Investigar apenas regressões futuras reprodutíveis contra a linha de base; a investigação anterior de GPU/WebView está encerrada. Não reabrir o redesign como “falha” pelo critério antigo

**Fora do produto:** desativar VBS/HVCI/Isolamento de Núcleo, TPM/Secure Boot, mitigações, Defender/Update/Firewall/SmartScreen/UAC; limpadores de RAM/working set/registro; tweaks TCP; drivers, DLL injection, hooks, arquivos de jogos ou handles em seus processos. Não entram como backlog implementável.


## Experimentos de UX 3.8 — fora desta entrega

Mica/Acrylic, paleta Ctrl+K, tela de atalhos e notificações de sessão não foram implementados ou habilitados. Exigem pedido/medição e aceite próprios. Ícones Appx continuam com iniciais quando o inventário não oferece imagem confiável; ampliar a leitura não é autorização para acessar dados de outros usuários. Conferir recursos variados de ícone e alto DPI no aceite manual.
