// Cross-stack release consistency, without launching an application or changing Windows.
import assert from 'node:assert/strict';
import { readFileSync, existsSync } from 'node:fs';
import { createHash } from 'node:crypto';
const root = new URL('../', import.meta.url);
const read = (name) => readFileSync(new URL(name, root), 'utf8');
const json = (name) => JSON.parse(read(name));
const manifest = json('apps/desktop/package.json');
const version = manifest.version;
assert.match(version, /^\d+\.\d+\.\d+$/);
assert.equal(read('Cargo.toml').match(/\[workspace\.package\][\s\S]*?^version = "([^"]+)"/m)?.[1], version);
const lock = json('apps/desktop/package-lock.json');
assert.equal(lock.version, version);
assert.equal(lock.packages[''].version, version);
assert.deepEqual(lock.packages[''].engines, manifest.engines);
assert.equal(read('.nvmrc').trim(), manifest.engines.node);
assert.equal(read('apps/desktop/.nvmrc').trim(), manifest.engines.node);
assert.equal(manifest.engines.npm, '11.20.0');
assert.match(read('apps/desktop/.npmrc'), /^engine-strict=true/m);
assert.match(read('rust-toolchain.toml'), /^channel = "1\.98\.1"/m);
const config = json('apps/desktop/src-tauri/tauri.conf.json');
assert.equal(config.version, version);
assert.equal(config.productName, 'Ametista');
assert.equal(config.identifier, 'io.github.nin40301.ametista');
assert.equal(config.app.windows[0].title, 'Ametista');
for (const icon of config.bundle.icon) assert.ok(existsSync(new URL('apps/desktop/src-tauri/' + icon, root)));
const cargoPackages = [...read('Cargo.lock').matchAll(/\[\[package\]\]\r?\nname = "(ametista-[^"]+)"\r?\nversion = "([^"]+)"/g)];
assert.ok(cargoPackages.length >= 20);
for (const [, name, actual] of cargoPackages) assert.equal(actual, version, name);
for (const name of ['apps/desktop/src-tauri/app.manifest', 'crates/helper/helper.manifest', 'crates/monitor/monitor.manifest']) {
  assert.equal(read(name).match(/assemblyIdentity version="([^"]+)"/)?.[1], version + '.0', name);
}
for (const name of ['crates/helper/helper.rc', 'crates/monitor/monitor.rc']) {
  const text = read(name);
  assert.ok(text.includes('FILEVERSION ' + version.replaceAll('.', ',') + ',0'), name);
  assert.ok(text.includes('PRODUCTVERSION ' + version.replaceAll('.', ',') + ',0'), name);
  assert.ok(text.includes(`"ProductVersion", "${version}\\0"`), name);
  assert.ok(text.includes(`"FileVersion", "${version}\\0"`), name);
}
for (const name of ['apps/desktop/src-tauri/src/games.rs', 'crates/monitor/src/tray.rs', 'tools/test-resource-budget.ps1']) {
  assert.ok(read(name).includes(config.identifier), name);
  assert.ok(!read(name).includes('local.ametista.desktop'), name);
}
// Public measurement hashes must survive a Git checkout's normalized LF endings.
const baseline = json('docs/perf-baseline.json');
for (const mode of ['full', 'reduced']) {
  assert.equal(baseline.sources[mode].length, 5);
  for (const source of baseline.sources[mode]) {
    assert.equal(createHash('sha256').update(readFileSync(new URL(source.path, root))).digest('hex'), source.sha256, source.path);
  }
}
assert.equal(createHash('sha256').update(readFileSync(new URL(baseline.monitor.report, root))).digest('hex'), baseline.monitor.reportSha256);
console.log(`Verified Ametista ${version}, stable identifier, Win32 resources and public baseline hashes`);
