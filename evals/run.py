"""With-skill vs without-skill agent runs.

Each case copies the eframe-project starter app into a temp folder, asks
`claude -p` for a change, and grades the result: `cargo check` must pass, and
patterns the skills warn about must be absent (or required ones present). Every
case runs with this repo loaded through --plugin-dir and without it.

    python evals/run.py [--case NAME] [--runs N] [--model MODEL]

Needs a logged-in claude CLI, cargo, and network. It spends API usage.
"""

import argparse
import json
import os
import re
import shutil
import subprocess
import tempfile
from pathlib import Path

root = Path(__file__).resolve().parent.parent
env = {**os.environ, "CARGO_TARGET_DIR": str(root / "evals" / "target")}

CASES = {
    "side-panel": {
        "prompt": "Add a resizable left side panel that lists five hardcoded file names as "
        "selectable labels, and show the selected name in the central panel.",
        "forbid": [r"\bSidePanel\b", r"\bTopBottomPanel\b", r"fn update\s*\("],
        "require": [r"Panel::left"],
    },
    "background-load": {
        "prompt": "Add a Load button that reads Cargo.toml on a background thread and shows "
        "the text when it is ready. The window must stay responsive while it loads.",
        "forbid": [r"\.recv\(\)", r"\.join\(\)", r"fn update\s*\("],
        "require": [r"request_repaint"],
    },
    "note-list": {
        "prompt": "Keep a list of notes, each with a u64 id, a title, and a body. Show each "
        "note as a collapsing header with a multiline editor for the body and a Delete "
        "button, plus an Add note button.",
        "forbid": [r"fn update\s*\("],
        "require": [r"push_id|id_salt"],
    },
    "glow": {
        "prompt": "Switch this app to the glow (OpenGL) renderer and remove wgpu from the "
        "build. It must still open windows with decorations on Linux under X11 and "
        "Wayland, and keep screen reader support.",
        "forbid": [r'"wgpu"'],
        "require": [
            r"default-features\s*=\s*false",
            r'"x11"',
            r'"wayland"',
            r'"accesskit"',
            r"wayland-csd-adwaita",
        ],
    },
}

TOOLS = ["Read", "Edit", "Write", "Glob", "Grep", "Skill", "Bash(cargo:*)"]


def starter(work: Path) -> None:
    (work / "src").mkdir()
    shutil.copy(root / "examples/manifests/crates/my_app/Cargo.toml", work / "Cargo.toml")
    shutil.copy(root / "examples/snippets/src/app.rs", work / "src/app.rs")
    main = (root / "examples/snippets/src/main.rs").read_text(encoding="utf-8")
    (work / "src/main.rs").write_text(main.split("\n// Everything above")[0] + "\n", encoding="utf-8")


def run_agent(work: Path, prompt: str, with_skills: bool, model: str | None) -> tuple[bool, dict]:
    cmd = [
        shutil.which("claude") or "claude", "-p", prompt,
        "--output-format", "stream-json", "--verbose", "--no-session-persistence",
        "--setting-sources", "project",  # no user plugins, hooks, or skills in either arm
        "--permission-mode", "acceptEdits", "--allowedTools", *TOOLS,
        "--max-budget-usd", "3",
    ]
    if with_skills:
        cmd += ["--plugin-dir", str(root)]
    if model:
        cmd += ["--model", model]
    out = subprocess.run(cmd, cwd=work, env=env, capture_output=True, text=True, encoding="utf-8", timeout=1800)
    skill_used, result = False, {}
    for line in out.stdout.splitlines():
        try:
            event = json.loads(line)
        except json.JSONDecodeError:
            continue
        if event.get("type") == "assistant":
            content = event["message"]["content"]
            skill_used |= any(c.get("type") == "tool_use" and c.get("name") == "Skill" for c in content)
        elif event.get("type") == "result":
            result = event
    return skill_used, result


def grade(work: Path, case: dict) -> list[str]:
    fails = []
    check = subprocess.run(
        ["cargo", "check", "--quiet", "--message-format=short"],
        cwd=work, env=env, capture_output=True, text=True, encoding="utf-8",
    )
    if check.returncode:
        fails.append("cargo check: " + next((l for l in check.stderr.splitlines() if "error" in l), "failed"))
    files = [work / "Cargo.toml", *work.glob("src/**/*.rs")]
    text = "\n".join(p.read_text(encoding="utf-8", errors="replace") for p in files)
    fails += [f"has `{p}`" for p in case["forbid"] if re.search(p, text)]
    fails += [f"lacks `{p}`" for p in case["require"] if not re.search(p, text)]
    return fails


parser = argparse.ArgumentParser()
parser.add_argument("--case", action="append", choices=CASES)
parser.add_argument("--runs", type=int, default=1)
parser.add_argument("--model")
args = parser.parse_args()

print("| Case | Skills | Pass | Skill tool used | Turns | Denied tools | Cost | Failures |")
print("|---|---|---|---|---|---|---|---|")
totals = {}
for name in args.case or CASES:
    for arm in ("with", "without"):
        for _ in range(args.runs):
            work = Path(tempfile.mkdtemp(prefix=f"egui-eval-{name}-"))
            try:
                starter(work)
                used, result = run_agent(work, CASES[name]["prompt"], arm == "with", args.model)
                fails = grade(work, CASES[name])
            finally:
                shutil.rmtree(work, ignore_errors=True)
            cost = result.get("total_cost_usd", 0.0)
            passed, runs, spent = totals.get(arm, (0, 0, 0.0))
            totals[arm] = (passed + (not fails), runs + 1, spent + cost)
            print(
                f"| {name} | {arm} | {'no' if fails else 'yes'} | {'yes' if used else 'no'} "
                f"| {result.get('num_turns', '?')} | {len(result.get('permission_denials', []))} "
                f"| ${cost:.2f} | {'; '.join(fails)} |",
                flush=True,
            )

for arm, (passed, runs, spent) in totals.items():
    print(f"\n{arm} skills: {passed}/{runs} passed, ${spent:.2f}")
