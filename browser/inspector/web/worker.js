import init,{compare_profiles} from './pkg/ferris_profile_inspector.js';
await init();postMessage({type:'ready'});
onmessage=({data})=>postMessage({id:data.id,revision:data.revision,text:compare_profiles(data.before,data.after)});
