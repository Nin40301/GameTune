# Ametista Desktop — Fases 4 a 6

Interface Windows em Tauri 2, React, TypeScript strict e Tailwind. Usa o mesmo
catálogo, presets, proteção, WindowsSession e journal da CLI. Disponível em
pt-BR/en, escuro/claro, com navegação por teclado, diálogo de detalhes com foco
contido/restaurado, regiões de status e testes axe AA. Não exibe métricas fictícias.

Na Fase 5, Início inclui medições reais e Benchmark conduz as rodadas Antes/
Depois. Leia [protocolo e limites](benchmark.md), [aceite de VM](benchmark-vm.md)
e [resultados e consumo](phase-5.md). PresentMon é importação offline de CSV,
sem execução da ferramenta pelo app. Na Fase 6, [Jogos](games.md) cadastra perfis
temporários e Configurações inicia o monitor sem WebView, que sobrevive à janela.

## Compilar

Requisitos de desenvolvimento: Rust estável 1.98+, Node 24.13+ ou 26, npm,
Windows SDK e Visual Studio Build Tools C++. Runtime: Windows suportado, WebView2
Evergreen instalado, PowerShell 5.1/.NET do Windows. Sem download de runtime pelo app.

Na raiz:

```powershell
cd apps/desktop
npm ci --no-audit --no-fund
npm run build
cd ../..
cargo build -p ametista-desktop -p ametista-helper -p ametista-monitor --release --locked --offline
./tools/verify-desktop.ps1
```

Os arquivos `target/release/ametista.exe`, `ametista-helper.exe` e `ametista-monitor.exe` devem ficar
juntos. UI/monitor têm manifesto **asInvoker**, o helper **requireAdministrator**.
O build padrão inclui assets locais, não depende do servidor Vite e exclui mocks
e autorização de fixtures. Não use `--all-features` para um release de produção.
Os executáveis ainda não são assinados; instalador e assinatura entram na Fase 7.

Abra ametista.exe pelo Explorer/terminal comum. Se iniciado elevado, o backend
recusa a análise e orienta abrir como usuário comum. Não abra o helper diretamente.
Ele não aceita comandos de shell, caminhos de alvo ou scripts em argumentos.

Para desenvolver sem alterações de configuração:

```powershell
cd apps/desktop
npm run tauri -- dev --no-default-features
```

Para navegar com backend fictício (build exclusivamente de testes):

```powershell
npm run preview:test
```

Esse modo tem faixa explícita de demonstração, inventário fictício e nenhuma
ponte para o helper. Não é um modo selecionável no produto instalado.

## Usar

1. Em **Início**, escolha Seguro/Equilibrado/Agressivo ou Personalizado e clique
   **Simule**. Sem análise completa não é possível revisar aplicação/remoção.
2. Em **Itens**, busque, filtre e consulte o painel de detalhes. Presets são
   calculados em Rust, inclusive suporte por edição/build e exclusão de risco
   alto. Modo avançado mostra não verificados; continua exigindo consentimento.
3. Em **Revisão**, confira resumo por tipo, efeitos, reinício e consentimentos.
   Aplicar consome uma autorização de uso único e solicita UAC somente então.
4. Em **Limpeza de apps**, selecione vários apps instalados/removíveis do
   catálogo. Cada app exige confirmação própria de ausência de undo; risco alto
   e não verificados exigem confirmações adicionais. A remoção afeta todos os
   usuários e o provisionamento, conforme inventário recapturado elevado.
5. Em **Histórico**, desfaça itens ou tudo após revisão. Reinstalar abre a página
   validada da Store após ação/revisão explícita; **Desfazer não reinstala apps**
   nem recupera dados. O app não instala automaticamente pela Store.
6. Em **Configurações**, escolha idioma, tema e modo avançado. Preferências são
   locais, nunca concedem consentimento ou mudam o catálogo.

A análise roda fora da thread da janela, informa progresso por item e é
cancelável. Usa até quatro workers e um inventário Appx por análise, em memória.
Cancelar interrompe apenas os filhos PowerShell de leitura criados pelo app,
descarta resultados e impede revisão; não busca nem encerra processos de terceiros.
Não há cancelamento de escrita que prometa rollback instantâneo: se fechar no
meio de uma transação, consulte o journal e desfaça na próxima execução.

Resultados/revisões expiram em 120 segundos. Alterar a seleção invalida a revisão
na interface; iniciar/cancelar análise invalida evidência/ticket no backend.
O helper recaptura o estado e revalida domínio/consentimentos independentemente
de caches e tickets da janela. Não repete operação automaticamente após timeout.

## Histórico protegido e privilégio mínimo

O WAL continua acessível apenas a Administradores/SYSTEM. A análise não depende
dele. Se a janela não conseguir ler o histórico, **Carregue o histórico protegido**
prepara uma revisão de leitura explícita e usa o mesmo helper/UAC para retornar
uma projeção. Isso não cria ProgramData ou altera configurações. Após uma escrita,
o helper também retorna o histórico para evitar outra elevação imediata.

Esta é uma exceção explícita de UX à meta de pedir UAC apenas para escritas:
sem enfraquecer a ACL do journal, sua leitura por usuário comum exige autorização.
Não foi introduzida uma cópia autoritativa editável no perfil do usuário.

## Verificar

```powershell
cd apps/desktop
npm run format:check
npm run lint
npm run build
npm test
npx playwright install chromium
npm run test:e2e
cd ../..
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
cargo test --workspace --locked --offline
```

Playwright sempre usa backend mockado. Testes IPC normais usam pipes locais,
validam ACL/SID/PID/replay/tamanho/JSON e não lançam helper nem UAC. Os dois testes
legados que escrevem em HKCU agora são ignorados e exigem opt-in de VM; veja o
[harness de fixtures](windows-vm.md). Validação de UAC/auxiliar com escrita e
outra conta deve seguir [o roteiro desktop em VM](desktop-vm.md).

Sem rede na UI: CSP permite somente assets locais e transporte IPC virtual do
Tauri. Não há plugin shell, fs, http ou updater. Fontes/documentação do catálogo
aparecem como texto, sem navegação automática. A Store pode usar rede ao ser
aberta por solicitação explícita. Desenvolvimento/instalação de dependências usa
rede de build; o produto em execução não depende dela.
