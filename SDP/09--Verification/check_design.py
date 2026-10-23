#!/usr/bin/env python3
"""Scoped authoring checks. No renderer/host execution or SDL language extension."""
import copy
import hashlib
import json
from pathlib import Path
import re
from urllib.parse import unquote
from jsonschema import Draft202012Validator, ValidationError

ROOT = Path(__file__).resolve().parents[1]
CONTRACTS = ROOT / '06--Container-Design/contracts'
FIXTURES = ROOT / '09--Verification/fixtures'

def pairs(items):
    out = {}
    for key, value in items:
        if key in out:
            raise ValueError('duplicate JSON key: ' + key)
        out[key] = value
    return out

def load(path):
    return json.loads(path.read_text(), object_pairs_hook=pairs)

def require(condition, message):
    if not condition:
        raise ValueError(message)

def confined(root, relative):
    p = (root / relative).resolve()
    require(p.is_relative_to(root.resolve()) and p.exists(), 'Missing/escaping path: ' + relative)
    return p

def source_semantics(model):
    require(len(json.dumps(model, ensure_ascii=False).encode()) <= 262144, 'source budget')
    bindings = {b['id']: b for b in model['bindings']}
    require(len(bindings) == len(model['bindings']), 'duplicate binding')
    nodes, diagrams = set(), []
    def visit(n, depth):
        require(depth <= 16, 'depth budget')
        require(n['id'] not in nodes, 'duplicate node')
        nodes.add(n['id'])
        size = n.get('size', {})
        require(size.get('min', 0) <= size.get('max', 8192), 'invalid size')
        for field, role in [('valueBinding', 'value'), ('commandBinding', 'command')]:
            if field in n:
                b = bindings.get(n[field])
                require(b is not None and b['role'] == role, 'binding role/reference')
                if n['kind'] == 'input':
                    require(b['type'] == 'string', 'input type')
                if n['kind'] == 'button':
                    require(b['type'] == 'none', 'button argument type')
        if n['kind'] == 'diagram':
            diagrams.append(n)
            require(n['source'].splitlines()[0].split()[0] == n['family'], 'child family')
            require(len(n['source'].encode()) <= 65536, 'child budget')
        for c in n.get('children', []):
            visit(c, depth + 1)
    visit(model['root'], 1)
    require(model['root']['kind'] in ['row', 'column'], 'root region')
    require(len(nodes) <= 256 and len(diagrams) <= 8, 'node/child budget')
    for b in bindings.values():
        require(not (b['role'] == 'value' and b['type'] == 'none'), 'invalid value type')

def negative(check, value, title):
    try:
        check(value)
    except (ValueError, ValidationError) as e:
        print('REJECT:', title)
        return
    raise ValueError('Negative unexpectedly accepted: ' + title)

def main():
    manifest = load(ROOT/'sdp-project.json')
    schema = load(ROOT/'00--Project/sdp-project.schema.json')
    Draft202012Validator.check_schema(schema)
    Draft202012Validator(schema).validate(manifest)
    ids = set()
    for group in ['documents', 'modules', 'sourceSets', 'targets', 'blueprints']:
        for entry in manifest[group]:
            require(entry['id'] not in ids, 'duplicate registry id')
            ids.add(entry['id'])
    docs = {d['id']: d for d in manifest['documents']}
    for d in docs.values():
        text = confined(ROOT, d['path']).read_text()
        require(f"document_id: {d['id']}\n" in text, 'document metadata')
        require(f"status: {d['status']}\n" in text, 'document status')
    for m in manifest['modules']:
        require(m['document'] in docs, 'module document')
        for b in m['bindings']:
            confined(ROOT.parent, b['path'])
    for ss in manifest['sourceSets']:
        for path in ss['members']:
            confined(ROOT, path)
        for e in ss['entryPoints']:
            require(e['path'] in ss['members'], 'entry membership')
    targets = {t['id']: t for t in manifest['targets']}
    def walk(id, chain):
        require(id in targets and id not in chain, 'target cycle/reference')
        for dep in targets[id]['requires']:
            walk(dep, chain + [id])
    for t in targets.values():
        require(t['module'] in ids, 'target module')
        walk(t['id'], [])
    for rel in manifest['relations']:
        require(rel['from'] in ids and rel['to'] in ids, 'relation reference')
    for b in manifest['blueprints']:
        data = load(confined(ROOT, b['path']))
        Draft202012Validator({'$defs':schema['$defs'], '$ref':'#/$defs/blueprint'}).validate(data)
        require(data['document'] in docs and data['target'] in targets, 'blueprint reference')
        require(data['output'] in targets[data['target']]['outputs'], 'blueprint output')
        for id in data['sources']['documents'] + data['sources']['sourceSets']:
            require(id in ids, 'blueprint source')
    links = 0
    for p in ROOT.rglob('*.md'):
        for url in re.findall(r'\]\(([^)]+)\)', p.read_text()):
            if '://' in url or url.startswith('#'):
                continue
            target = unquote(url.split('#')[0])
            require((p.parent/target).exists(), f'broken link {p}: {url}')
            links += 1
    schemas = {}
    for name in ['source','model','frame','intent','prepare','result']:
        s = load(CONTRACTS/f'boxui-{name}.schema.json')
        Draft202012Validator.check_schema(s)
        schemas[name] = Draft202012Validator(s)
    fixture_names = {'source':'activity.boxui.json', 'model':'model.json', 'frame':'frame.mock.json',
                     'intent':'intent.json', 'prepare':'prepare.json', 'result':'result.json'}
    fixtures = {key:load(FIXTURES/name) for key,name in fixture_names.items()}
    for name,data in fixtures.items():
        schemas[name].validate(data)
    require(fixtures['frame']['key']==fixtures['prepare']['key'], 'frame/request key')
    require(fixtures['frame']['width']==fixtures['prepare']['viewport']['width'] and
            fixtures['frame']['height']==fixtures['prepare']['viewport']['height'], 'frame viewport')
    for c in fixtures['frame']['controls']:
        a,b=c['rect'],c['clip']
        require(a['x']>=b['x'] and a['y']>=b['y'] and
                a['x']+a['width']<=b['x']+b['width'] and
                a['y']+a['height']<=b['y']+b['height'], 'fixture control clipping')
    schemas['intent'].validate(load(FIXTURES/'intent-commit.json'))
    source_semantics(fixtures['source'])
    schemas['model'].validate(fixtures['prepare']['model'])
    require(fixtures['prepare']['model']==fixtures['model'], 'typed model drift')
    require(fixtures['prepare']['children'][0]['ref']==fixtures['model']['root']['children'][-1]['childRef'], 'child ref')
    md=(FIXTURES/'activity.md').read_text().split('```boxui\nboxui 0.1\n',1)[1].split('\n```',1)[0]
    require(json.loads(md)==fixtures['source'], 'Markdown source drift')
    for counter,value in fixtures['frame']['key'].items():
        if counter not in ['session','blockId']:
            require(int(value) < 2**64, 'uint64 range')
    bad=copy.deepcopy(fixtures['source']);bad['unexpected']=True
    negative(schemas['source'].validate,bad,'unknown field')
    bad=copy.deepcopy(fixtures['source']);bad['root']['children'][0]['kind']='html'
    negative(schemas['source'].validate,bad,'unknown widget')
    bad=copy.deepcopy(fixtures['source']);bad['profile']='boxui/99'
    negative(schemas['source'].validate,bad,'unknown profile')
    bad=copy.deepcopy(fixtures['source']);bad['root']['children'][0]['id']='panel'
    negative(source_semantics,bad,'duplicate node')
    bad=copy.deepcopy(fixtures['source']);bad['root']['children'][2]['commandBinding']='measurement'
    negative(source_semantics,bad,'wrong binding role')
    bad=copy.deepcopy(fixtures['source']);bad['root']['size']={'min':40,'max':20}
    negative(source_semantics,bad,'reversed sizing')
    bad=copy.deepcopy(fixtures['intent']);bad['action']='commit'
    negative(schemas['intent'].validate,bad,'commit without value/revision')
    bad=copy.deepcopy(fixtures['frame']);bad['width']=-1
    negative(schemas['frame'].validate,bad,'negative frame extent')
    bad=copy.deepcopy(fixtures['frame']);del bad['key']['bindingRevision']
    negative(schemas['frame'].validate,bad,'missing binding revision')
    negative(lambda s:json.loads(s,object_pairs_hook=pairs),'{"id":1,"id":2}','duplicate JSON key')
    print(f'PASS: {len(docs)} documents, {len(manifest["sourceSets"])} SDL source set, {len(targets)} targets, {links} local links, 6 schemas/fixtures, 10 negative checks.')
    print('No product parser, layout, input, SVG safety, PDF or SDL behavioral execution tested.')
    print('Manifest SHA256:', hashlib.sha256((ROOT/'sdp-project.json').read_bytes()).hexdigest())
if __name__=='__main__':
    main()
