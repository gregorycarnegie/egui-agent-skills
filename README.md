# egui Agent Skills

Agent skills for building Rust GUI apps with [egui](https://github.com/emilk/egui)
and [eframe](https://github.com/emilk/egui/tree/main/crates/eframe). They work with
AI coding tools such as Claude Code, Codex CLI, GitHub Copilot, and Google Antigravity.

The skills target **egui 0.36**. That release removed or renamed many APIs that
models still suggest, such as `App::update`, `SidePanel`, and `Frame::none()`. When
a project uses another egui version, the skills check API names against that
version's crate source before using them.

> These skills use AI and can make mistakes. Always check the output.

## Skills

| Skill | Type | Description |
|-------|------|-------------|
| `egui` | Conceptual | Rules for writing and fixing egui/eframe code: the immediate-mode model, IDs, repaints, threads, persistence, and a table of APIs changed in 0.36. |
| `egui-review` | Review | Runs `cargo clippy`, then four review passes (parallel agents for large changes): frame loop and repaint, IDs and widget state, API and Rust correctness, UX and accessibility. Read-only. |
| `egui-kittest` | Tool | Writes and runs `egui_kittest` UI tests: accessibility queries, simulated input, frame stepping, and image snapshots. |
| `eframe-project` | Process | Sets up or changes an eframe app: Cargo features, renderer choice, `main.rs` and `App` layout, persistence, workspace split, and web builds. |
| `egui-ui-design` | Conceptual | Designs or audits egui screens: layout, primary buttons and dialogs, interface wording, `Visuals` colours, `TextStyle` sizes, keyboard use, and accessibility, then checks a rendered screenshot. |
| `egui-profiler` | Tool | Finds why an egui app is slow or uses CPU when idle: repaint causes, `samply`, `profiling` scopes with puffin or Tracy, and a list of common causes. |

Skill types:

- **Conceptual**: corrects mistakes models make by default
- **Review**: structured code review
- **Process**: project setup and change workflows
- **Tool**: runs a tool and interprets its output

## Repository layout

```
skills/
  egui/SKILL.md
  egui-review/SKILL.md
  egui-kittest/SKILL.md
  eframe-project/SKILL.md
  egui-ui-design/SKILL.md
  egui-profiler/SKILL.md
.claude-plugin/           # Claude Code and Copilot CLI plugin manifests
gemini-extension.json     # Gemini CLI extension manifest
scripts/check_skills.py   # frontmatter and version checks, run in CI
examples/snippets/        # every Rust snippet from the skills, compiled and tested in CI
examples/manifests/       # every Cargo.toml snippet from the skills, resolved in CI
evals/                    # with-skill vs without-skill agent runs
CONTRIBUTING.md
LICENSE
```

## Installation

### Claude Code

Install as a plugin:

```
/plugin marketplace add gregorycarnegie/egui-agent-skills
/plugin install egui-development-skills
```

Or link one skill into your personal skills folder:

```bash
ln -s "$(pwd)/skills/egui" ~/.claude/skills/egui
```

### Codex CLI

```bash
ln -s "$(pwd)/skills/egui" ~/.codex/skills/egui
```

Restart Codex after adding skills. A link picks up changes when you pull this
repository; a copy (`cp -r`) has to be copied again after every update.

### GitHub Copilot

Copy a skill into `.github/skills/` (one project) or `~/.copilot/skills/` (all
projects). Copilot also finds skills installed for Claude Code under
`.claude/skills/`.

### Google Antigravity

Link a skill into the global skills folder, which the Antigravity app, IDE, and
CLI all read:

```bash
mkdir -p ~/.gemini/config/skills
ln -s "$(pwd)/skills/egui" ~/.gemini/config/skills/egui
```

For one project only, put the skill folder in `.agents/skills/` at the workspace
root instead.

### Gemini CLI

Gemini CLI stopped serving individual accounts (including Google AI Pro and
Ultra) on June 18, 2026. It still works with paid Gemini API keys and Gemini Code
Assist Standard or Enterprise. Everyone else should use Antigravity.

```bash
gemini extensions install https://github.com/gregorycarnegie/egui-agent-skills
```

## Updating

New versions are described in [CHANGELOG.md](CHANGELOG.md) and published as
[GitHub Releases](https://github.com/gregorycarnegie/egui-agent-skills/releases).
To hear about them, watch the repository and choose Releases.

- **Claude Code plugin**: refresh the marketplace, update the plugin, then
  restart Claude Code.

  ```bash
  claude plugin marketplace update egui-skills
  claude plugin update egui-development-skills
  ```

- **Linked skills** (Claude Code, Codex, Copilot, Antigravity): run `git pull`
  in your clone.
- **Copied skills**: run `git pull`, then copy the skill folders again.
- **Gemini CLI** (paid API keys, Code Assist Standard or Enterprise): run
  `gemini extensions update egui-skills`, or
  `gemini extensions update --all`. To update automatically, add
  `--auto-update` to the install command.

## License

MIT, with Qt-derived portions under BSD-3-Clause. See [LICENSE](LICENSE).
This repository started as a fork of the Qt AI Skills repository by The Qt
Company Ltd.; its notice is kept in the same file.
