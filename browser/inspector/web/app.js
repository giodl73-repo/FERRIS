const $=id=>document.getElementById(id);
let revision=0, job=0;
for(const id of ['before','after']) $(id).oninput=()=>{revision++;$('result').textContent='No result.';};
const worker=new Worker('./worker.js',{type:'module'});
worker.onerror=()=>{$('status').textContent='Rust engine failed. Inputs retained; reload to retry.';$('compare').disabled=true;};
worker.onmessage=({data})=>{
 if(data.type==='ready'){$('compare').disabled=false;$('status').textContent='Ready.';return;}
 if(data.id!==job||data.revision!==revision)return;
 $('result').textContent=data.text;$('status').textContent='Comparison completed; no owner work executed.';
};
$('compare').onclick=()=>{job++;$('result').textContent='No result.';worker.postMessage({id:job,revision,before:$('before').value,after:$('after').value});};
const initial=revision;
try{const inputs=await Promise.all(['before','after'].map(async id=>{const response=await fetch('./'+id+'.json');if(!response.ok)throw new Error('Example unavailable');return [id,await response.text()];}));if(revision===initial)for(const [id,text] of inputs)$(id).value=text;}
catch{$('status').textContent='Example unavailable; supply your own JSON.';}
