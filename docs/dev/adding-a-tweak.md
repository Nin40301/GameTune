# Como propor um ajuste

1. Descreva um problema concreto e como medi-lo. Não prometa FPS, limpeza de RAM ou melhora geral sem evidência
2. Localize documentação **oficial da Microsoft** que confirme alvo, valores, versões e edições. Registre URL e o que ela confirma em `catalog-sources.md`. Se não confirmar, use `verified: false`; uma postagem de fórum não torna a chave suportada
3. Confira a lista protegida do core e o escopo: Defender, Update, Firewall, UAC, SmartScreen, rede/áudio/vídeo/autenticação, drivers, TPM/Secure Boot, VBS/HVCI e mitigações nunca são liberados por um ID novo
4. Escolha tipo e categoria existentes, ID estável, nível, risco e descrição pt-BR/en. Explique o que pode quebrar, requisitos de SSD/build/edição, reinício e como voltar. Itens high nunca entram em perfil automático. Appx é manual, com aviso de que Desfazer não reinstala; dependências de Minecraft/Game Pass continuam de alto risco
5. Prove reversão **exata** antes de marcar `revert: auto`: valor bruto e tipo, ausência original, chaves criadas, StartMode/DelayedAutoStart/Running, XML/Enabled, StartupApproved, GUIDs/estado de energia. Se a reversão só recria uma aproximação, não é automática
6. Adicione o alvo tipado ao catálogo fechado. Nenhuma UI, perfil importado ou arquivo externo pode fornecer comando/script/caminho arbitrário de execução. Qualquer novo tipo exige decisão arquitetural e revisão de segurança separadas
7. Atualize hashes/escopos aprovados apenas depois de revisar o diff. O hash detecta corrupção, não prova que um ajuste é seguro. Siga [o formato do catálogo](catalog.md) e os scripts existentes; não edite snapshots/testes só para aceitar uma política perigosa
8. Teste parsing/schema, perfis por nível/risco/edição Home, proteção, primeiro original/idempotência, dry-run sem escrita, falha parcial, crash/recovery e apply → undo com executores mock. Na VM opt-in, comece por **alvos dedicados** e compare snapshot completo; catálogo real exige suíte separada e snapshot/reinícios próprios
9. Gere a documentação pública: `node tools/generate-catalog-docs.mjs`. Revise ambas as páginas. Rode fmt, clippy, testes, lint/TS e documentação; abra PR com fontes, risco, reversão, limites de compatibilidade e evidências

Não aplique o ajuste no PC principal para demonstrar o PR. Preencha o [modelo de proposta](../../.github/ISSUE_TEMPLATE/tweak.yml), peça revisão de CODEOWNERS e mantenha sugestões sem prova no [backlog](../backlog.md). Fonte oficial que apenas documenta uma API não comprova ganho de desempenho ou reversibilidade de uma alteração específica.
