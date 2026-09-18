#!/usr/bin/env python3
"""Verify the vendored, pinned libavoid sources without network access."""
import hashlib
import json
from pathlib import Path
root = Path(__file__).resolve().parents[1] / 'vendor'
lock = json.loads((root / 'libavoid.lock.json').read_text())
for name, expected in lock['files'].items():
    assert hashlib.sha256((root / name).read_bytes()).hexdigest() == expected, name
actual = {str(p.relative_to(root)) for p in (root / 'libavoid').iterdir() if p.is_file()}
assert actual == set(lock['files']), 'unexpected/missing vendored files'
print(f"Verified {len(actual)} files at {lock['commit']}")
