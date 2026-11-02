# Plano da Fase 2

Manter a arquitetura aprovada. Acrescentar `windows-runtime` como infraestrutura
compartilhada dos seis adaptadores, sem expor comandos livres. Registro usa
winreg com tipo e bytes originais. Operações de serviços, tarefas, energia,
identidade, ACL e restauração usam scripts PowerShell fixos embutidos; parâmetros
via JSON no stdin, nunca interpolados no código. O processo fica oculto, com
timeout e saída limitada. Não há execução de scripts recebidos da UI.

Cada executor revalida o ID/alvo no catálogo embutido e usa o mesmo SID interativo
para HKCU. A sessão é a do processo solicitante, não a conta administrativa.
Leitura não exige elevação; gravação exige token elevado. ProgramData terá DACL
protegida, rejeição de reparse points, bloqueio por arquivo e escopo máquina/SID.

Appx terá serviço de domínio separado com consentimento por app, confirmação
reforçada para dependências Xbox e histórico write-ahead próprio em
`removed-apps.json`. Desfazer configurações não reinstala apps. Reinstalar abre
somente URI Store validada derivada de PFN capturado, após ação do usuário.

Pesquisar fontes oficiais por item e registrar limites de build/edição. Não
confundir validade de um nome com eficácia de política no Home. Manter itens
duvidosos fora de perfis. Aplicar Apache-2.0, conforme a escolha do titular.

Testes do sistema usam exclusivamente alvos dedicados, protegidos por feature e
opt-in de VM. A suíte normal nunca executa o catálogo real. Verificar snapshots
de registro, configuração completa de serviço, XML de tarefa, Run/StartupApproved
e inventário/configurações de energia antes, depois e após undo. Executar fmt,
Clippy, testes portáveis e testes dedicados não elevados disponíveis neste host;
documentar separadamente os testes administrativos que exigem VM descartável.
