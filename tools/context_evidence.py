"""Run the Rust walkthrough and keep its README diagram tied to real output."""
import argparse
from datetime import datetime, timezone
import hashlib
from html import escape
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
REPORT = ROOT / "docs/evidence/context-walkthrough.json"
SVG = ROOT / "docs/assets/context-walkthrough.svg"


def source_hashes():
    return {name: hashlib.sha256((ROOT / name).read_bytes().replace(b"\r\n", b"\n")).hexdigest()
            for name in ("src/lib.rs", "examples/context_walkthrough.rs")}


def render(data):
    cards = []
    for index, row in enumerate(data["result"]["rows"]):
        x = 32 + index * 248
        ready = row["check"] == "Ready"
        color = "#087c79" if ready else "#9a461b"
        background = "#eaf6f3" if ready else "#fff1e9"
        cards.append(f'''
<rect x="{x}" y="142" width="232" height="202" rx="14" fill="{background}" stroke="#dce3ec"/>
<text x="{x+20}" y="173" font-size="18" fill="#172438" font-weight="700">{index+1}. {escape(row['action'])}</text>
<text x="{x+20}" y="208" font-size="15" fill="#526176">LIVE VALUE</text>
<text x="{x+20}" y="249" font-size="36" fill="#172438" font-weight="700">{row['live']}</text>
<text x="{x+20}" y="279" font-size="16" fill="#526176">Revision {row['revision']} · preview {row['preview']}</text>
<text x="{x+20}" y="318" font-size="17" fill="{color}" font-weight="700">{row['check']}</text>''')
    return f'''<svg xmlns="http://www.w3.org/2000/svg" width="1040" height="454" viewBox="0 0 1040 454" role="img" aria-labelledby="title description">
<title id="title">Undo restores a value, not the validity of an old draft</title>
<desc id="description">The executable Rust example opens a draft at value 100, revision 42, with preview 75. A host edit changes the value to 120 at revision 43. Undo restores 100 at revision 44. The value matches the original again, but edit-session rejects the stale context. Reset restores preview 100 without rebasing the captured revision. Finish returns ContextChanged.</desc>
<rect width="1040" height="454" rx="20" fill="#f4f7fb"/>
<path d="M20 0H1020Q1040 0 1040 20V116H0V20Q0 0 20 0Z" fill="#152238"/>
<g font-family="Arial, Helvetica, sans-serif">
<text x="32" y="34" font-size="14" fill="#c4cede" font-weight="700">EDIT-SESSION / RUNNABLE CONTEXT WALKTHROUGH</text>
<text x="32" y="75" font-size="30" fill="#fff" font-weight="700">Undo restores the value. The old draft stays stale.</text>
<text x="32" y="101" font-size="16" fill="#dae3f1">The host supplies a revision that changes on edits, Undo and Redo.</text>
{''.join(cards)}
<text x="32" y="379" font-size="19" fill="#172438" font-weight="700">Same value: 100. Different revision: 42 → 44. finish() returns ContextChanged.</text>
<text x="32" y="409" font-size="16" fill="#526176">The host must not apply the stale draft. Reset is not a rebase, a rollback or a freshness override.</text>
<text x="32" y="435" font-size="14" fill="#526176">Example output · verified {data['recorded_date']} · no GUI, automatic locking or upstream adoption claim.</text>
</g></svg>
'''


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, default=ROOT / "target/debug/examples/context_walkthrough")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    completed = subprocess.run([str(args.binary.resolve()), "--json"], text=True,
                               capture_output=True, timeout=15, check=True)
    result = json.loads(completed.stdout)
    if args.check:
        data = json.loads(REPORT.read_text(encoding="utf-8"))
        if data["result"] != result or data["source_files"] != source_hashes():
            raise SystemExit("Walkthrough changed; run this script without --check")
        if SVG.read_text(encoding="utf-8") != render(data):
            raise SystemExit("Diagram changed; run this script without --check")
    else:
        data = {"recorded_date": datetime.now(timezone.utc).date().isoformat(),
                "scope": "Original synchronous teaching model; actual edit-session API, no GUI or upstream test claim",
                "source_hash_format": "SHA-256 of UTF-8 text with LF line endings",
                "source_files": source_hashes(), "result": result}
        REPORT.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
        SVG.write_text(render(data), encoding="utf-8")
    print("Context walkthrough output, source hashes and diagram verified")


if __name__ == "__main__":
    main()
