# egui Agent Skills

Agent skills for building Rust GUI apps with [egui](https://github.com/emilk/egui)
and [eframe](https://github.com/emilk/egui/tree/main/crates/eframe). They work with
AI coding tools such as Claude Code, Codex CLI, Gemini CLI, and GitHub Copilot.

The skills target **egui 0.36**. That release removed or renamed many APIs that
models still suggest, such as `App::update`, `SidePanel`, and `Frame::none()`. When
a project uses another egui version, the skills check API names against that
version's crate source before using them.

> These skills use AI and can make mistakes. Always check the output.

## Skills

| Skill | Type | Description |
|-------|------|-------------|
| `egui` | Conceptual | Rules for writing and fixing egui/eframe code: the immediate-mode model, IDs, repaints, threads, persistence, and a table of APIs changed in 0.36. |
| `egui-review` | Review | Runs `cargo clippy`, then four review agents in parallel: frame loop and repaint, IDs and widget state, API and Rust correctness, UX and accessibility. Read-only. |
| `egui-kittest` | Tool | Writes and runs `egui_kittest` UI tests: accessibility queries, simulated input, frame stepping, and image snapshots. |
| `eframe-project` | Process | Sets up or changes an eframe app: Cargo features, renderer choice, `main.rs` and `App` layout, persistence, workspace split, and web builds. |
| `egui-ui-design` | Conceptual | Designs or audits egui screens: layout, type scale through `TextStyle`, semantic colour through `Visuals`, keyboard use, and accessibility. |
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
CONTRIBUTING.md
LICENSE
```

## Installation

### Claude Code

Install as a plugin from a local checkout or a git URL:

```
/plugin marketplace add <path-or-git-url>
/plugin install egui-development-skills
```

Or link one skill into your personal skills folder:

```bash
ln -s "$(pwd)/skills/egui" ~/.claude/skills/egui
```

### Codex CLI

```bash
cp -r skills/egui ~/.codex/skills/egui
```

Restart Codex after adding skills.

### GitHub Copilot

Copy a skill into `.github/skills/` (one project) or `~/.copilot/skills/` (all
projects). Copilot also finds skills installed for Claude Code under
`.claude/skills/`.

### Gemini CLI

```bash
gemini extensions install <path-or-git-url>
```

## License

BSD-3-Clause. See [LICENSE](LICENSE). This repository started as a fork of the Qt
AI Skills repository by The Qt Company Ltd.; the license requires keeping its
copyright notice.
