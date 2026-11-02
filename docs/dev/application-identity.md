# Identidade estável do aplicativo

Produto: **Ametista**. Binários: `ametista.exe`, `ametista-cli.exe`, `ametista-helper.exe`, `ametista-monitor.exe`. Versão desta preparação: **0.6.5 alfa**. O titular de copyright no NOTICE e a licença Apache-2.0 permanecem intactos.

O identifier passa de `local.ametista.desktop` (identificador provisório efetivamente usado na 6.5) para **`io.github.nin40301.ametista`**. A referência anterior `local.gametune.desktop` pertence ao checkpoint anterior à renomeação. O formato reverso identifica o produto e seu mantenedor no GitHub; não faz conexão de rede nem depende do nome atual do repositório.

O Tauri usa um identificador único em configurações do sistema e no diretório de dados da WebView. Depois do primeiro instalador estável, ele deve permanecer fixo entre versões. [Configuração oficial do Tauri](https://v2.tauri.app/reference/config/#identifier).

Atalhos, agrupamento e notificações do Windows precisam de uma identidade coerente. O instalador da Fase 7 deverá usar esse mesmo identificador como referência para o AppUserModelID dos atalhos e das integrações que implementar; a troca não é uma implementação de notificações. Um AppUserModelID explícito deve ser consistente nos pontos de integração, e notificações desktop dependem da instalação correta do atalho. [AppUserModelIDs](https://learn.microsoft.com/en-us/windows/win32/shell/appids), [notificações desktop](https://learn.microsoft.com/en-us/windows/win32/shell/enable-desktop-toast-with-appusermodelid).

| Área | Efeito da mudança |
| --- | --- |
| Preferências, WebView/cache, medições e biblioteca de jogos | Novo diretório `%LOCALAPPDATA%\io.github.nin40301.ametista` |
| Monitor separado e UI | Ambos usam o diretório novo; o harness acompanha a mudança |
| Journal e armazenamento protegido | `%ProgramData%\Ametista` permanece; não se perde o original por trocar a identidade da UI |
| Catálogo, IDs, IPC, consentimentos e lista de proteção | Sem alteração |
| Dados anteriores | Permanecem nos diretórios antigos; não são apagados, movidos ou importados automaticamente |
| Tag `v0.6.0` | Preservada em `de75a7531faa45600d78818a3391451811f723de` |

Antes de testar uma versão com identidade diferente, encerre o monitor anterior e conclua a recuperação/desfazer que estiver pendente. Não execute duas versões simultâneas. Preserve um backup dos dados de teste. A biblioteca de jogos e as preferências antigas não aparecerem no diretório novo não significa que foram apagadas. Não copie nem edite o journal protegido para contornar verificações de identidade.

O motivo e o efeito em instalador/AppUserModelID estão documentados; o aceite em atalhos, notificações, atualização/desinstalação e UAC continua pendente de VM e da Fase 7. Esta mudança não instala atalhos ou associações no host.
