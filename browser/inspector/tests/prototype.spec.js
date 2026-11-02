import {test,expect} from '@playwright/test';
import fs from 'node:fs';
test('real worker compares the public example with exact native result',async({page})=>{
 await page.goto('./');await expect(page.locator('#compare')).toBeEnabled();await expect(page.locator('#before')).not.toHaveValue('');await expect(page.locator('#after')).not.toHaveValue('');
 await page.locator('#compare').click();await expect(page.locator('#status')).toContainText('Comparison completed');
 const cases=JSON.parse(fs.readFileSync('.cache/native-cases.json','utf8'));expect(await page.locator('#result').textContent()).toBe(cases[0].output);expect(JSON.parse(cases[0].output).record.executable).toBe(false);
 const metrics=await page.locator('#status').evaluate(node=>({measurements:{...node.dataset},user_agent:navigator.userAgent,hardware_concurrency:navigator.hardwareConcurrency,scope:'Worker creation-to-ready and request-to-result samples with JS/message/serialization overhead; not whole-page load, build speed or comparative performance.'}));
 for(const name of ['startupMs','workerInitializationMs','queryRoundtripMs','workerComputationMs','inputBytes'])expect(Number.isFinite(Number(metrics.measurements[name]))).toBe(true);
 fs.writeFileSync('.cache/worker-timings.json',JSON.stringify(metrics,null,2));
});
test('own input and invalid source retain native classifications',async({page})=>{
 await page.goto('./');await expect(page.locator('#compare')).toBeEnabled();await expect(page.locator('#after')).not.toHaveValue('');
 await page.locator('#before').fill('{');await page.locator('#compare').focus();await page.keyboard.press('Enter');await expect(page.locator('#status')).toContainText('Comparison completed');expect(JSON.parse(await page.locator('#result').textContent()).result_class).toBe('invalid');
});
test('every portable native response matches executing WASM exactly',async({page})=>{
 await page.goto('./');const cases=JSON.parse(fs.readFileSync('.cache/native-cases.json','utf8'));
 const responses=await page.evaluate(async cases=>{const m=await import('./pkg/ferris_profile_inspector.js');await m.default();return cases.map(c=>m.compare_profiles(c.before,c.after));},cases);
 expect(responses).toEqual(cases.map(c=>c.output));
});
