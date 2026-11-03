// Offline source check: shared keys and no prose embedded in rendered JSX/labels.
import assert from 'node:assert/strict';
import { readdirSync, readFileSync } from 'node:fs';
import path from 'node:path';
import ts from '../apps/desktop/node_modules/typescript/lib/typescript.js';
const root = path.resolve(import.meta.dirname,'../apps/desktop/src');
const pt = JSON.parse(readFileSync(path.join(root,'locales/pt-BR.json'),'utf8'));
const en = JSON.parse(readFileSync(path.join(root,'locales/en.json'),'utf8'));
assert.deepEqual(Object.keys(pt).sort(), Object.keys(en).sort());
for(const dictionary of [pt,en]) for(const text of Object.values(dictionary)) assert.equal(typeof text,'string');
let count=0; const errors=[];
function walk(dir) {
  for(const entry of readdirSync(dir,{withFileTypes:true})) {
    const name=path.join(dir,entry.name);
    if(entry.isDirectory())walk(name);
    else if(entry.name.endsWith('.tsx')) {
      count++;
      const source=ts.createSourceFile(name,readFileSync(name,'utf8'),ts.ScriptTarget.Latest,true,ts.ScriptKind.TSX);
      function fail(node,text){ const loc=source.getLineAndCharacterOfPosition(node.getStart(source)); errors.push(`${path.relative(root,name)}:${loc.line+1}: ${text.trim()}`); }
      function visit(node) {
        if(ts.isJsxText(node) && /\p{L}/u.test(node.text)) fail(node,node.text);
        if(ts.isJsxAttribute(node) && /^(aria-label|aria-description|placeholder|title|alt)$/.test(node.name.getText(source)) && node.initializer && ts.isStringLiteral(node.initializer) && /\p{L}/u.test(node.initializer.text)) fail(node,node.initializer.text);
        if(ts.isCallExpression(node) && node.expression.getText(source)==='t' && node.arguments.length===1 && ts.isStringLiteral(node.arguments[0]) && !Object.hasOwn(pt,node.arguments[0].text)) fail(node,`unknown translation ${node.arguments[0].text}`);
        ts.forEachChild(node,visit);
      }
      visit(source);
    }
  }
}
walk(root);
if(errors.length)throw new Error(errors.join('\n'));
console.log(`Verified ${Object.keys(pt).length} shared locale keys and ${count} JSX files without embedded UI prose`);
