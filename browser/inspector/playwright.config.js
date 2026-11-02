import {defineConfig} from '@playwright/test';
const live=process.env.FERRIS_BASE_URL;
const site=process.env.FERRIS_SITE_DIR||'../../target/profile-release';
export default defineConfig({testDir:'./tests',workers:1,retries:0,timeout:30000,use:{baseURL:live||'http://127.0.0.1:4192/FERRIS/'},reporter:[['list'],['json',{outputFile:live?'.cache/live-browser-results.json':'.cache/browser-results.json'}]],webServer:live?undefined:{command:`python -m http.server 4192 --directory "${site}"`,url:'http://127.0.0.1:4192/FERRIS/',reuseExistingServer:false}});
