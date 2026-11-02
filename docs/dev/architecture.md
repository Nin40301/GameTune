# Arquitetura — Ametista 0.6.5

**Produto:** Tauri 2 + React + TypeScript strict + Tailwind no Windows. Assets,
catálogo e pt-BR/en locais; tema escuro/claro; sem rede no WebView. Não há engine
de ajustes em JavaScript. CLI e UI compartilham core/catalog/WindowsSession.

**Domínio:** core contém modelos, presets, proteção por alvo exato, validação de
pedidos/consentimentos, perfis fechados e journal. catalog verifica schema, hash
e compatibilidade. Seis exec_* adaptam Executor; windows_runtime implementa as
operações Windows, identidade interativa, armazenamento protegido e scripts fixos.
exec_mock serve apenas a testes/exemplos. Nenhum pedido externo escolhe comandos,
scripts ou alvos livres; todos referenciam IDs embutidos e validados novamente.

**Leitura:** desktop_backend inicia workers fora da thread da janela. Um scan
tem no máximo quatro leitores e um inventário Appx em memória. Progresso por item,
cancelamento dos próprios filhos PowerShell de leitura e descarte de evidência
cancelada. A análise não cria journal, checkpoint ou ProgramData. A janela é
asInvoker e recusa operação normal quando iniciada elevada. Bootstrap/presets e
revisões são calculados em Rust; preferências/search/tema são apenas de UI.

**Revisão e privilégio:** nove comandos Tauri tipados, limitados à janela local main,
preparam ticket aleatório vinculado a operação/IDs e prazo de 120 s. Após revisão
e consentimentos, o ticket é consumido. ipc lança o helper irmão com manifesto
requireAdministrator via UAC. Pipe local de uso único com DACL por SID interativo,
PID/SID reais, nonce, hash do catálogo, limites de tamanho/prazos e rejeição de
replay. O helper revalida tudo e chama WindowsSession; encerra após responder.
Só módulos FFI isolados de IPC/métricas/WMI/bandeja admitem unsafe comentado. Não há serviço elevado
persistente. Ler o histórico protegido pode exigir UAC explícito, sem abrir o WAL.

**Transação:** WindowsSession recaptura o estado elevado, usa o hive do SID do
desktop e protege ProgramData com DACL Administradores/SYSTEM. Prepared é
sincronizado antes de aplicar e Reverting antes de undo. Original imutável por
alvo ativo; lock, cadeia de hashes e escopo máquina/SID detectam concorrência,
corrupção e troca de usuário. Recuperação privilegia rollback, em ordem inversa.
Registro preserva tipo/bytes/ausência; serviços, tarefas, inicialização e energia
guardam snapshots. Estado externo divergente falha fechado, sem sobrescrever.

**Appx:** remoção manual separada, nunca automática. Seleção múltipla exige
consentimento individual de ausência de undo e risco reforçado. Nome/PFN e
inventário são persistidos antes da ação. Reinstalar abre somente Store URI
validada no desktop interativo, após ação explícita; não recupera dados/versão.

**Medições:** metrics consulta snapshot global NT, RAM/CPU e plano/boot somente
leitura; não abre processos individuais. Um sampler serve painel e benchmark.
benchmark mantém protocolo versionado com lock, checksum e escrita atômica no
AppData do usuário; 5 × 60 s por condição, descarte da primeira, reboot e plano
constante. Medianas por rodada, p5/p95/MAD e faixa conservadora de ruído; sem
promessa de ganho. PresentMon é somente importação CSV externa. Exportadores
offline/anonimizados usam seletores nativos, sem expor caminhos livres à UI.

**Jogos:** watcher lê configurações de launchers e perfis locais com IDs fechados;
detector puro usa caminho exato + PID/CreationDate. Monitor Rust asInvoker sem
WebView, sob demanda, bandeja Win32 e eventos WMI. União de ações temporárias
compartilhadas; serviços apenas parados. Helper UAC de sessão, heartbeat/EOF,
WAL próprio protegido e lease de máquina reutilizam o motor/undo existentes.
Recuperação anterior obrigatória antes de nova sessão; sem autostart instalado.

**Verificação/entrega:** Rust estável, formato/Clippy, TS/ESLint, componentes,
Playwright com mock isolado, axe, testes reais não elevados de pipe e harnesses
mutáveis exclusivamente opt-in de VM. Código-fonte, binários portáteis e evidência
por fase. Gates separados somam árvore da UI (mediana de cinco execuções, regressão das
medianas de RAM/CPU >15% ou teto de RAM de 200 MiB; p95 como alerta) e monitor (picos de 15 MiB/0,5%
CPU), sem flags experimentais da WebView. UAC, reinícios, helper em sessão e
matriz Home/Pro precisam de aceite em VM. Fase 7: instalador/assinatura/acabamento.
