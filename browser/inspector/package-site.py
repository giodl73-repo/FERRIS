"""Stage only a clean-source FERRIS comparison site. Never deploys."""
import hashlib,html,json,os,pathlib,shutil,subprocess,zipfile
P=pathlib.Path(__file__).resolve().parent;ROOT=P.parent.parent
SITE=ROOT/'target/profile-release/FERRIS';PROTOTYPE=ROOT/'target/profile-prototype/FERRIS'
def run(*args):return subprocess.check_output(args,cwd=ROOT)
def digest(data):return hashlib.sha256(data).hexdigest()
def write_json(path,value):path.write_text(json.dumps(value,indent=2)+'\n',encoding='utf-8')
def copy(source,destination):
 assert not source.is_symlink(),source
 destination.parent.mkdir(parents=True,exist_ok=True);destination.write_bytes(source.read_bytes())
if run('git','status','--porcelain').strip():raise SystemExit('Release staging requires a clean checkout.')
head=run('git','rev-parse','HEAD').decode().strip()
run('git','diff','--exit-code','e9306126b412ad5359fed3246423761ee3615969','--','Cargo.toml','Cargo.lock','crates/ferris-cli','tests/fixtures','docs/specs')
assert SITE.resolve()==ROOT.resolve()/'target/profile-release/FERRIS'
if SITE.exists():shutil.rmtree(SITE)
SITE.mkdir(parents=True)
runtime=[]
for name in ['index.html','style.css','app.js','worker.js','before.json','after.json']:
 source=PROTOTYPE/name;relative='browser/inspector/web/'+name
 assert source.read_bytes()==run('git','show',head+':'+relative),f'Stale/non-Git runtime source: {name}'
 copy(source,SITE/name);runtime.append(name)
for name in ['ferris_profile_inspector.js','ferris_profile_inspector_bg.wasm']:
 copy(PROTOTYPE/'pkg'/name,SITE/'pkg'/name);runtime.append('pkg/'+name)
for name in ['LICENSE-MIT','LICENSE-APACHE']:copy(ROOT/name,SITE/name)
# Git archive preserves the complete first-party tree, including all test includes.
# Avoid thousands of Git child processes for this 131 MB uncompressed repository.
run('git','archive','--format=zip','--prefix=FERRIS-source/','--output',str(SITE/'source.zip'),head)
source_pins=[]
with zipfile.ZipFile(SITE/'source.zip') as archive:
 assert archive.comment.decode()==head
 for name in sorted(archive.namelist()):
  if name.endswith('/'):continue
  relative=pathlib.PurePosixPath(name);assert relative.parts[0]=='FERRIS-source' and '..' not in relative.parts
  data=archive.read(name);source_pins.append({'path':'/'.join(relative.parts[1:]),'sha256':digest(data),'bytes':len(data)})
tracked=set(filter(None,run('git','ls-files','-z').decode().split('\0')))
assert {r['path'] for r in source_pins}==tracked,'Source ZIP does not cover exact tracked files'
write_json(SITE/'source-files.json',source_pins)
metadata=json.loads(run('cargo','+1.95.0','metadata','--manifest-path','browser/inspector/Cargo.toml','--locked','--format-version','1','--filter-platform','wasm32-unknown-unknown'))
packages=[]
for package in metadata['packages']:
    folder=pathlib.Path(package['manifest_path']).parent
    notice_root=folder
    if package['name'] in ['ferris-core','ferris-profile-inspector']: notice_root=ROOT
    elif package.get('source','') and package['source'].startswith('git+'):
        for ancestor in [folder,*folder.parents[:4]]:
            if (ancestor/'LICENSE').is_file(): notice_root=ancestor;break
    notices=[f for f in notice_root.iterdir() if f.is_file() and f.name.upper().startswith(('LICENSE','COPYRIGHT','NOTICE','UNLICENSE'))]
    if not notices: raise SystemExit('No license notices for '+package['name'])
    records=[]
    for notice in sorted(notices):
        destination=SITE/'licenses/dependencies'/(package['name']+'-'+package['version'])/notice.name
        copy(notice,destination)
        records.append({'path':destination.relative_to(SITE).as_posix(),'sha256':digest(notice.read_bytes())})
    license_expression=package['license']
    packages.append({'name':package['name'],'version':package['version'],'source':package.get('source') or 'first-party FERRIS source.zip','license':license_expression,'notices':records})
write_json(SITE/'dependencies.json',packages)
sysroot=pathlib.Path(run('rustc','+1.95.0','--print','sysroot').decode().strip())
rust_docs=sysroot/'share/doc/rust'
copy(rust_docs/'COPYRIGHT-library.html',SITE/'rust/COPYRIGHT-library.html')
rust_notices=[]
for notice in sorted((rust_docs/'licenses').rglob('*')):
    if notice.is_file():
        destination=SITE/'rust/licenses'/notice.relative_to(rust_docs/'licenses')
        copy(notice,destination)
        rust_notices.append({'path':destination.relative_to(SITE).as_posix(),'sha256':digest(notice.read_bytes())})
write_json(SITE/'rust/provenance.json',{'rustc':run('rustc','+1.95.0','-vV').decode().strip(),'library_copyright_sha256':digest((SITE/'rust/COPYRIGHT-library.html').read_bytes()),'notices':rust_notices})
links=['<h1>FERRIS software and dependency licenses</h1><p>FERRIS retains its existing Apache-2.0 OR MIT software grants. Original dependency and Rust library notices remain under their own terms; prior grants are preserved.</p><p><a href="../LICENSE-MIT">FERRIS MIT</a> | <a href="../LICENSE-APACHE">FERRIS Apache</a> | <a href="../dependencies.json">Dependency inventory</a> | <a href="../rust/COPYRIGHT-library.html">Rust library copyright</a> | <a href="../rust/provenance.json">Rust notices</a></p>']
for package in packages:
 links.append('<h2>'+html.escape(package['name']+' '+package['version'])+'</h2><p>'+html.escape(package['license'] or 'See original notices')+'</p>')
 for notice in package['notices']:
  path=notice['path'].removeprefix('licenses/');links.append('<p><a href="'+html.escape(path)+'">'+html.escape(path)+'</a></p>')
(SITE/'licenses/index.html').write_text('<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>FERRIS licenses</title><body>'+''.join(links)+'</body></html>',encoding='utf-8')
copy(P/'BUILD.md',SITE/'BUILD.md')
provenance={'schema':'ferris-profile-browser-release/v1','commit':head,'native_entry':'e9306126b412ad5359fed3246423761ee3615969','repository':'https://github.com/giodl73-repo/FERRIS','exact_full_source':'https://github.com/giodl73-repo/FERRIS/archive/'+head+'.zip','rustc':run('rustc','+1.95.0','-vV').decode().strip(),'wasm_bindgen':'0.2.127','source_zip_sha256':digest((SITE/'source.zip').read_bytes()),'wasm_sha256':digest((SITE/'pkg/ferris_profile_inspector_bg.wasm').read_bytes()),'source_files':len(source_pins),'legal_packages':len(packages),'rust_notice_files':len(rust_notices),'native_limits':{'input_bytes_each':1048576,'changes':10000},'browser_limits':{'containers':24,'combined_leaf_path_bytes':2097152,'report_bytes':4194304,'wasm_linear_memory_bytes':67108864},'delivery_budgets':{'initial_runtime_bytes':2097152,'optional_source_zip_bytes':20971520,'complete_site_bytes':26214400},'runtime_files':runtime,'cross_directory_binary_identity':'Not established; independently rebuild and require exact native/WASM responses.','scope':'Experimental evidence comparison only. No profile generation, owner execution, support/compatibility/freshness/approval/readiness or held-out diagnostic conclusion.'}
write_json(SITE/'provenance.json',provenance)
files=[]
for f in sorted(SITE.rglob('*')):
 if f.is_file():files.append({'path':f.relative_to(SITE).as_posix(),'bytes':f.stat().st_size,'sha256':digest(f.read_bytes())})
runtime_bytes=sum((SITE/name).stat().st_size for name in runtime)
assert runtime_bytes<=2097152 and (SITE/'source.zip').stat().st_size<=20971520
assert sum(r['bytes'] for r in files)<=26214400
write_json(SITE/'inventory.json',{'commit':head,'runtime_bytes':runtime_bytes,'files':files,'bytes_excluding_inventory':sum(r['bytes'] for r in files)})
print(json.dumps({'commit':head,'files':len(files)+1,'runtime_bytes':runtime_bytes,'source_zip_bytes':(SITE/'source.zip').stat().st_size,'total_bytes_excluding_inventory':sum(r['bytes'] for r in files),'source_files':len(source_pins),'legal_packages':len(packages),'rust_notices':len(rust_notices)}))
