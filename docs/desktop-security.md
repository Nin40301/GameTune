# Limite de privilégios da interface

## Superfície do WebView

Somente a janela local `main` recebe oito permissões: bootstrap, iniciar/consultar/
cancelar análise, preparar revisão, executar revisão e os comandos tipados
`lab_action`/`lab_file`. Estes últimos controlam somente coleta/estado e seletores
nativos de importação/exportação; não aceitam caminhos fornecidos pela UI,
comandos livres ou operações elevadas. Payloads desconhecidos são rejeitados.
AppManifest enumera esses
comandos para o controle de acesso do Tauri 2; não se depende da permissão implícita
de comandos customizados. Não há plugins de execução, arquivos ou rede.

CSP: default-src none; scripts/estilos/fontes locais; connect-src apenas `ipc:` e
`http://ipc.localhost` (transporte virtual do Tauri no Windows); sem eval, inline,
frames ou formulários. Navegação externa e novas janelas são negadas. Somente no
build de desenvolvimento é permitida navegação para 127.0.0.1:1420. Mocks ficam
em importação eliminada por constante de build; o verificador recusa esses símbolos
nos assets de produção. Preferências locais não autorizam operações.

O sampler usa um snapshot global NT, sem handles individuais de jogos. Ajustes
ficam bloqueados enquanto uma rodada está ativa; uma reserva impede a corrida
inversa de iniciar coleta durante a execução do auxiliar. Os dados do benchmark
no AppData não concedem autorização ao motor nem substituem consentimentos.

## Named pipe de uso único

1. Após revisão e aprovação, o backend consome um ticket aleatório de 256 bits,
   vinculado à operação/IDs e com validade de 120 s. O renderer não fornece código,
   parâmetros de processo, caminho do helper ou nome livre de pipe.
2. O backend gera outro nonce de 256 bits e inicia o helper irmão por
   ShellExecuteExW/runas, com somente nonce hexadecimal e PID numérico próprios.
   Guarda o handle/PID do helper que ele próprio lançou; não enumera jogos.
3. O helper deriva o SID interativo pelo WindowsSession. Cria pipe local aleatório,
   primeira instância exclusiva, máximo uma instância, rejeitando clientes remotos.
   DACL protegida permite só o SID interativo. A ACE usa direitos específicos que
   excluem FILE_CREATE_PIPE_INSTANCE; label Medium permite comunicação da UI comum.
4. O cliente autentica o PID do servidor contra o helper lançado. O servidor consulta
   PID real e SID do cliente via APIs de pipe/token, não aceita identidade declarada
   pelo JSON. Impersonação é síncrona, só para leitura do token e nunca atravessa
   await; falha ao reverter encerra o processo. O cliente usa identificação, sem
   delegar ao servidor o uso de suas credenciais.
5. Uma única mensagem com versão, hash do catálogo, nonce, sequência 1 e Command
   fechado é aceita. Catálogos diferentes entre UI/helper são rejeitados.
   Campos desconhecidos, JSON inválido, ID desconhecido/duplicado, confirmação
   ausente e segundo uso são rejeitados antes de executar. Tentativa inválida
   também consome a sessão. Não há reconexão/retry que repita escrita incerta.
6. O helper usa catálogo, proteção, Machine e validação compartilhada do Engine;
   recaptura estados pelo WindowsSession. RemoveApps exige consentimento individual,
   ausência de undo, risco de jogos e não verificação quando aplicáveis. Nada vindo
   do renderer determina uma chave, serviço, script ou URI arbitrária.
7. Resposta é limitada; helper encerra após responder/ack. A conexão inicial tem
   prazo de 30 s, entrada de 15 s, pedido até 64 KiB, resposta até 4 MiB, e a janela
   aguarda até 20 min por transação sem repeti-la. Se a janela fechar ou perder a
   resposta, o helper pode terminar a transação; originais continuam no WAL.

FFI Rust se concentra em `ipc/src/native.rs`, com deny unsafe no restante do crate
e forbid unsafe nos demais. As únicas aberturas de token/handle são do próprio
processo, thread de impersonação e helper recém-lançado. Não há OpenProcess em PID
de terceiros, leitura de memória, injeção, drivers ou handles em jogos.

Comandos: Apply, Undo, UndoAll, RemoveApps, History e Reinstall. History só lê;
Reinstall resolve o PFN no histórico/inventário validado e usa o broker de desktop
existente. Ambos podem pedir UAC para ler histórico protegido. Não há serviço
elevado persistente. Artefatos devem manter UI/helper juntos e com mesma versão.

## Limites do modelo de ameaça

SID/PID/nonce e DACL protegem contra conexão de outro usuário, sequestro do nome,
mensagens fora do protocolo e replay. Não são autenticação de código contra malware
com controle integral da mesma conta, um administrador malicioso ou substituição
dos binários portáteis. Consentimentos são dados tipados revalidados; não provam que
uma pessoa leu um texto se o próprio cliente estiver comprometido. Assinatura,
instalador protegido e revisão independente de segurança permanecem na Fase 7.

O cache é só de leitura e dura uma análise. Nenhum snapshot do WebView é original
autoritativo. Arquivos/WAL e escopo máquina/SID são os da implementação anterior.
ACL do journal não foi aberta ao usuário comum para facilitar a UI.

Fontes primárias:
[Tauri capabilities](https://v2.tauri.app/security/capabilities/),
[runtime authority](https://v2.tauri.app/security/runtime-authority/),
[named pipe security](https://learn.microsoft.com/en-us/windows/win32/ipc/named-pipe-security-and-access-rights),
[impersonação de cliente](https://learn.microsoft.com/en-us/windows/win32/ipc/impersonating-a-named-pipe-client),
[Tokio ServerOptions](https://docs.rs/tokio/1.53.3/tokio/net/windows/named_pipe/struct.ServerOptions.html).
