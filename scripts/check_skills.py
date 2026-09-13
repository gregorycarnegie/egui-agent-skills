"""Checks skill frontmatter, README rows, Rust snippets, and plugin versions.

Run from anywhere: python scripts/check_skills.py (needs PyYAML).
"""

import json
import re
import sys
from pathlib import Path

import yaml

root = Path(__file__).resolve().parent.parent
errors = []
readme = (root / "README.md").read_text(encoding="utf-8")
skills = sorted((root / "skills").glob("*/SKILL.md"))


def squash(code: str) -> str:
    """Drop blank lines and indentation, so a block matches wherever it is nested."""
    return "\n".join(line.strip() for line in code.splitlines() if line.strip())


def corpus(pattern: str) -> str:
    files = sorted(p for p in root.glob(pattern) if "target" not in p.parts)
    return "\n" + "\n".join(squash(p.read_text(encoding="utf-8")) for p in files) + "\n"


# Where each fenced block language must be copied, so CI builds or resolves it.
copies = {
    "rust": ("examples/snippets/src", corpus("examples/snippets/src/*.rs")),
    "toml": ("a Cargo.toml under examples/manifests", corpus("examples/manifests/**/Cargo.toml")),
}

for path in skills:
    rel = path.relative_to(root).as_posix()
    folder = path.parent.name
    text = path.read_text(encoding="utf-8")
    match = re.match(r"---\r?\n(.*?)\r?\n---\r?\n", text, re.DOTALL)
    if not match:
        errors.append(f"{rel}: missing frontmatter")
        continue
    try:
        meta = yaml.safe_load(match.group(1))
    except yaml.YAMLError as e:
        errors.append(f"{rel}: invalid YAML: {e}")
        continue
    if not isinstance(meta, dict):
        errors.append(f"{rel}: frontmatter is not a mapping")
        continue

    name = meta.get("name") or ""
    if name != folder:
        errors.append(f"{rel}: name {name!r} does not match folder {folder!r}")
    elif not re.fullmatch(r"[a-z0-9-]{1,64}", name):
        errors.append(f"{rel}: name must be lowercase letters, digits, hyphens, max 64")
    description = meta.get("description")
    if not isinstance(description, str) or not description.strip():
        errors.append(f"{rel}: missing description")
    elif len(description) > 1024:
        errors.append(f"{rel}: description is {len(description)} chars, max 1024")
    if not meta.get("license"):
        errors.append(f"{rel}: missing license")
    if text.count("\n") > 500:
        errors.append(f"{rel}: over 500 lines")
    if f"| `{folder}` |" not in readme:
        errors.append(f"{rel}: no row in the README skills table")
    for block in re.finditer(r"^[ \t]*```(rust|toml)\r?\n(.*?)^[ \t]*```", text, re.DOTALL | re.MULTILINE):
        kind, code = block.groups()
        where, src = copies[kind]
        if f"\n{squash(code)}\n" not in src:
            line = text.count("\n", 0, block.start()) + 1
            errors.append(f"{rel}:{line}: {kind} block is not copied into {where}")


def load(name: str):
    return json.loads((root / name).read_text(encoding="utf-8"))


marketplace = load(".claude-plugin/marketplace.json")
versions = {
    ".claude-plugin/plugin.json": load(".claude-plugin/plugin.json")["version"],
    "marketplace.json metadata": marketplace["metadata"]["version"],
    **{f"marketplace.json plugin {p['name']}": p["version"] for p in marketplace["plugins"]},
    "gemini-extension.json": load("gemini-extension.json")["version"],
}
if len(set(versions.values())) != 1:
    errors.append("plugin versions differ: " + ", ".join(f"{k}={v}" for k, v in versions.items()))

if not skills:
    errors.append("no skills found under skills/")

for error in errors:
    print(f"error: {error}", file=sys.stderr)
if errors:
    sys.exit(1)
print(f"ok: {len(skills)} skills, version {next(iter(versions.values()))}")
