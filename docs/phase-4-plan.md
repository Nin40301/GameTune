# Fase 4 — plano de arquitetura

Adicionar apps/desktop (React, TypeScript strict, Tailwind e Tauri 2), um crate
desktop_backend e um crate ipc com binário gametune-helper. A UI permanece
asInvoker; o helper tem manifesto requireAdministrator, é lançado apenas por
ação explícita após revisão e encerra após uma transação. Ambos usam o catálogo,
perfis, proteção, WindowsSession, consentimentos e journal existentes. A UI não
implementa seleção de presets nem regras de autorização de execução.

IPC local por named pipe exclusivo de nome/nonce aleatórios, limite de tamanho,
versão, mensagens tipadas com campos fechados e transação de uso único. O helper
deriva SID/sessão interativos, restringe a DACL ao SID, recusa clientes remotos e
valida identidade real do cliente e PID do processo UI; a UI autentica o PID do
helper que lançou. Rejeitar replay, desconhecidos e consentimentos insuficientes
antes de qualquer escrita. FFI Windows fica em módulo pequeno comentado, com
unsafe proibido no restante. Não há comandos, caminhos ou scripts livres no IPC.

Análise em worker, com progresso por item, cancelamento cooperativo, consultas
somente leitura em até quatro workers e um inventário Appx por análise. Cache
somente de leitura durante a análise; cancelar descarta resultados, impede revisão
e deixa de agendar consultas. Aplicar sempre recaptura pelo backend elevado.
Resultados têm prazo e revisão vinculada à seleção; alterar a seleção invalida
revisão. Cancelamento não promete interromper uma transação de escrita.

Telas: Início, Itens, Revisão, Limpeza de apps, Histórico e Configurações. Estética
de painel técnico sóbrio, grafite com acento lima e listas densas legíveis; claro
equivalente. Textos locais pt-BR/en, sem fontes/assets remotos. Consentimentos por
app, aviso sem undo e confirmação reforçada próximos de cada item. Sem métricas
fictícias. Histórico autoritativo continua protegido; leitura negada oferece
autorização explícita de leitura pelo mesmo helper, sem enfraquecer ACL do WAL.

Tauri expõe apenas comandos tipados necessários, capability da janela principal,
CSP sem rede, sem plugins shell/fs/http e sem navegação externa. Preferências
locais de idioma/tema não autorizam ações. Mocks de navegador são restritos ao
build de testes e nunca carregados no binário de produção.

Validar Rust/IPC (incluindo pipes Windows reais sem ajustes), componentes,
Playwright com backend mock, axe e inspeção visual em claro/escuro e telas
estreitas. Compilar manifests e ambos os binários. Nenhuma aplicação, remoção,
elevação ou reinstalação real será executada no host; integração mutável e UAC
ficam em harness opt-in de VM. Documentar dependências, limites, contagens,
comandos e pendências; fazer commit e entregar fontes/binários em outputs.
