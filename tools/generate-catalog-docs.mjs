// Public transparency pages generated from the embedded catalog, never the reverse.
import { readFileSync, writeFileSync } from "node:fs";
const root = new URL("../", import.meta.url);
const catalog = JSON.parse(
  readFileSync(new URL("catalog/items.json", root), "utf8"),
);
const items = catalog.items;
const messages = {
  "pt-BR": {
    file: "adjustments.md",
    title: "O que cada ajuste faz",
    intro:
      "Esta página é gerada do catálogo da Ametista. Simule primeiro e confira o estado real do computador. Um item listado não está necessariamente instalado ou disponível na sua edição do Windows.",
    fields: [
      "O que muda",
      "Risco",
      "O que pode deixar de funcionar",
      "Como voltar",
      "Nível",
      "Verificação",
      "Edições declaradas",
      "Build mínimo",
      "Reinício",
    ],
    risk: {
      none: "Nenhum risco funcional conhecido",
      low: "Baixo",
      high: "Alto — seleção manual e confirmação reforçada",
    },
    levels: ["Somente manual", "Seguro", "Equilibrado", "Agressivo"],
    yes: "Sim",
    no: "Não",
    auto: "Use Desfazer para restaurar o estado original registrado antes da aplicação",
    manual:
      "Desfazer não reinstala o app nem recupera seus dados. Use Reinstalar no histórico para abrir a Microsoft Store",
    unverified: "Não verificado — não entra em perfis automáticos",
    verified:
      "Fonte oficial registrada; a compatibilidade ainda depende da versão/edição",
    sources: "Fontes",
    automatic: "Elegível para perfil automático",
    warning:
      "Appx fica sempre fora dos perfis automáticos. Risco alto e itens não verificados nunca são pré-selecionados. A lista de proteção pode bloquear um item mesmo que ele esteja documentado aqui.",
  },
  en: {
    file: "adjustments.en.md",
    title: "What each adjustment does",
    intro:
      "This page is generated from the Ametista catalog. Simulate first and check the real state of your computer. A listed item may not be installed or supported by your Windows edition.",
    fields: [
      "What changes",
      "Risk",
      "What may stop working",
      "How to restore",
      "Level",
      "Verification",
      "Declared editions",
      "Minimum build",
      "Restart",
    ],
    risk: {
      none: "No known functional risk",
      low: "Low",
      high: "High — manual selection and reinforced consent",
    },
    levels: ["Manual only", "Safe", "Balanced", "Aggressive"],
    yes: "Yes",
    no: "No",
    auto: "Use Undo to restore the original state recorded before applying",
    manual:
      "Undo does not reinstall the app or recover its data. Use Reinstall in history to open Microsoft Store",
    unverified: "Unverified — excluded from automatic profiles",
    verified:
      "Official source recorded; compatibility still depends on Windows version/edition",
    sources: "Sources",
    automatic: "Eligible for automatic profiles",
    warning:
      "Appx is always excluded from automatic profiles. High-risk and unverified items are never preselected. The protection list may block an item even when documented here.",
  },
};
const safe = (text) =>
  String(text)
    .replaceAll("\r", "")
    .replaceAll("\n", " ")
    .replaceAll("|", "\\|");
for (const [language, m] of Object.entries(messages)) {
  const blocks = items.map((item) => {
    const values = [
      item.description[language],
      m.risk[item.risk],
      item.riskNote[language],
      item.revert === "auto"
        ? m.auto
        : item.type === "appx"
          ? m.manual
          : language === "en"
            ? "No guaranteed automatic restoration. Check risks and sources; protection may refuse this item"
            : "Não há restauração automática garantida. Confira riscos e fontes; a proteção pode recusar este item",
      m.levels[item.level],
      item.verified ? m.verified : m.unverified,
      item.supportedEditions.join(", "),
      item.minBuild,
      item.requiresReboot ? m.yes : m.no,
    ];
    return `## ${safe(item.name[language])}\n\n\`${item.id}\` · \`${item.type}\`\n\n${m.fields.map((field, index) => `- **${field}:** ${safe(values[index])}`).join("\n")}\n\n${m.sources}: ${item.sources.length ? item.sources.map((url, i) => `[${i + 1}](${url})`).join(", ") : m.unverified}\n`;
  });
  const output = `# ${m.title}\n\n${m.intro}\n\n${m.warning}\n\n${items.length} ${language === "en" ? "catalog items" : "itens no catálogo"}. Fonte: [items.json](../../catalog/items.json).\n\n${blocks.join("\n")}`;
  const target = new URL(`docs/user/${m.file}`, root);
  if (process.argv.includes("--check")) {
    if (readFileSync(target, "utf8") !== output)
      throw new Error(`Stale catalog page: ${m.file}`);
  } else writeFileSync(target, output);
}
console.log(`Verified/generated ${items.length} items in two catalog pages`);
