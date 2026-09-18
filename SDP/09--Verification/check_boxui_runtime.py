#!/usr/bin/env python3
"""Cross-check real Rust protocol output against frozen draft1 JSON schemas.

Prerequisites: cargo build --locked --no-default-features --example boxui_host
              cargo run --locked --no-default-features --example boxui_demo
"""
import json
from pathlib import Path
import subprocess
from jsonschema import Draft202012Validator

ROOT = Path(__file__).resolve().parents[2]
SDP = ROOT / "SDP"
FIXTURES = SDP / "09--Verification/fixtures"
CONTRACTS = SDP / "06--Container-Design/contracts"
HOST = ROOT / "target/debug/examples/boxui_host"


def load(path):
    return json.loads(path.read_text())


def check(name, value):
    Draft202012Validator(load(CONTRACTS / ("boxui-" + name + ".schema.json"))).validate(value)


def invoke(mode, data, status=0):
    result = subprocess.run([str(HOST), mode], input=data, capture_output=True, timeout=30)
    assert result.returncode == status, (result.returncode, result.stdout, result.stderr)
    value = json.loads(result.stdout)
    assert value["contract"] == "BX-HOST/0.1-draft1"
    return value


def main():
    assert HOST.exists(), "Build the boxui_host example first (see module docstring)"
    source = b"boxui 0.1\n" + (FIXTURES / "activity.boxui.json").read_bytes()
    parsed = invoke("parse", source)
    check("model", parsed["model"])
    assert parsed["model"] == load(FIXTURES / "model.json")
    for child in parsed["childSources"]:
        assert json.loads(source[child["sourceStart"]:child["sourceEnd"]]) == child["source"]

    frame = invoke("prepare", (FIXTURES / "prepare.json").read_bytes())
    check("frame", frame)
    assert frame["key"] == load(FIXTURES / "prepare.json")["key"]
    assert [c["id"] for c in frame["controls"]] == ["context-input", "pause", "continue"]
    assert frame["staticSvg"] != frame["previewSvg"]

    generated = ROOT / "target/boxui-demo"
    real = load(generated / "frame.json")
    request = load(generated / "prepare.json")
    check("prepare", request)
    check("model", request["model"])
    check("frame", real)
    assert real["diagnostics"] == []
    assert real == invoke("prepare", (generated / "prepare.json").read_bytes())

    # Exercise errors across the actual stdin/stdout envelope boundary too.
    bad = invoke("parse", source.replace(b"boxui 0.1", b"boxui 9.9"), 2)
    assert "model" not in bad and bad["diagnostics"][0]["severity"] == "error"
    bad = invoke("prepare", b'{"contract":"a","contract":"b"}', 1)
    assert bad["diagnostics"][0]["code"] == "invalid-json"
    bad = invoke("parse", b"x" * (256 * 1024 + 1), 3)
    assert bad["diagnostics"][0]["code"] == "source-budget"
    print("PASS: real parse/model/prepare/frame envelopes conform to draft1; byte ranges, keys, controls, deterministic replay and 3 protocol failures checked.")
    print("Approximate metrics; native XFMD input/publication/PDF are not exercised.")


if __name__ == "__main__":
    main()
