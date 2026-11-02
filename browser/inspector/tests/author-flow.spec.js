import {test,expect} from '@playwright/test';
import fs from 'node:fs';
const before=fs.readFileSync(new URL('../web/before.json',import.meta.url),'utf8').replace(/\r\n?/g,'\n');
const after=fs.readFileSync(new URL('../web/after.json',import.meta.url),'utf8').replace(/\r\n?/g,'\n');
async function open(page){await page.goto('./');await expect(page.locator('#compare')).toBeEnabled();await expect(page.locator('#before')).not.toHaveValue('');await expect(page.locator('#after')).not.toHaveValue('');}
async function compare(page){await page.locator('#compare').click();await expect(page.locator('#status')).toContainText(/Comparison completed|Browser declined/);}
async function download(page,id){const [file]=await Promise.all([page.waitForEvent('download'),page.locator(id).click()]);return fs.readFileSync(await file.path());}

test('paired original bytes survive explicit LF copies and edits independently',async({page})=>{
 await open(page);const raw=Buffer.from('\uFEFF'+before.replace(/\n/g,'\r\n'));
 await page.locator('#before-file').setInputFiles({name:'before.json',mimeType:'application/json',buffer:raw});await expect(page.locator('#before-file-status')).toContainText('original bytes retained');
 expect(await download(page,'#before-original')).toEqual(raw);expect(await download(page,'#before-current')).toEqual(raw);expect(await download(page,'#after-current')).toEqual(Buffer.from(after));
 await compare(page);expect(JSON.parse(await page.locator('#result').textContent()).result_class).toBe('invalid');
 await page.locator('#before-lf').click();await expect(page.locator('#download')).toBeDisabled();await compare(page);expect(JSON.parse(await page.locator('#result').textContent()).result_class).toBe('difference');
 expect(await download(page,'#before-original')).toEqual(raw);expect(await download(page,'#before-current')).toEqual(Buffer.from(before));
 await page.locator('#after-file').setInputFiles({name:'after.json',mimeType:'application/json',buffer:Buffer.from(after)});await expect(page.locator('#after-file-status')).toContainText('original bytes retained');await page.locator('#after').fill('{}');await expect(page.locator('#download')).toBeDisabled();expect(await download(page,'#after-original')).toEqual(Buffer.from(after));expect(await download(page,'#before-original')).toEqual(raw);
});

test('invalid UTF-8, oversized file, paste and drop retain accepted input',async({page})=>{
 await open(page);for(const [name,buffer] of [['invalid.json',Buffer.from([255])],['large.json',Buffer.alloc(1048577,32)]]){await page.locator('#before-file').setInputFiles({name,mimeType:'application/json',buffer});await expect(page.locator('#before-file-status')).toContainText('retained');expect(await download(page,'#before-current')).toEqual(Buffer.from(before));}
 for(const type of ['paste','drop']){await page.locator('#before').evaluate((input,type)=>{input.setSelectionRange(input.value.length,input.value.length);const data=new DataTransfer();data.setData('text/plain','x'.repeat(1048577));input.dispatchEvent(type==='paste'?new ClipboardEvent('paste',{clipboardData:data,bubbles:true,cancelable:true}):new DragEvent('drop',{dataTransfer:data,bubbles:true,cancelable:true}));},type);await expect(page.locator('#before-file-status')).toContainText('Entire operation rejected');expect(await download(page,'#before-current')).toEqual(Buffer.from(before));}
});

test('typed status, native digests and redaction remain explicit and inert',async({page})=>{
 await open(page);await compare(page);await expect(page.locator('.class-title')).toHaveText('Structural differences');await expect(page.locator('tbody')).toContainText('changed');await expect(page.locator('tbody')).toContainText('sha256:');
 const input=JSON.parse(after);input.sections.support={'visible/~field':'<img src=x onerror=alert(1)> RAW_VALUE_STAYS_OUT_OF_REPORT'};await page.locator('#after').fill(JSON.stringify(input));await compare(page);await expect(page.locator('#summary')).toContainText('visible~1~0field');expect(await page.locator('#summary img,#summary script').count()).toBe(0);expect((await download(page,'#download')).toString()).not.toContain('RAW_VALUE_STAYS_OUT_OF_REPORT');
 await page.locator('#after').fill(before);await compare(page);await expect(page.locator('.class-title')).toHaveText('No structural differences');
 const unsupported=JSON.parse(after);unsupported.schema='unknown/v8';await page.locator('#after').fill(JSON.stringify(unsupported));await compare(page);await expect(page.locator('.class-title')).toHaveText('Unsupported evidence schema');
});

test('bounded page rendering preserves all native changes in report download',async({page})=>{
 await open(page);const b=JSON.parse(before),a=JSON.parse(before);b.sections.features=[];a.sections.features=Array(250).fill(0);await page.locator('#before').fill(JSON.stringify(b));await page.locator('#after').fill(JSON.stringify(a));await compare(page);
 expect(await page.locator('tbody tr').count()).toBe(100);await page.getByRole('button',{name:'Show next 100 changes'}).click();expect(await page.locator('tbody tr').count()).toBe(200);expect(JSON.parse((await download(page,'#download')).toString()).record.changes.length).toBe(250);
});

test('resource refusals stay separate and runtime memory ceiling is enforced',async({page})=>{
 await open(page);const input=JSON.parse(before);let tree=0;for(let i=0;i<30;i++)tree=[tree];input.sections.features=tree;await page.locator('#after').fill(JSON.stringify(input));await compare(page);await expect(page.locator('#result-state')).toHaveText('Browser refusal');const refused=JSON.parse((await download(page,'#download')).toString());expect(refused.schema).toBe('ferris.browser-admission/v1');expect(refused.result_class).toBeUndefined();
 const ceiling=await page.evaluate(async()=>{const m=await import('./pkg/ferris_profile_inspector.js');const instance=await m.default();try{instance.memory.grow(1024);return false;}catch(error){return error instanceof RangeError;}});expect(ceiling).toBe(true);
});

test('file reads cannot overwrite newer edits or selections and have a deadline',async({page})=>{
 await page.addInitScript(()=>{const original=Blob.prototype.arrayBuffer;Blob.prototype.arrayBuffer=function(){if(this.name==='stuck.json')return new Promise(()=>{});const promise=original.call(this);return this.name==='slow.json'?new Promise(resolve=>setTimeout(()=>resolve(promise),800)):promise;};});
 await open(page);await page.locator('#before-file').setInputFiles({name:'slow.json',mimeType:'application/json',buffer:Buffer.from(before)});await page.locator('#before').fill('{}');await page.waitForTimeout(1000);await expect(page.locator('#before')).toHaveValue('{}');await expect(page.locator('#before-file-status')).toContainText('Pending read canceled');
 await page.locator('#before-file').setInputFiles({name:'slow.json',mimeType:'application/json',buffer:Buffer.from(before)});await page.locator('#before-file').setInputFiles({name:'new.json',mimeType:'application/json',buffer:Buffer.from('{}')});await expect(page.locator('#before-file-status')).toContainText('Opened new.json');await page.waitForTimeout(1000);expect(await download(page,'#before-original')).toEqual(Buffer.from('{}'));
 await page.locator('#before-file').setInputFiles({name:'stuck.json',mimeType:'application/json',buffer:Buffer.from(before)});await expect(page.locator('#before-file-status')).toContainText('File read exceeded 5 seconds.',{timeout:7000});await expect(page.locator('#compare')).toBeEnabled();expect(await download(page,'#before-original')).toEqual(Buffer.from('{}'));
});

test('startup timeout and crash retain input, failure guidance and recovery',async({page})=>{
 await page.route('**/worker.js',route=>route.fulfill({contentType:'text/javascript',body:'onmessage=()=>{};'}));await page.goto('./');await expect(page.locator('#before')).not.toHaveValue('');await expect(page.locator('#status')).toContainText('Engine startup exceeded 12 seconds.',{timeout:15000});expect(await download(page,'#before-current')).toEqual(Buffer.from(before));await page.locator('#before').fill(before+' ');await expect(page.locator('#status')).toContainText('Engine startup exceeded 12 seconds.');
 await page.unroute('**/worker.js');await page.route('**/worker.js',route=>route.fulfill({contentType:'text/javascript',body:"throw new Error('fixture crash');"}));await page.locator('#restart').click();await expect(page.locator('#status')).toContainText('Rust engine failed.');await page.unroute('**/worker.js');await page.locator('#restart').click();await expect(page.locator('#compare')).toBeEnabled();await compare(page);
});

test('computation timeout clears exports and restart retains both documents',async({page})=>{
 await page.route('**/worker.js',route=>route.fulfill({contentType:'text/javascript',body:"postMessage({type:'ready'});onmessage=()=>{};"}));await open(page);await page.locator('#compare').click();await expect(page.locator('#status')).toContainText('Comparison exceeded 10 seconds.',{timeout:13000});await expect(page.locator('#download')).toBeDisabled();expect(await download(page,'#before-current')).toEqual(Buffer.from(before));expect(await download(page,'#after-current')).toEqual(Buffer.from(after));await page.unroute('**/worker.js');await page.locator('#restart').click();await expect(page.locator('#compare')).toBeEnabled();await compare(page);
});

test('old worker generation, errors and wrong request IDs cannot publish',async({page})=>{
 await page.addInitScript(()=>{window.workers=[];window.Worker=class{constructor(){window.workers.push(this);setTimeout(()=>this.onmessage?.({data:{type:'ready'}}),0);}postMessage(request){this.request=request;}terminate(){}};});await open(page);await page.locator('#compare').click();await page.locator('#after').fill('{}');await expect(page.locator('#compare')).toBeEnabled();await page.evaluate(()=>{const old=window.workers[0];old.onmessage({data:{id:old.request.id,text:'OLD'}});old.onerror();});await expect(page.locator('#download')).toBeDisabled();await page.locator('#compare').click();await page.evaluate(()=>window.workers.at(-1).onmessage({data:{id:'wrong',text:'wrong'}}));await expect(page.locator('#download')).toBeDisabled();
});

test('mobile keyboard flow has readable output and no horizontal page overflow',async({page})=>{
 await page.setViewportSize({width:390,height:844});await open(page);await page.locator('#compare-bottom').focus();await page.keyboard.press('Enter');await expect(page.locator('#status')).toContainText('Comparison completed');await expect(page.locator('.class-title')).toHaveText('Structural differences');expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);await page.screenshot({path:'.cache/mobile.png',fullPage:true});await page.setViewportSize({width:1440,height:1100});await page.screenshot({path:'.cache/desktop.png',fullPage:true});
});
