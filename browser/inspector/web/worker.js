import init,{compare_profiles} from './pkg/ferris_profile_inspector.js';
const started=performance.now();await init();postMessage({type:'ready',initialization_ms:performance.now()-started});
onmessage=({data})=>{try{const started=performance.now(),text=compare_profiles(data.before,data.after);postMessage({type:'result',id:data.id,text,computation_ms:performance.now()-started});}catch(error){postMessage({type:'error',id:data.id,message:String(error)});}};
