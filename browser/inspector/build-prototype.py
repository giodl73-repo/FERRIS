"""Local portability probe only; not a publication-ready staging command."""
import argparse,hashlib,json,os,pathlib,shutil,subprocess
P=pathlib.Path(__file__).resolve().parent;ROOT=P.parent.parent
parser=argparse.ArgumentParser();parser.add_argument('--bindgen',required=True);args=parser.parse_args()
for name in ['app.js','worker.js']:subprocess.run(['node','--check',str(P/'web'/name)],check=True)
target=pathlib.Path(os.environ['CARGO_TARGET_DIR']);assert target.is_absolute()
assert subprocess.check_output([args.bindgen,'--version']).decode().strip()=='wasm-bindgen 0.2.127'
env=dict(os.environ);env.pop('RUSTFLAGS',None)
cargo_home=pathlib.Path(os.environ.get('CARGO_HOME',str(pathlib.Path.home()/'.cargo'))).resolve()
env['CARGO_ENCODED_RUSTFLAGS']='\x1f'.join(['--remap-path-prefix='+str(ROOT)+'=/ferris','--remap-path-prefix='+str(cargo_home)+'=/cargo'])
for flags in [['--release','--target','wasm32-unknown-unknown','--lib'],['--bin','ferris-profile-inspector-cli']]:
 build_env=dict(env)
 if '--target' in flags:build_env['CARGO_ENCODED_RUSTFLAGS']+='\x1f-C\x1flink-arg=--max-memory=67108864'
 subprocess.run(['cargo','+1.95.0','build','--locked','--manifest-path',str(P/'Cargo.toml'),*flags],cwd=ROOT,env=build_env,check=True)
site=ROOT/'target/profile-prototype/FERRIS';site.mkdir(parents=True,exist_ok=True)
for name in ['index.html','style.css','app.js','worker.js','before.json','after.json']:shutil.copyfile(P/'web'/name,site/name)
subprocess.run([args.bindgen,'--target','web','--out-dir',str(site/'pkg'),'--out-name','ferris_profile_inspector',str(target/'wasm32-unknown-unknown/release/ferris_profile_inspector.wasm')],check=True)
family_root=ROOT/'tests/fixtures/profile-evidence'
def load(family,name):return (family_root/family/(name+'.json')).read_text(encoding='utf-8')
base=load('pure-data','before');after=load('pure-data','after');pairs=[(base,after)]
for directory in sorted(family_root.iterdir()):
 if directory.is_dir():
  before=load(directory.name,'before');after=load(directory.name,'after');pairs.extend([(before,before),(before,after),(after,before)])
for bad in ['', '{', '{}','null','{"x":1,"x":2}','\ufeff'+base,' '*1048577]:pairs.extend([(bad,base),(base,bad)])
for field,value in [('schema','unsupported/v9'),('consumer','other-consumer'),('profile_id','other-profile'),('revision','x'*257),('revision','non-ascii-\u00e9')]:
 v=json.loads(base);v[field]=value;pairs.append((base,json.dumps(v)))
for count in [10000,10001]:
 before=json.loads(base);after=json.loads(base);before['sections']['features']=[];after['sections']['features']=[0]*count;pairs.append((json.dumps(before),json.dumps(after)))
cli=target/'debug'/('ferris-profile-inspector-cli.exe' if os.name=='nt' else 'ferris-profile-inspector-cli')
records=[]
for before,after in pairs:
 request=json.dumps({'before':before,'after':after}).encode('utf-8')
 response=subprocess.run([str(cli)],input=request,stdout=subprocess.PIPE,stderr=subprocess.PIPE,check=True)
 records.append({'before':before,'after':after,'output':response.stdout.decode('utf-8').rstrip('\r\n')})
cache=P/'.cache';cache.mkdir(exist_ok=True);(cache/'native-cases.json').write_text(json.dumps(records)+'\n',encoding='utf-8')
wasm=site/'pkg/ferris_profile_inspector_bg.wasm'
receipt={'prototype_only':True,'native_cases':len(records),'development_families':9,'wasm_bytes':wasm.stat().st_size,'wasm_sha256':hashlib.sha256(wasm.read_bytes()).hexdigest(),'toolchain':'Rust1.95.0; wasm-bindgen0.2.127','publication_qualified':False}
(cache/'prototype-build.json').write_text(json.dumps(receipt,indent=2)+'\n',encoding='utf-8');print(json.dumps(receipt))
