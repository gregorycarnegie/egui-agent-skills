# Changelog

## [0.2.0] - 2026-09-13

### Added

- A tokio recipe in the `egui` skill: enter a runtime before
  `eframe::run_native`, spawn from `ui`, and send results back over a channel
  with a repaint request. Tests cover the pattern and the panic when no runtime
  is entered.
- `examples/manifests/`: every TOML block from the skills, resolved by cargo in
  CI, which fails on a crate version or feature name that does not exist. CI
  also checks that the glow-only recipe keeps winit's Wayland features.
- `evals/run.py`: runs coding tasks with and without the skills loaded, and
  grades each result with `cargo check` and pattern checks.

### Changed

- `egui-review` reviews small changes in one pass instead of starting four
  agents.
- CI runs once per pull request instead of twice.

### Removed

- The per-skill `metadata.version` field. It duplicated the plugin version and
  had drifted from it.

### Fixed

- The glow-only feature list in `eframe-project` also turned off winit's default
  features, so windows on GNOME Wayland had no title bar and the build linked
  libwayland directly. The recipe now restores both.
- The `egui` API table now maps the old panel sizing methods (`default_width`
  and the rest) to `Panel`'s `*_size` methods. The first eval run caught an
  agent writing `Panel::default_width`.

## [0.1.0] - 2026-09-13

### Added

- Six skills targeting egui 0.36: `egui`, `egui-review`, `egui-kittest`,
  `eframe-project`, `egui-ui-design`, and `egui-profiler`.
- Plugin manifests and installation instructions for Claude Code, Codex CLI,
  GitHub Copilot, and Gemini CLI.
- CI checks for skill metadata, documentation, snippet coverage, and matching
  plugin versions.
- A Rust snippet crate compiled against egui 0.36.2, with Clippy checks and
  tests for kittest behaviour and the immediate viewport pattern.

### Changed

- Licensed original contributions under MIT while retaining BSD-3-Clause
  terms and Qt's copyright notice for Qt-derived portions.

### Fixed

- Corrected egui 0.36 guidance for `TextEdit::id_source`,
  `show_animated_inside`, and viewport callback arguments.
- Replaced installation URL placeholders with the repository address.

[0.2.0]: https://github.com/gregorycarnegie/egui-agent-skills/releases/tag/v0.2.0
[0.1.0]: https://github.com/gregorycarnegie/egui-agent-skills/releases/tag/v0.1.0
