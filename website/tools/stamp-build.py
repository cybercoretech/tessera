#!/usr/bin/env python3
"""Writes website/dist/build.json: facts about the build the site footer shows.

Run it after `tessera build`, before deploying:

    cargo run --manifest-path ../Cargo.toml -- build && python3 tools/stamp-build.py

It records the Tessera version (from the workspace Cargo.toml), the git commit
the site was built from (with a -dirty suffix if the tree had changes), and the
number of files, pages and bytes in dist/. build.json itself is not counted.
"""
import json
import subprocess
import sys
import tomllib
from datetime import datetime, timezone
from pathlib import Path

WEBSITE = Path(__file__).resolve().parent.parent
REPO = WEBSITE.parent
DIST = WEBSITE / "dist"
OUT = DIST / "build.json"


def git(*args: str) -> str:
    return subprocess.run(["git", "-C", str(REPO), *args], capture_output=True, text=True, check=True).stdout.strip()


def main() -> int:
    if not (DIST / "index.html").is_file():
        print(f"error: {DIST}/index.html not found; run `tessera build` first", file=sys.stderr)
        return 1

    with open(REPO / "Cargo.toml", "rb") as fh:
        version = tomllib.load(fh)["package"]["version"]

    commit = git("rev-parse", "--short", "HEAD")
    dirty = bool(git("status", "--porcelain", "--", "website", "src", "crates", "Cargo.toml"))

    files = [p for p in DIST.rglob("*") if p.is_file() and p != OUT]
    stats = {
        "tessera": version,
        "commit": commit + ("-dirty" if dirty else ""),
        "built": datetime.now(timezone.utc).strftime("%Y-%m-%d"),
        "files": len(files),
        "pages": sum(1 for p in files if p.suffix == ".html"),
        "bytes": sum(p.stat().st_size for p in files),
    }
    OUT.write_text(json.dumps(stats, indent=2) + "\n")
    print(f"{OUT.relative_to(WEBSITE)}: {stats}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
