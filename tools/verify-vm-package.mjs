// Development-only packaging inspection. --list enumerates, never runs tests.
import assert from 'node:assert/strict';
import { readFileSync, readdirSync, realpathSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { resolve, relative, sep } from 'node:path';
const root = realpathSync(resolve(process.argv[2] ?? 'outputs/vm-acceptance'));
const allowed = realpathSync('outputs') + sep;
assert(root.startsWith(allowed), 'Inspect only a generated local output package');
const manifest = JSON.parse(readFileSync(resolve(root, 'package.json'), 'utf8'));
assert.equal(manifest.staticCrt, true);
assert.equal(manifest.cliFeatures, 'default only');
assert.equal(Object.keys(manifest.files).length, 9);
for (const [name, hash] of Object.entries(manifest.files)) {
  const path = realpathSync(resolve(root, name));
  assert(!relative(root, path).startsWith('..'));
  assert.equal(createHash('sha256').update(readFileSync(path)).digest('hex'), hash);
}
for (const name of readdirSync(resolve(root, 'bin'))) {
  assert(name.endsWith('.exe'));
  const b = readFileSync(resolve(root, 'bin', name));
  const pe = b.readUInt32LE(0x3c), opt = pe + 24;
  assert.equal(b.toString('ascii', pe, pe + 4), 'PE\0\0');
  assert.equal(b.readUInt16LE(opt), 0x20b);
  const sections = [];
  for (let i = 0; i < b.readUInt16LE(pe + 6); i++) {
    const at = opt + b.readUInt16LE(pe + 20) + 40 * i;
    sections.push({ va: b.readUInt32LE(at + 12), size: Math.max(b.readUInt32LE(at + 8), b.readUInt32LE(at + 16)), ptr: b.readUInt32LE(at + 20) });
  }
  const position = rva => {
    const s = sections.find(s => s.va <= rva && rva < s.va + s.size);
    assert(s, 'Invalid import RVA');
    return s.ptr + rva - s.va;
  };
  let at = position(b.readUInt32LE(opt + 120));
  while (b.subarray(at, at + 20).some(v => v !== 0)) {
    const start = position(b.readUInt32LE(at + 12));
    const dll = b.toString('ascii', start, b.indexOf(0, start));
    assert(!/^(vcruntime|msvcp|msvcr)/i.test(dll), `Non-inbox CRT: ${dll}`);
    at += 20;
  }
}
if (process.platform === 'win32') {
  for (const [name, args, count] of [
    ['vm-tests', ['--list'], 2],
    ['vm-unit-tests', ['dedicated_hkcu', '--list'], 2],
    ['real-profile-tests', ['--list'], 1],
  ]) {
    const text = execFileSync(resolve(root, 'bin', name + '.exe'), args, { encoding: 'utf8', timeout: 10000 });
    assert.equal((text.match(/: test$/gm) ?? []).length, count, 'Missing acceptance tests');
  }
}
console.log('VM kit: nine manifest hashes, five x64 binaries/static CRT and 2+2+1 test declarations verified; no administrative test executed');
