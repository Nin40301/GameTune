import {readFileSync,readdirSync} from 'node:fs';
const dir=new URL('../apps/desktop/dist/assets/',import.meta.url);
for(const file of readdirSync(dir).filter(name=>name.endsWith('.js'))) {
  const source=readFileSync(new URL(file,dir),'utf8');
  if(/AMETISTA_DESIGN_SYSTEM_DEV_ONLY|\/design-system|__ametistaPseudo|createMockBackend|__ametistaTest/.test(source)) throw new Error(`Development UI leaked into ${file}`);
}
console.log('Production assets exclude design-system, pseudo-localization and mock backend');
