# O que cada ajuste faz

Esta página é gerada do catálogo da Ametista. Simule primeiro e confira o estado real do computador. Um item listado não está necessariamente instalado ou disponível na sua edição do Windows.

Appx fica sempre fora dos perfis automáticos. Risco alto e itens não verificados nunca são pré-selecionados. A lista de proteção pode bloquear um item mesmo que ele esteja documentado aqui.

86 itens no catálogo. Fonte: [items.json](../../catalog/items.json).

## Desativar Copilot clássico

`reg.copilot` · `registry`

- **O que muda:** Desativa o Copilot clássico por política. O app Copilot independente pode não ser afetado. Desfazer restaura tipo e bytes originais ou remove o valor criado
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O painel clássico do Copilot pode ficar indisponível
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-windowsai#turnoffwindowscopilot)

## Ocultar botão do Copilot

`reg.copilot_button` · `registry`

- **O que muda:** Oculta o botão do Copilot na barra de tarefas. Desfazer restaura tipo e bytes originais ou remove o valor criado
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O atalho visual do Copilot fica oculto
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Não verificado — não entra em perfis automáticos
- **Edições declaradas:** pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: Não verificado — não entra em perfis automáticos

## Desativar análise do Recall

`reg.recall` · `registry`

- **O que muda:** Solicita a desativação da análise de IA do Recall. Desfazer restaura tipo e bytes originais ou remove o valor criado
- **Risco:** Alto — seleção manual e confirmação reforçada
- **O que pode deixar de funcionar:** Pode apagar o histórico existente do Recall. Restaurar o registro não recupera esse histórico. Ação bloqueada
- **Como voltar:** Não há restauração automática garantida. Confira riscos e fontes; a proteção pode recusar este item
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** pro, enterprise, education
- **Build mínimo:** 26100
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-windowsai#disableaidataanalysis)

## Remover Microsoft.Copilot

`appx.microsoft.copilot` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Remover Microsoft.Windows.Ai.Copilot.Provider

`appx.microsoft.windows.ai.copilot.provider` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Seguro
- **Verificação:** Não verificado — não entra em perfis automáticos
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: Não verificado — não entra em perfis automáticos

## Remover Microsoft.549981C3F5F10

`appx.microsoft.549981c3f5f10` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Desativar Cortana

`reg.cortana` · `registry`

- **O que muda:** Desativa a Cortana por política nas edições compatíveis. Desfazer restaura tipo e bytes originais ou remove o valor criado
- **Risco:** Baixo
- **O que pode deixar de funcionar:** A assistente Cortana fica indisponível. Não altera a busca de arquivos
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-experience#allowcortana)

## Remover Microsoft.GamingApp

`appx.microsoft.gamingapp` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Remover Microsoft.XboxApp

`appx.microsoft.xboxapp` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remover Microsoft.XboxGamingOverlay

`appx.microsoft.xboxgamingoverlay` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Remover Microsoft.XboxGameOverlay

`appx.microsoft.xboxgameoverlay` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remover Microsoft.XboxSpeechToTextOverlay

`appx.microsoft.xboxspeechtotextoverlay` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remover Microsoft.XboxIdentityProvider

`appx.microsoft.xboxidentityprovider` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Alto — seleção manual e confirmação reforçada
- **O que pode deixar de funcionar:** Pode impedir Minecraft Bedrock, Game Pass e jogos da Microsoft Store de abrir ou autenticar
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Agressivo
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remover Microsoft.GamingServices

`appx.microsoft.gamingservices` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Alto — seleção manual e confirmação reforçada
- **O que pode deixar de funcionar:** Pode impedir Minecraft Bedrock, Game Pass e jogos da Microsoft Store de abrir ou autenticar
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Agressivo
- **Verificação:** Não verificado — não entra em perfis automáticos
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: Não verificado — não entra em perfis automáticos

## Desativar Autenticação Xbox

`svc.xblauthmanager` · `service`

- **O que muda:** Interrompe a autenticação Xbox Live. Desfazer restaura o modo de início e o estado de execução
- **Risco:** Alto — seleção manual e confirmação reforçada
- **O que pode deixar de funcionar:** Pode impedir Minecraft Bedrock, Game Pass e jogos da Microsoft Store de abrir ou autenticar
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Agressivo
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Desativar Jogos salvos do Xbox

`svc.xblgamesave` · `service`

- **O que muda:** Interrompe a sincronização de jogos salvos. Desfazer restaura o modo de início e o estado de execução
- **Risco:** Alto — seleção manual e confirmação reforçada
- **O que pode deixar de funcionar:** Pode impedir Minecraft Bedrock, Game Pass e jogos da Microsoft Store de abrir ou autenticar
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Agressivo
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Desativar Rede Xbox Live

`svc.xboxnetapisvc` · `service`

- **O que muda:** Oferece conectividade para jogos Xbox Live. Desfazer restaura o modo de início e o estado de execução
- **Risco:** Alto — seleção manual e confirmação reforçada
- **O que pode deixar de funcionar:** Pode impedir Minecraft Bedrock, Game Pass e jogos da Microsoft Store de abrir ou autenticar — Bloqueado pela proteção de serviços de rede
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Agressivo
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Desativar Acessórios Xbox

`svc.xboxgipsvc` · `service`

- **O que muda:** Interrompe a comunicação com acessórios Xbox. Desfazer restaura o modo de início e o estado de execução
- **Risco:** Alto — seleção manual e confirmação reforçada
- **O que pode deixar de funcionar:** Pode impedir o controle Xbox sem fio ou seu adaptador de funcionar
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Agressivo
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Remover Microsoft.BingNews

`appx.microsoft.bingnews` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Remover Microsoft.BingWeather

`appx.microsoft.bingweather` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remover Microsoft.GetHelp

`appx.microsoft.gethelp` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remover Microsoft.Getstarted

`appx.microsoft.getstarted` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remover Microsoft.WindowsFeedbackHub

`appx.microsoft.windowsfeedbackhub` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remover Microsoft.MicrosoftOfficeHub

`appx.microsoft.microsoftofficehub` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remover Microsoft.People

`appx.microsoft.people` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remover Microsoft.MixedReality.Portal

`appx.microsoft.mixedreality.portal` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Remover Microsoft.Microsoft3DViewer

`appx.microsoft.microsoft3dviewer` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remover Microsoft.Print3D

`appx.microsoft.print3d` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remover Microsoft.SkypeApp

`appx.microsoft.skypeapp` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remover MicrosoftTeams

`appx.microsoftteams` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Remover MSTeams

`appx.msteams` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Remover Clipchamp.Clipchamp

`appx.clipchamp.clipchamp` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Remover Microsoft.Todos

`appx.microsoft.todos` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Remover Microsoft.PowerAutomateDesktop

`appx.microsoft.powerautomatedesktop` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Remover Microsoft.YourPhone

`appx.microsoft.yourphone` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Remover Microsoft.WindowsMaps

`appx.microsoft.windowsmaps` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remover MicrosoftCorporationII.MicrosoftFamily

`appx.microsoftcorporationii.microsoftfamily` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Equilibrado
- **Verificação:** Não verificado — não entra em perfis automáticos
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: Não verificado — não entra em perfis automáticos

## Remover Microsoft.Windows.DevHome

`appx.microsoft.windows.devhome` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Remover Microsoft.ZuneVideo

`appx.microsoft.zunevideo` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remover microsoft.windowscommunicationsapps

`appx.microsoft.windowscommunicationsapps` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remover Microsoft.MicrosoftSolitaireCollection

`appx.microsoft.microsoftsolitairecollection` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remover Microsoft.ZuneMusic

`appx.microsoft.zunemusic` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Agressivo
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/application-management/remove-provisioned-apps-during-update)

## Remover Microsoft.OutlookForWindows

`appx.microsoft.outlookforwindows` · `appx`

- **O que muda:** Remova o app instalado e seu provisionamento. Reinstalação manual pode exigir internet e não recupera dados nem a versão original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O app fica indisponível. Reinstalação manual não recupera dados nem garante a versão original
- **Como voltar:** Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store
- **Nível:** Agressivo
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/AppxPackages.json)

## Desativar Diagnóstico conectado

`svc.diagtrack` · `service`

- **O que muda:** Reduz a coleta de diagnóstico em segundo plano. Desfazer restaura o modo de início e o estado de execução
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Pode reduzir dados disponíveis para diagnosticar falhas
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Desativar Mensagens de gerenciamento

`svc.dmwappushservice` · `service`

- **O que muda:** Interrompe o roteamento de mensagens de gerenciamento. Desfazer restaura o modo de início e o estado de execução
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Pode afetar dispositivos gerenciados pela organização
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Desativar Coletor de diagnóstico

`svc.diagnosticshub.standardcollector.service` · `service`

- **O que muda:** Interrompe a coleta de diagnóstico do desenvolvedor. Desfazer restaura o modo de início e o estado de execução
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Ferramentas de diagnóstico podem deixar de coletar dados
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Desativar Demonstração de loja

`svc.retaildemo` · `service`

- **O que muda:** Desativa a demonstração para lojas. Desfazer restaura o modo de início e o estado de execução
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O modo de demonstração deixa de funcionar
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Desativar Compartilhamento de mídia

`svc.wmpnetworksvc` · `service`

- **O que muda:** Compartilha mídia pela rede. Desfazer restaura o modo de início e o estado de execução
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Bloqueado pela proteção de serviços de rede
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Desativar Registro remoto

`svc.remoteregistry` · `service`

- **O que muda:** Desativa o acesso remoto ao registro. Desfazer restaura o modo de início e o estado de execução
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Pode afetar o suporte remoto da organização
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Desativar Fax

`svc.fax` · `service`

- **O que muda:** Desativa o envio e recebimento de fax. Desfazer restaura o modo de início e o estado de execução
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Fax deixa de funcionar
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Desativar Relatórios de erros

`svc.wersvc` · `service`

- **O que muda:** Interrompe relatórios de erros em segundo plano. Desfazer restaura o modo de início e o estado de execução
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Pode reduzir informações para investigar travamentos
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Desativar Mapas baixados

`svc.mapsbroker` · `service`

- **O que muda:** Interrompe o gerenciamento de mapas offline. Desfazer restaura o modo de início e o estado de execução
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Mapas offline podem deixar de atualizar
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Desativar Programa Windows Insider

`svc.wisvc` · `service`

- **O que muda:** Desativa o serviço de avaliação do Windows. Desfazer restaura o modo de início e o estado de execução
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Pode afetar a participação no Windows Insider
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Desativar Integração de telefone

`svc.phonesvc` · `service`

- **O que muda:** Desativa a integração de telefonia. Desfazer restaura o modo de início e o estado de execução
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Recursos integrados de telefone podem parar
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Desativar Pagamentos e NFC

`svc.semgrsvc` · `service`

- **O que muda:** Desativa o gerenciamento de pagamentos e NFC. Desfazer restaura o modo de início e o estado de execução
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Pagamentos e NFC podem deixar de funcionar
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Desativar Hotspot móvel

`svc.icssvc` · `service`

- **O que muda:** Compartilha internet com outros dispositivos. Desfazer restaura o modo de início e o estado de execução
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Bloqueado pela proteção de serviços de rede
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Usar início manual em Otimização de Entrega

`svc.dosvc` · `service`

- **O que muda:** Muda o início da Otimização de Entrega para manual. Desfazer restaura o modo de início e o estado de execução
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Pode afetar a distribuição de downloads entre PCs. Não desativa o Windows Update
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Desativar Localização

`svc.lfsvc` · `service`

- **O que muda:** Desativa a localização do dispositivo. Desfazer restaura o modo de início e o estado de execução
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Apps que usam localização podem falhar
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Agressivo
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Desativar Indexação de arquivos

`svc.wsearch` · `service`

- **O que muda:** Interrompe a indexação de arquivos. Desfazer restaura o modo de início e o estado de execução
- **Risco:** Baixo
- **O que pode deixar de funcionar:** A busca de arquivos pode ficar lenta ou incompleta
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Agressivo
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Desativar Pré-carregamento de apps

`svc.sysmain` · `service`

- **O que muda:** Interrompe o pré-carregamento de apps. Desfazer restaura o modo de início e o estado de execução
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Só pré-selecione em SSD ou NVMe. O carregamento pode piorar
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/iot/iot-enterprise/optimize/services)

## Reduzir dados de diagnóstico

`reg.telemetry` · `registry`

- **O que muda:** Solicita o menor nível configurado de diagnóstico. O resultado depende da edição. Desfazer restaura tipo e bytes originais ou remove o valor criado
- **Risco:** Baixo
- **O que pode deixar de funcionar:** A edição pode impor um nível mínimo diferente. Não garante ausência de tráfego
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-system#allowtelemetry)

## Desabilitar Microsoft Compatibility Appraiser

`task.microsoft_compatibility_appraiser` · `scheduled_task`

- **O que muda:** Desabilite esta tarefa agendada. Desfazer restaura seu estado de habilitação
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Relatórios de diagnóstico ou atualizações de mapas podem deixar de ocorrer
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Não verificado — não entra em perfis automáticos
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration)

## Desabilitar ProgramDataUpdater

`task.programdataupdater` · `scheduled_task`

- **O que muda:** Desabilite esta tarefa agendada. Desfazer restaura seu estado de habilitação
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Relatórios de diagnóstico ou atualizações de mapas podem deixar de ocorrer
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Não verificado — não entra em perfis automáticos
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration)

## Desabilitar Consolidator

`task.consolidator` · `scheduled_task`

- **O que muda:** Desabilite esta tarefa agendada. Desfazer restaura seu estado de habilitação
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Relatórios de diagnóstico ou atualizações de mapas podem deixar de ocorrer
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Não verificado — não entra em perfis automáticos
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration)

## Desabilitar UsbCeip

`task.usbceip` · `scheduled_task`

- **O que muda:** Desabilite esta tarefa agendada. Desfazer restaura seu estado de habilitação
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Relatórios de diagnóstico ou atualizações de mapas podem deixar de ocorrer
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Não verificado — não entra em perfis automáticos
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration)

## Desabilitar DmClient

`task.dmclient` · `scheduled_task`

- **O que muda:** Desabilite esta tarefa agendada. Desfazer restaura seu estado de habilitação
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Relatórios de diagnóstico ou atualizações de mapas podem deixar de ocorrer
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Não verificado — não entra em perfis automáticos
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration)

## Desabilitar QueueReporting

`task.queuereporting` · `scheduled_task`

- **O que muda:** Desabilite esta tarefa agendada. Desfazer restaura seu estado de habilitação
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Relatórios de diagnóstico ou atualizações de mapas podem deixar de ocorrer
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Não verificado — não entra em perfis automáticos
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration)

## Desabilitar MapsToastTask

`task.mapstoasttask` · `scheduled_task`

- **O que muda:** Desabilite esta tarefa agendada. Desfazer restaura seu estado de habilitação
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Relatórios de diagnóstico ou atualizações de mapas podem deixar de ocorrer
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Não verificado — não entra em perfis automáticos
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration)

## Desabilitar MapsUpdateTask

`task.mapsupdatetask` · `scheduled_task`

- **O que muda:** Desabilite esta tarefa agendada. Desfazer restaura seu estado de habilitação
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Relatórios de diagnóstico ou atualizações de mapas podem deixar de ocorrer
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Não verificado — não entra em perfis automáticos
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration)

## Desativar sugestões de apps

`reg.consumer_features` · `registry`

- **O que muda:** Desativa experiências promocionais do Windows onde a política é aceita. Desfazer restaura tipo e bytes originais ou remove o valor criado
- **Risco:** Baixo
- **O que pode deixar de funcionar:** A política pode ser ignorada em algumas edições
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-experience#allowwindowsconsumerfeatures)

## Desativar identificação para anúncios

`reg.advertising` · `registry`

- **O que muda:** Desativa o identificador usado para anúncios personalizados. Desfazer restaura tipo e bytes originais ou remove o valor criado
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Anúncios podem deixar de ser personalizados
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-privacy#disableadvertisingid)

## Desativar Widgets

`reg.widgets` · `registry`

- **O que muda:** Desativa o painel Widgets onde a política é aceita. Desfazer restaura tipo e bytes originais ou remove o valor criado
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Notícias e clima no painel ficam indisponíveis
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** pro, enterprise, education
- **Build mínimo:** 22000
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-newsandinterests#allownewsandinterests)

## Desativar sugestão SystemPaneSuggestionsEnabled

`reg.systempanesuggestionsenabled` · `registry`

- **O que muda:** Desativa sugestões ou instalações promocionais automáticas. Desfazer restaura o valor original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Sugestões do Windows podem deixar de aparecer
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows-server/remote/remote-desktop-services/remote-desktop-services-vdi-optimize-configuration), [2](https://github.com/The-Virtual-Desktop-Team/Virtual-Desktop-Optimization-Tool/blob/c93dfb2db35a16584efebb27a626a2f51c503c54/2009/ConfigurationFiles/DefaultUserSettings.json)

## Desativar sugestão SilentInstalledAppsEnabled

`reg.silentinstalledappsenabled` · `registry`

- **O que muda:** Desativa sugestões ou instalações promocionais automáticas. Desfazer restaura o valor original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Sugestões do Windows podem deixar de aparecer
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Não verificado — não entra em perfis automáticos
- **Edições declaradas:** pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: Não verificado — não entra em perfis automáticos

## Desativar sugestão SubscribedContent-338389Enabled

`reg.subscribedcontent-338389enabled` · `registry`

- **O que muda:** Desativa sugestões ou instalações promocionais automáticas. Desfazer restaura o valor original
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Sugestões do Windows podem deixar de aparecer
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Não verificado — não entra em perfis automáticos
- **Edições declaradas:** pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: Não verificado — não entra em perfis automáticos

## Desativar recomendações no Iniciar

`reg.iris` · `registry`

- **O que muda:** Solicita a desativação de recomendações no menu Iniciar. Desfazer restaura tipo e bytes originais ou remove o valor criado
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Recomendações podem sumir do menu Iniciar
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 22000
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/apps/develop/settings/settings-windows-11)

## Desativar gravação de jogos em segundo plano

`reg.game_dvr_policy` · `registry`

- **O que muda:** Desativa a gravação e transmissão de jogos por política no Windows 10. Desfazer restaura tipo e bytes originais ou remove o valor criado
- **Risco:** Baixo
- **O que pode deixar de funcionar:** A gravação de jogos do Windows fica indisponível
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-applicationmanagement#allowgamedvr)

## Desativar Game DVR do usuário

`reg.game_dvr_user` · `registry`

- **O que muda:** Desativa Game DVR para o usuário interativo. Desfazer restaura tipo e bytes originais ou remove o valor criado
- **Risco:** Baixo
- **O que pode deixar de funcionar:** A gravação de jogos em segundo plano pode parar
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Não verificado — não entra em perfis automáticos
- **Edições declaradas:** pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: Não verificado — não entra em perfis automáticos

## Ativar Modo de Jogo

`reg.game_mode` · `registry`

- **O que muda:** Ativa a preferência Modo de Jogo do Windows. Desfazer restaura tipo e bytes originais ou remove o valor criado
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O resultado varia por jogo e deve ser medido
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Seguro
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 22000
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/apps/develop/settings/settings-windows-11)

## Desativar pré-carregamento do Edge

`reg.edge_preload` · `registry`

- **O que muda:** Desativa o pré-carregamento do Edge. Desfazer restaura tipo e bytes originais ou remove o valor criado
- **Risco:** Baixo
- **O que pode deixar de funcionar:** O primeiro início do Edge pode demorar mais
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/deployedge/microsoft-edge-policies/startupboostenabled)

## Desativar Edge em segundo plano

`reg.edge_background` · `registry`

- **O que muda:** Desativa a execução do Edge em segundo plano. Desfazer restaura tipo e bytes originais ou remove o valor criado
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Notificações e extensões em segundo plano podem parar
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/deployedge/microsoft-edge-policies/backgroundmodeenabled)

## Ativar agendamento de GPU por hardware

`reg.hags` · `registry`

- **O que muda:** Solicita agendamento de GPU acelerado por hardware. Desfazer restaura tipo e bytes originais ou remove o valor criado
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Exige GPU e driver compatíveis e reinício. O desempenho pode piorar
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Equilibrado
- **Verificação:** Não verificado — não entra em perfis automáticos
- **Edições declaradas:** pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Sim

Fontes: Não verificado — não entra em perfis automáticos

## Ativar Alto Desempenho

`power.high_performance` · `power`

- **O que muda:** Ative Alto Desempenho. Pode aumentar calor e consumo. Desfazer restaura o plano anterior e remove apenas planos criados pelo app
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Pode alterar a disponibilidade deste recurso
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Equilibrado
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/win32/power/power-setting-guids), [2](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/powercfg-command-line-options)

## Bloquear apps em segundo plano

`reg.background_apps` · `registry`

- **O que muda:** Bloqueia a execução em segundo plano de apps da Store. Desfazer restaura tipo e bytes originais ou remove o valor criado
- **Risco:** Alto — seleção manual e confirmação reforçada
- **O que pode deixar de funcionar:** Apps da Store como WhatsApp podem deixar de notificar quando estiverem fechados
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Agressivo
- **Verificação:** Fonte oficial registrada; a compatibilidade ainda depende da versão/edição
- **Edições declaradas:** pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: [1](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-privacy#letappsruninbackground)

## Revisar inicialização HKCU

`startup.hkcu` · `startup`

- **O que muda:** Descubra entradas de inicialização. Escolha cada entrada manualmente. Entradas essenciais ficam ocultas
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Pode alterar a disponibilidade deste recurso
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Somente manual
- **Verificação:** Não verificado — não entra em perfis automáticos
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: Não verificado — não entra em perfis automáticos

## Revisar inicialização HKLM

`startup.hklm` · `startup`

- **O que muda:** Descubra entradas de inicialização. Escolha cada entrada manualmente. Entradas essenciais ficam ocultas
- **Risco:** Baixo
- **O que pode deixar de funcionar:** Pode alterar a disponibilidade deste recurso
- **Como voltar:** Use Desfazer para restaurar o estado original registrado antes da aplicação
- **Nível:** Somente manual
- **Verificação:** Não verificado — não entra em perfis automáticos
- **Edições declaradas:** home, pro, enterprise, education
- **Build mínimo:** 19045
- **Reinício:** Não

Fontes: Não verificado — não entra em perfis automáticos
