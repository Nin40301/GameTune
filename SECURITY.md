# Segurança

Ametista 0.6.5 é uma alfa para VM descartável. Não há versão estável ou garantia de prazo de correção. A tag v0.6.0 é referência histórica, não recomendação de instalação no PC principal.

## Relate em privado

Use **Security → Advisories → Report a vulnerability** ou o [formulário privado do GitHub](https://github.com/Nin40301/GameTune/security/advisories/new). O relato privado está habilitado no repositório. Não abra issue pública com exploit, credenciais ou detalhes que permitam abuso antes da correção coordenada.

Informe versão/commit, edição/build do Windows, impacto, pré-condições e passos mínimos reproduzíveis em VM. Anexe somente evidência revisada e anonimizada. Não envie tokens, certificados, chaves de recuperação, dados pessoais ou journal bruto. Não teste em máquinas de terceiros.

Questões de UAC/IPC, substituição do auxiliar, ACL, validação do catálogo, journal e execução não autorizada são de segurança. Erros comuns de uso podem seguir o modelo público de ajuda, com dados mínimos.

O projeto não pede para desativar Defender, Firewall, SmartScreen, UAC, Secure Boot, TPM, VBS/HVCI ou outros mecanismos de proteção. Não contorne essas proteções para reproduzir uma falha. Veja o [modelo de segurança](docs/security/model.md).
