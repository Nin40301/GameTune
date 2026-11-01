"""Refresh reviewed catalog hashes. Does not mark items verified or grant targets."""
import hashlib
import json
from pathlib import Path

catalog = Path(__file__).resolve().parent.parent / "catalog"
hashes = {
    name: hashlib.sha256((catalog / name).read_bytes()).hexdigest()
    for name in ("items.json", "schema.json", "profile.schema.json")
}
(catalog / "sha256.json").write_text(
    json.dumps(hashes, indent=2) + "\n", encoding="utf-8", newline="\n"
)
