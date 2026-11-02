// Offline repository-local link check. Does not fetch external URLs or execute examples.
import { readdirSync, readFileSync, existsSync, statSync } from 'node:fs';
import { dirname, resolve, relative, sep } from 'node:path';
import { fileURLToPath } from 'node:url';
const root = fileURLToPath(new URL('../', import.meta.url));
function markdown(folder) {
  return readdirSync(folder, { withFileTypes: true }).flatMap((entry) => {
    const path = resolve(folder, entry.name);
    return entry.isDirectory() ? markdown(path) : entry.name.endsWith('.md') ? [path] : [];
  });
}
const files = [...markdown(resolve(root, 'docs')), ...readdirSync(root).filter((name) => name.endsWith('.md')).map((name) => resolve(root, name)), ...markdown(resolve(root, '.github'))];
const errors = [];
let checked = 0;
for (const file of files) {
  const text = readFileSync(file, 'utf8').replace(/```[\s\S]*?```/g, '');
  const urls = [...text.matchAll(/!?\[[^\]]*\]\((?:<([^>]+)>|([^\s)]+))(?:\s+"[^"]*")?\)/g)].map((m) => m[1] ?? m[2]);
  urls.push(...[...text.matchAll(/\b(?:src|srcset|href)="([^"]+)"/g)].map((m) => m[1]));
  for (const url of urls) {
    if (/^(?:[a-z][a-z0-9+.-]*:|#)/i.test(url)) continue;
    const plain = decodeURIComponent(url.split('#')[0].split('?')[0]);
    if (!plain) continue;
    const target = resolve(dirname(file), plain);
    const name = relative(root, target);
    checked++;
    if (name.startsWith('..' + sep) || !existsSync(target)) {
      errors.push(`${relative(root, file)}: ${url}`);
      continue;
    }
    // Case-sensitive spelling is required even on Windows (GitHub/Linux).
    let parent = root;
    for (const part of name.split(sep)) {
      if (!part) continue;
      if (!statSync(parent).isDirectory() || !readdirSync(parent).includes(part)) {
        errors.push(`${relative(root, file)}: wrong case ${url}`);
        break;
      }
      parent = resolve(parent, part);
    }
  }
}
if (errors.length) { console.error(errors.join('\n')); process.exitCode = 1; }
else console.log(`Verified ${checked} local links across ${files.length} Markdown files (external URLs and anchors not fetched)`);
