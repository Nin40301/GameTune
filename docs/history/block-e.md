# Bloco E — auxiliar, APIs nativas e suporte

## Resumo

Antes do UAC, ACL/dono/ancestrais e reparse points da instalação são conferidos, o auxiliar fica aberto sem compartilhamento de escrita/exclusão, seu SHA-256 deve corresponder ao build e Authenticode presente é validado offline. Pasta portátil gravável recusa elevação; não há bypass. O build compila o helper primeiro e embute seu hash nos dois clientes (UI e monitor).

Identidade usa WTS/token próprio/hives locais; serviços usam SCM com direitos mínimos. Catálogo, consentimentos, lista protegida e journal continuam com as mesmas regras. Há logs protegidos com eventos tipados, rotação de 256 KiB/14 dias e cinco arquivos. Suporte abre ferramentas fixas e gera ZIP por projeção, prévia, consentimento e token de uso único; não exporta valores originais, SIDs, caminhos ou nomes de usuário. Dados antigos recebem orientação, sem migração/exclusão.

O novo comando Tauri `support_action` é fechado e tipado, sem execução genérica. A capacidade passa de nove para dez comandos; CSP/rede/privilégios não foram ampliados. ZIP usa a biblioteca mantida `zip 9.2.2`, sem compressão; justificativa e dependências PowerShell no [relatório específico](../dev/powershell-dependency.md). [Inventário](../security/unsafe-inventory.md): 91 linhas com blocos/extern unsafe, por arquivo/função/justificativa/hash; nada de unwrap/expect.

## Comandos e resultados

```powershell
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
cargo test --workspace --all-features --locked --offline
cargo audit
npm run format:check --prefix apps/desktop
npm run lint --prefix apps/desktop
npm test --prefix apps/desktop
npm run build --prefix apps/desktop
npm run test:e2e --prefix apps/desktop
pwsh -NoProfile -File tools/build-release.ps1 -SkipFrontend
pwsh -NoProfile -File tools/verify-desktop.ps1
node tools/audit-unsafe.mjs
node tools/check-doc-links.mjs
```

133 testes Rust passaram, cinco administrativos ignorados; 26 componentes e 48 e2e/axe passaram. Auditoria Cargo sem vulnerabilidades classificadas, com avisos informativos preservados. Verificação nativa confirma manifestos, dez permissões, CSP, versões e hash do helper embutido em UI/monitor. Uma análise real **somente leitura** retornou 88 linhas idênticas antes/depois; tempo exploratório 4,61 → 2,19 s, sem controle estatístico ou promessa de ganho.

O CI do Bloco D aprovou frontend Linux, Rust Linux e auditorias; Windows encontrou teste de identidade que não previa Server no runner hospedado. O teste agora aceita a recusa de edição incompatível, sem autorizar operações nesse sistema. O novo push conferirá novamente. Uma edição por script também corrompeu acentos UTF-8; testes detectaram e todos os arquivos afetados foram reparados antes deste commit.

## Riscos e pendências

Somente a VM pode validar UAC real, outra conta, ACL/Authenticode, SCM apply/undo completo, energia, Appx, CLM, rotação protegida e reinícios. Os roteiros foram ampliados; nada disso foi executado no host. A consulta não elevada da Proteção do Sistema não oferece prova de ativação por volume: estado desconhecido é mostrado honestamente, política DisableSR explícita é detectada e a ferramenta nativa permite conferir. Não inferimos proteção ativa de pontos antigos. A criação/verificação do ponto continua obrigatória para aplicar, e o app não ativa proteção.

Não há instalador assinado; portátil em pasta gravável fica limitado a leitura. Validação Authenticode usa cache local e não faz revogação online nem pin de editor. Logs não substituem o WAL e falha de log não pode impedir desfazer. Dados protegidos indisponíveis são declarados na prévia, sem elevar para leitura nem abrir ACLs. Próximo: Bloco F, organização da UI, fluxos, acessibilidade e imagens.
