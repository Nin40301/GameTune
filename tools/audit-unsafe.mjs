// Inventory every Rust unsafe boundary; documentation must match source after edits.
import { readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import path from 'node:path';
const root = path.resolve(import.meta.dirname, '..');
const files = [];
function walk(dir) {
  for (const entry of readdirSync(path.join(root, dir), { withFileTypes: true })) {
    if (['target', 'node_modules', 'work', 'dist', 'gen'].includes(entry.name)) continue;
    const name = `${dir}/${entry.name}`;
    if (entry.isDirectory()) walk(name);
    else if (entry.name.endsWith('.rs')) files.push(name);
  }
}
walk('crates'); walk('apps/desktop/src-tauri');
const reasons = {
  'crates/windows_runtime/src/icon/native.rs': 'Shell extrai um recurso de arquivo local cadastrado; GDI usa bitmap fixo de 32x32 e recursos próprios liberados por RAII. Sem executar o arquivo, rede, hooks ou handles em processos.',
  'crates/ipc/src/native.rs': 'ACLs, SID/token próprio, pipe autenticado, arquivo do auxiliar e ShellExecute/WinVerifyTrust offline. Buffers e handles pertencem à chamada; nunca processos de jogos.',
  'crates/windows_runtime/src/native.rs': 'WTS da sessão local, token próprio, pasta conhecida e SCM. Alocações do Windows copiadas/liberadas uma vez; estruturas alinhadas e limitadas; handles são de serviços, nunca de jogos.',
  'crates/metrics/src/native.rs': 'Contadores de sistema e snapshot de processos do sistema; buffers/alinhamento e tamanhos verificados. Nenhum OpenProcess de terceiros.',
  'crates/metrics/src/resource_native.rs': 'Gate somente leitura: processos da árvore da própria aplicação e seus contadores. Não é usado para acessar jogos.',
  'crates/watcher/src/events.rs': 'Eventos/snapshot do sistema para nome, caminho e PID. Buffers COM/ETW pertencem à fronteira; nenhum handle no jogo.',
  'crates/watcher/src/fs_native.rs': 'GetDriveTypeW com raiz local validada; recusa unidades de rede antes de ler launchers. Não acessa processos.',
  'crates/watcher/src/wmi.rs': 'COM/WMI local em worker MTA próprio; interfaces e VARIANTs RAII, só eventos/snapshot de nome/caminho/PID da sessão, sem handles de jogos.',
  'crates/monitor/src/tray.rs': 'Somente janela, ícone e fila de mensagens da própria bandeja, pasta conhecida local. Recursos liberados no encerramento.',
};
let rows = [], digests = [];
for (const name of files.sort()) {
  const text = readFileSync(path.join(root, name), 'utf8').replaceAll('\r\n', '\n');
  let fn = 'módulo'; let found = false;
  for (const [index, line] of text.split('\n').entries()) {
    if (line.trimStart().startsWith('//')) continue;
    const match = line.match(/\bfn\s+(\w+)/);
    if (match) fn = match[1];
    if (/\bunsafe\s*\{|\bunsafe\s+extern/.test(line)) {
      const reason = reasons[name];
      if (!reason) throw new Error(`Unsafe boundary needs explicit review: ${name}:${index + 1}`);
      rows.push(`| [${name}:${index + 1}](../../${name}#L${index + 1}) | \`${fn}\` | ${reason} |`);
      found = true;
    }
  }
  if (found) digests.push(`- \`${name}\`: \`${createHash('sha256').update(text).digest('hex')}\``);
  if (/\.(unwrap|expect)\s*\(/.test(text)) throw new Error(`Forbidden unwrap/expect: ${name}`);
}
const output = `# Inventário de unsafe\n\nGerado por \`node tools/audit-unsafe.mjs --write\`. Cada linha com bloco unsafe/extern é listada; várias chamadas no mesmo bloco compartilham a justificativa. Revisão de código não substitui ensaio em VM, Miri ou auditoria independente. Fora destas fronteiras, unsafe continua proibido/negado; unwrap/expect são rejeitados pelo lint e pelo inventário.\n\n${rows.length} linhas de fronteira, sem handles em processos de jogos. As justificativas abaixo delimitam o propósito; os comentários SAFETY no código descrevem a validade dos ponteiros e a propriedade dos recursos.\n\n| Local | Função | Justificativa |\n| --- | --- | --- |\n${rows.join('\n')}\n\n## Hashes do código revisado (UTF-8/LF)\n\n${digests.join('\n')}\n`;
const target = path.join(root, 'docs/security/unsafe-inventory.md');
if (process.argv.includes('--write')) writeFileSync(target, output);
else if (readFileSync(target, 'utf8') !== output) throw new Error('Unsafe inventory is stale; review every change before regenerating');
console.log(`Reviewed ${rows.length} unsafe source lines; no unwrap/expect`);
