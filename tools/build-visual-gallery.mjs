// Offline evidence index. Images remain untouched; hashes identify every capture.
import {readFileSync,writeFileSync,readdirSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import path from 'node:path';
const root=path.resolve(import.meta.dirname,'../outputs/phase-6.7/delivery-2');
const files=['screenshots','states'].flatMap(directory=>readdirSync(path.join(root,directory)).filter(name=>name.endsWith('.png')).map(name=>`${directory}/${name}`));
const escape=text=>text.replaceAll('&','&amp;').replaceAll('<','&lt;').replaceAll('"','&quot;');
const manifest={sourceCommit:execFileSync('git',['rev-parse','HEAD'],{encoding:'utf8'}).trim(),generatedUtc:new Date().toISOString(),mockData:true,files:files.sort().map(file=>({file,sha256:createHash('sha256').update(readFileSync(path.join(root,file))).digest('hex')}))};
writeFileSync(path.join(root,'screenshots-manifest.json'),JSON.stringify(manifest,null,2)+'\n');
writeFileSync(path.join(root,'gallery.html'),`<!doctype html><html lang="pt-BR"><meta charset="utf-8"><meta name="viewport" content="width=device-width"><meta http-equiv="Content-Security-Policy" content="default-src 'none'; img-src 'self'; style-src 'self'; base-uri 'none'; form-action 'none'"><link rel="stylesheet" href="gallery.css"><title>Ametista · capturas de teste</title><h1>Ametista · capturas de teste</h1><p>Dados de demonstração identificados na tela. As imagens não são medições reais. Abra uma captura para conferir o tamanho original. Barras fixas aparecem na posição do viewport nas capturas de documento inteiro.</p><main>${files.sort().map(file=>`<figure><a href="${escape(file)}"><img loading="lazy" src="${escape(file)}" alt="${escape(path.basename(file))}"></a><figcaption>${escape(path.basename(file))}</figcaption></figure>`).join('')}</main></html>`);
writeFileSync(path.join(root,'gallery.css'),'body{font:15px/1.6 system-ui,sans-serif;background:#f6f2ff;color:#1f1a33;margin:24px}p{max-width:72ch}main{display:grid;grid-template-columns:repeat(auto-fit,minmax(280px,1fr));gap:24px}figure{margin:0;padding:16px;background:white;border:1px solid #d9cbff;border-radius:16px}img{width:100%;height:300px;object-fit:contain}a:focus-visible{outline:3px solid #6d28d9}figcaption{overflow-wrap:anywhere;margin-top:12px}');
console.log(`Indexed ${files.length} unmodified offline screenshots`);
