// Render repository-native vector assets locally; no CDN or image generator.
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { chromium } from '../apps/desktop/node_modules/playwright/index.mjs';
const root = new URL('../',import.meta.url);
const output = new URL('apps/desktop/src-tauri/icons/',root);
mkdirSync(output,{recursive:true});
const sizes=[16,20,24,32,40,48,64,128,256];
const browser=await chromium.launch();
try {
  for(const kind of ['app','tray']) {
    const svg=readFileSync(new URL(`apps/desktop/src/assets/brand/${kind}.svg`,root),'utf8');
    const frames=[];
    for(const size of sizes) {
      const page=await browser.newPage({viewport:{width:size,height:size},deviceScaleFactor:1});
      await page.setContent(`<style>html,body{margin:0;padding:0;background:transparent}svg{display:block;width:${size}px;height:${size}px}</style>${svg}`);
      const png=await page.screenshot({omitBackground:true});
      frames.push({size,png});
      writeFileSync(new URL(`${kind}-${size}.png`,output),png);
      await page.close();
    }
    // ICO directory + PNG frames, as supported by Windows Vista and later.
    const header=Buffer.alloc(6+16*frames.length);header.writeUInt16LE(1,2);header.writeUInt16LE(frames.length,4);
    let offset=header.length;
    for(const [i,{size,png}] of frames.entries()) {
      const at=6+16*i;header[at]=size===256?0:size;header[at+1]=header[at];
      header.writeUInt16LE(1,at+4);header.writeUInt16LE(32,at+6);header.writeUInt32LE(png.length,at+8);header.writeUInt32LE(offset,at+12);offset+=png.length;
    }
    writeFileSync(new URL(kind==='app'?'icon.ico':'tray.ico',output),Buffer.concat([header,...frames.map(frame=>frame.png)]));
  }
} finally {await browser.close();}
console.log('Generated two ICOs with 9 sizes each and 18 PNG inspection frames');
