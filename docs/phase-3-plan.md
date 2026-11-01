# Plano da Fase 3

Criar gametune-cli como consumidor de WindowsSession. Não haverá dependência de
parser: a gramática pequena e fechada terá análise própria e testes de conflitos.
Sem flags, usar perfil Seguro, leitura real e nenhuma alteração de configuração.
Aplicar/desfazer/remover/reinstalar só com --apply e confirmação explícita;
--dry-run prevalece como modo independente e conflita com --apply.

Separar inspeção somente leitura de autorização de escrita. O relatório de
análise incluirá todos os descritores e entradas descobertas, seleção do perfil,
compatibilidade, bloqueio, existência, estado atual e comparação com o alvo.
Não confundir acesso negado ou inventário Appx parcial com ausência. Não criar
ProgramData, WAL, checkpoint ou abrir aplicativos durante dry-run.

JSON terá versão, resultado, códigos estáveis e mensagens localizadas. Texto é
o padrão interativo; --json mantém stdout estritamente JSON. Exportação de relatório
é apenas por redirecionamento explícito do usuário. Mensagens ficam em pt-BR/en.
Appx terá subcomandos próprios, consentimento individual e confirmação reforçada;
reinstalação abre somente link validado a partir de inventário/histórico.

Acrescentar um ensaio destrutivo separado de VM para Seguro/Equilibrado reais.
Exigir opt-in adicional específico e registrar antes/após/reinício/undo, com
checagem funcional local de Loja, Edge, Configurações e Segurança do Windows.
O ensaio será retomável por etapas após reinício; nunca reiniciar automaticamente
o host ou rodar perfis reais na suíte comum/na suíte de fixtures dedicadas.

Confirmar contagem, testar CLI em Linux com backend falso e em Windows sem admin,
manter fmt/Clippy/testes verdes, compilar o harness de VM, documentar códigos de
saída e limitações de validação, fazer commit e entregar fontes/relatórios.
