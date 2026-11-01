# Dependências da interface e coleta

Não há biblioteca de componentes, ícones remotos, fontes web, cliente HTTP,
analytics, shell plugin ou framework de estado adicional.

| Dependência direta | Justificativa |
| --- | --- |
| Tauri 2 / tauri-build | Janela WebView2, assets embutidos, bridge tipada, CSP e permissões por janela |
| React / react-dom | Componentes e estado da interface com DOM semântico |
| TypeScript strict / tipos React, DOM e Node | Verificação de contratos e ferramentas; tipos não entram no runtime |
| Tailwind / plugin Vite | Preflight, tokens locais e pipeline CSS; layout usa classes semânticas e variáveis de tema |
| Vite / plugin React | Compilar assets locais, eliminar backend de testes e servidor de desenvolvimento |
| @tauri-apps/api | Somente invoke dos oito comandos tipados; sem APIs shell/fs/http |
| @tauri-apps/cli | Desenvolvimento/build; não é incluído no aplicativo |
| Tokio | Named pipes overlapped seguros, limites e timeouts; não são usados sockets de rede |
| windows-sys | FFI mínima para DACL, SID/PID de pipe, token próprio/thread, helper e snapshots globais de métricas, sem abrir jogos |
| getrandom | Nonces de 256 bits fornecidos pelo gerador do sistema operacional |
| embed-resource | Manifesto Windows do binário helper independente do Tauri |
| Vitest / jsdom / Testing Library / jest-dom / user-event | Componentes e fluxos de consentimento, usando backend mock |
| Playwright / axe-core | Chromium, teclado/foco, revisão, consentimentos e auditoria AA automatizada |
| ESLint / typescript-eslint / hooks / globals | Lint TypeScript e regras de hooks |
| Prettier | Formatação reprodutível da interface e testes |
| csv 1.4 | Parsing de CSV PresentMon com aspas/escapes e validação própria de limites, identidade e métricas; não executa a ferramenta |
| rfd 0.17 (somente Windows, sem features padrão) | Seletor nativo de importação/exportação; sem conceder fs/shell geral à UI |

`tempfile`, serde/serde_json e SHA-256 também são reutilizados para persistência
atômica, checksum e lock do experimento. A coleta deliberadamente não usa
`sysinfo` para enumerar recursos de jogos: o backend usa snapshot global para
evitar handles individuais. PresentMon não é dependência, não é empacotado e
não é baixado. Apenas seu contrato CSV externo é reconhecido.

`serde_json/float_roundtrip` preserva os bits de f64 coletados através de
salvar/reabrir; sem isso, arredondamentos de parsing poderiam invalidar o
checksum de um protocolo legítimo. Há teste de roundtrip com razões não exatas.

serde/serde_json, SHA-256, catálogo, core e WindowsSession já existiam e são
reutilizados. package-lock.json e Cargo.lock fixam resoluções. CI usa Node 26 e
Actions por SHA. Compilar/instalar dependências pode usar rede; executar a UI
de produção usa somente assets locais e IPC. Os testes de navegador rodam apenas
contra 127.0.0.1 e mocks, sem aplicar configurações reais.
