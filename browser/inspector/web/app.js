const $ = id => document.getElementById(id), MAX_BYTES = 1048576;
const docs = Object.fromEntries(['before','after'].map(id => [id,{id,text:'',revision:0,readToken:0,reading:false,original:null}]));
let revision=0, generation=0, worker=null, ready=false, pending=null, jobTimer;
let report=null, exampleToken=0, exampleController=null, engineMessage='Starting Rust engine...';
const bytes=text=>new TextEncoder().encode(text).length;
function element(tag,text,className){const node=document.createElement(tag);node.textContent=text;if(className)node.className=className;return node;}
function status(message){$('status').textContent=message;}
function update(){
 $('compare').disabled=!ready||pending!==null||Object.values(docs).some(d=>d.reading);
 for(const d of Object.values(docs)){$(`${d.id}-bytes`).textContent=`${bytes(d.text).toLocaleString()} UTF-8 bytes`;$(`${d.id}-lf`).disabled=!d.text;$(`${d.id}-original`).disabled=!d.original;}
}
function clearReport(){report=null;$('download').disabled=true;$('result').textContent='No result.';$('result-state').textContent='No result';$('summary').replaceChildren(element('p','Compare the current pair to see structural evidence changes.','empty'));}
function abortExample(){exampleToken++;exampleController?.abort();exampleController=null;}
function cancelRead(d){d.readToken++;if(d.reading)$(`${d.id}-file-status`).textContent='Pending read canceled; current text and opened original retained.';d.reading=false;}
function changed(d,text,message){d.revision++;revision++;d.text=text;clearReport();if(pending)start();update();if(message)status(ready?message:`${message} ${engineMessage}`);}
function fail(message){clearTimeout(jobTimer);generation++;worker?.terminate();worker=null;ready=false;pending=null;clearReport();update();engineMessage=`${message} Restart engine to retry. Both inputs and opened originals are retained.`;status(engineMessage);}
function render(result){
 const container=$('summary');container.replaceChildren();
 if(result.adapter_error){$('result-state').textContent='Browser refusal';container.append(element('p','Browser resource admission declined this pair.','class-title'));container.append(element('p',result.adapter_error.code+': '+result.adapter_error.message));container.append(element('p',result.scope,'boundary'));return;}
 const names={success:'No structural differences',difference:'Structural differences',invalid:'Invalid evidence input',unsupported:'Unsupported evidence schema',incomplete:'Evidence input incomplete',blocked:'Comparison blocked by native bound',internal:'Native internal failure'};
 $('result-state').textContent=result.result_class;container.append(element('p',names[result.result_class]||result.result_class,'class-title'));
 container.append(element('p','This result does not establish compatibility, support, freshness, approval or readiness.','boundary'));
 for(const diagnostic of result.diagnostics||[]){const row=element('div','','diagnostic');row.append(element('strong',diagnostic.code),element('p',diagnostic.message));for(const action of diagnostic.next_actions||[])row.append(element('p',action));container.append(row);}
 const record=result.record;
 if(record){
  const refs=element('div','','references');for(const side of ['before','after']){const ref=record[side],box=element('div');box.append(element('h3',side==='before'?'Before declaration':'After declaration'));for(const [label,value] of [['Profile',ref.profile_id],['Revision',ref.revision],['Consumer',ref.consumer],['Content digest',ref.content_digest]]){const row=element('div','','identity');row.append(element('strong',label),element('span',value));box.append(row);}refs.append(box);}container.append(refs);
  for(const [label,values] of [['Changed sections',record.changed_sections],['Unchanged sections',record.unchanged_sections]]){container.append(element('h3',label));const list=element('div','','section-list');for(const value of values)list.append(element('span',value));container.append(list);}
  container.append(element('h3',`${record.changes.length.toLocaleString()} structural changes`));
  if(!record.changes.length)container.append(element('p','No change paths. Equal declared evidence is not approval.'));
  else {
   const wrapper=element('div','','changes'),table=document.createElement('table'),head=document.createElement('thead'),row=document.createElement('tr');for(const label of ['JSON Pointer','Kind','Before digest','After digest'])row.append(element('th',label));head.append(row);table.append(head);const body=document.createElement('tbody');table.append(body);wrapper.append(table);container.append(wrapper);let shown=0;
   const more=element('button','Show next 100 changes'),count=element('p','','fine');
   const append=()=>{const end=Math.min(shown+100,record.changes.length);for(;shown<end;shown++){const change=record.changes[shown],row=document.createElement('tr');for(const [label,value] of [['JSON Pointer',change.path],['Kind',change.change_kind],['Before digest',change.before_value_digest??'(none)'],['After digest',change.after_value_digest??'(none)']]){const cell=element('td',value);cell.dataset.label=label;row.append(cell);}body.append(row);}count.textContent=`Showing ${shown.toLocaleString()} of ${record.changes.length.toLocaleString()}; download includes every change.`;more.hidden=shown===record.changes.length;};more.onclick=append;append();container.append(count,more);
  }
  const limits=document.createElement('details');limits.append(element('summary','Native unknowns and limitations'));for(const text of [...record.unknowns,...record.limitations])limits.append(element('p',text));container.append(limits);
 }
 const identity=document.createElement('details');identity.append(element('summary','Canonical result identities'));for(const key of ['selection_identity','invocation_identity','result_identity'])identity.append(element('p',key+': '+result[key],'identity'));container.append(identity);
}
function start(){
 worker?.terminate();clearTimeout(jobTimer);generation++;ready=false;pending=null;clearReport();update();engineMessage='Starting Rust engine...';status(engineMessage);const own=generation,started=performance.now();
 try{worker=new Worker('./worker.js',{type:'module'});}catch{fail('Rust worker could not start.');return;}const active=worker;
 jobTimer=setTimeout(()=>{if(own===generation)fail('Engine startup exceeded 12 seconds.');},12000);
 active.onerror=active.onmessageerror=()=>{if(own===generation&&worker===active)fail('Rust engine failed.');};
 active.onmessage=({data})=>{
  if(own!==generation||worker!==active)return;
  if(data.type==='ready'&&!ready){clearTimeout(jobTimer);ready=true;update();$('status').dataset.startupMs=String(performance.now()-started);$('status').dataset.workerInitializationMs=String(data.initialization_ms);status('Ready. Compare your pair or load the example.');return;}
  if(!pending||data.id!==pending.id)return;clearTimeout(jobTimer);const request=pending;pending=null;update();if(request.revision!==revision)return;
  if(data.type==='error'){fail(data.message);return;}
  try{const result=JSON.parse(data.text);report=data.text;$('result').textContent=report;$('download').disabled=false;$('status').dataset.queryRoundtripMs=String(performance.now()-request.started);$('status').dataset.workerComputationMs=String(data.computation_ms);$('status').dataset.inputBytes=String(request.inputBytes);render(result);status(result.adapter_error?'Browser declined this pair; no partial comparison was produced.':'Comparison completed; no owner work executed.');}catch{fail('Rust engine returned an unreadable result.');}
 };
}
$('compare').onclick=()=>{
 if(!ready||pending||Object.values(docs).some(d=>d.reading))return;clearReport();
 for(const d of Object.values(docs))if(bytes(d.text)>MAX_BYTES){status(`${d.id==='before'?'Before':'After'} input exceeds 1 MiB UTF-8; nothing was truncated or compared.`);return;}
 const id=crypto.randomUUID();pending={id,revision,started:performance.now(),inputBytes:bytes(docs.before.text)+bytes(docs.after.text)};update();status('Comparing with Rust...');const own=generation;jobTimer=setTimeout(()=>{if(own===generation)fail('Comparison exceeded 10 seconds.');},10000);
 try{worker.postMessage({id,before:docs.before.text,after:docs.after.text});}catch{fail('Worker request could not be posted.');}
};
$('restart').onclick=start;
function download(data,name,type){const url=URL.createObjectURL(new Blob([data],{type})),a=document.createElement('a');a.href=url;a.download=name;a.click();setTimeout(()=>URL.revokeObjectURL(url),1000);}
async function openFile(d,file){
 abortExample();cancelRead(d);const token=d.readToken,ownRevision=d.revision;
 if(file.size>MAX_BYTES){$(`${d.id}-file-status`).textContent='File exceeds 1 MiB; previous input and original retained.';update();return;}
 d.reading=true;update();$(`${d.id}-file-status`).textContent=`Reading ${file.name}...`;let timer;
 try{const buffer=await Promise.race([file.arrayBuffer(),new Promise((_,reject)=>{timer=setTimeout(()=>reject(new Error('File read exceeded 5 seconds.')),5000);})]);if(token!==d.readToken||ownRevision!==d.revision)return;const raw=new Uint8Array(buffer);if(raw.length>MAX_BYTES)throw new Error('File exceeds 1 MiB.');const text=new TextDecoder('utf-8',{fatal:true,ignoreBOM:true}).decode(raw);d.original={raw,name:file.name};$(d.id).value=text;changed(d,text,'File loaded; compare the current pair.');$(`${d.id}-file-status`).textContent=`Opened ${file.name}: ${raw.length.toLocaleString()} original bytes retained.`;}
 catch(error){if(token===d.readToken&&ownRevision===d.revision)$(`${d.id}-file-status`).textContent=`${error.message} Previous input and original retained.`;}
 finally{clearTimeout(timer);if(token===d.readToken)d.reading=false;update();}
}
for(const d of Object.values(docs)){
 const input=$(d.id);input.oninput=()=>{abortExample();cancelRead(d);changed(d,input.value,'Input changed; previous comparison cleared.');};
 $(`${d.id}-open`).onclick=()=>$(`${d.id}-file`).click();$(`${d.id}-file`).onchange=event=>{const file=event.target.files[0];event.target.value='';if(file)openFile(d,file);};
 $(`${d.id}-lf`).onclick=()=>{abortExample();cancelRead(d);const text=d.text.replace(/^\uFEFF/,'').replace(/\r\n?/g,'\n');input.value=text;changed(d,text,'Explicit LF copy created; opened original remains downloadable.');};
 $(`${d.id}-original`).onclick=()=>{if(d.original)download(d.original.raw,d.original.name,'application/octet-stream');};$(`${d.id}-current`).onclick=()=>download(d.text,`ferris-${d.id}-current.json`,'text/plain;charset=utf-8');
 const insert=(text,event)=>{const next=input.value.slice(0,input.selectionStart)+text+input.value.slice(input.selectionEnd);if(bytes(next)>MAX_BYTES){event.preventDefault();$(`${d.id}-file-status`).textContent='Paste/drop exceeds 1 MiB UTF-8. Entire operation rejected; input retained.';return;}if(event.type==='drop'){event.preventDefault();input.setRangeText(text,input.selectionStart,input.selectionEnd,'end');input.dispatchEvent(new Event('input',{bubbles:true}));}};
 input.addEventListener('paste',event=>insert(event.clipboardData.getData('text/plain'),event));input.addEventListener('dragover',event=>event.preventDefault());input.addEventListener('drop',event=>{if(event.dataTransfer.files.length){event.preventDefault();openFile(d,event.dataTransfer.files[0]);}else insert(event.dataTransfer.getData('text/plain'),event);});
}
async function loadExample(){
 abortExample();for(const d of Object.values(docs))cancelRead(d);const token=exampleToken,ownRevision=revision,controller=new AbortController();exampleController=controller;const timer=setTimeout(()=>controller.abort(),5000);$('example-status').textContent='Loading public development example...';
 try{const pair=await Promise.all(['before','after'].map(async id=>{const response=await fetch('./'+id+'.json',{signal:controller.signal});if(!response.ok)throw new Error('Example unavailable.');const text=await response.text();if(bytes(text)>MAX_BYTES)throw new Error('Example exceeds input bound.');return [id,text];}));if(token!==exampleToken||ownRevision!==revision)return;for(const [id,text] of pair){$(id).value=text;changed(docs[id],text);}status(ready?'Example pair loaded; compare or edit it.':`Example pair loaded. ${engineMessage}`);$('example-status').textContent='Public development pair loaded. Opened originals remain separately downloadable.';}
 catch(error){if(token===exampleToken&&ownRevision===revision)$('example-status').textContent=`Example could not load (${error.message}). Inputs are retained.`;}
 finally{clearTimeout(timer);if(exampleController===controller)exampleController=null;update();}
}
$('example').onclick=loadExample;$('download').onclick=()=>{if(report)download(report,'ferris-profile-comparison.json','application/json');};
update();start();loadExample();
