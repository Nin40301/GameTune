# Use a interface

**Alfa 0.6.5: teste em VM descartável com snapshot. Não use no PC principal.** Não há instalador final assinado nesta fase. A [preparação de build](../dev/build.md) é destinada a desenvolvimento/testes; aplicar exige uma instalação protegida e aceite na VM.

## Simule primeiro

Abra Ametista como usuário comum. Em Início, escolha Seguro e selecione Simule. A análise consulta o que existe no Windows; não aplica mudanças. Aguarde o progresso ou cancele. Confira os resultados e abra Itens para ler nome, descrição, risco e reversão. Um item ausente ou indisponível não é uma recomendação para instalar/forçar algo.

## Revise antes de aplicar

Use Revise os ajustes. Confira o resumo por tipo e as consequências. Aplique somente os itens que entende, na VM, com a confirmação exigida. O UAC pertence ao auxiliar daquela operação; não execute a interface inteira como administrador. Se a análise ficar desatualizada ou o app pedir para analisar de novo, siga essa orientação.

Itens de alto risco não entram em perfis automáticos. As dependências de Minecraft Bedrock, Game Pass e jogos da Loja recebem proteção extra. Não desative proteções do Windows para tentar melhorar desempenho.

## Remova apps separadamente

Em Limpeza de apps, selecione apenas apps do catálogo instalados. Leia o aviso e dê o consentimento de cada app na revisão. Desfazer não reinstala, não recupera dados e não garante a versão original. O histórico oferece Abra na Loja quando houver registro para reinstalação; isso pode precisar de internet.

## Desfaça e confira

Em Histórico, use Desfaça tudo ou a ação de um item, confira a revisão e confirme. Preserve qualquer erro/conflito e o journal; não edite os arquivos para forçar uma restauração. Se o PC apresentar problema, pare de aplicar e siga [o guia de recuperação](recovery.md).

## Jogos, medições e preferências

- [Jogos](games.md): escolha um executável e um perfil temporário; habilite o monitor somente quando quiser. Ele é um processo Rust separado e pode continuar com a janela fechada
- [Medições](benchmark.md): compare condições equivalentes e a variação entre rodadas. O app não promete FPS
- Configurações: escolha idioma, tema e efeitos. Completo é o padrão novo; Reduzido permanece opção de transparência/acessibilidade. Preferências existentes são preservadas

Os limites e a medição de consumo estão em [performance.md](../performance.md). Validação em NVDA/Narrador, DPI físico e matriz de Windows permanece pendente; testes automatizados não são uma certificação de acessibilidade.
