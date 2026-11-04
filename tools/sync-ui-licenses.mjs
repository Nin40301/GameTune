// Local legal texts, bundled offline; CI detects stale or altered copies.
import {readFileSync,writeFileSync} from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
const root=path.resolve(import.meta.dirname,'..');
const sources=['NOTICE','LICENSE','LICENSES/PlusJakartaSans-OFL-1.1.txt','LICENSES/lucide-ISC.txt'];
const output=JSON.stringify(sources.map(source=>({source,text:readFileSync(path.join(root,source),'utf8').replaceAll('\r\n','\n')})),null,2)+'\n';
const target=path.join(root,'apps/desktop/src/legal.json');
if(process.argv.includes('--write'))writeFileSync(target,output);
else assert.equal(readFileSync(target,'utf8'),output,'Run node tools/sync-ui-licenses.mjs --write after legal text changes');
console.log('Offline UI legal notices match their source files');
