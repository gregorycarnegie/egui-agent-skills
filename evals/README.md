# Evals

`run.py` gives an agent a small eframe 0.36 app and a coding task, once with this
repo loaded as a plugin and once without. Each result is graded by `cargo check`
and by patterns the skills warn about, such as `SidePanel`, `fn update`, or a
blocking `recv()`.

```bash
python evals/run.py --runs 3               # every case
python evals/run.py --case glow --runs 1   # one case
```

It needs a logged-in `claude` CLI, cargo, and network access, and it uses API
credit. Neither arm loads user settings, plugins, or hooks.

## Results: 2026-09-13, CLI default model, 3 runs per case

| Case | With skills | Without | Cost with | Cost without |
|---|---|---|---|---|
| `side-panel` | 3/3 | 3/3 | $0.70 | $0.94 |
| `background-load` | 3/3 | 3/3 | $0.54 | $0.53 |
| `note-list` | 3/3 | 3/3 | $0.56 | $0.44 |
| `glow` | 3/3 | 0/3 | $0.64 | $1.37 |
| **Total** | **12/12** | **9/12** | **$2.43** | **$3.29** |

The skill loaded in all 12 runs of the with-skills arm.

What this shows:

- **`glow`**: without the skills, every run dropped winit's
  `wayland-csd-adwaita`, so the app has no title bar on GNOME Wayland. Those runs
  also took about twice as many turns (14–33 against 11–13).
- **`side-panel`**: both arms compiled, but runs without the skills spent more
  turns and tool calls finding the 0.36 names.
- **`background-load`, `note-list`**: no difference. The starter code already
  uses `fn ui`, and the model gets threads, repaints, and loop IDs right without
  help. These cases are too easy to tell the arms apart. Harder cases, such as a
  starter with no 0.36 code to copy, would give a clearer signal.
- Before the panel sizing row was added to the `egui` API table, a smoke run
  with the skills wrote `Panel::default_width`, which does not compile.

Limits: three runs per case is a small sample, and the `glow` check looks for
the skill's own fix, so it tests whether the advice is followed more than
whether it is right. The recipe itself is confirmed separately, by the manifests
job in CI.

## `egui-ui-design` cases: 2026-09-13, 3 runs per case, two rounds

Round 1 used the skill's earlier description. Round 2 used the current one,
which names dialogs, confirmations, and error messages. In round 2 the `egui`
skill also points to `egui-ui-design` for anything users see.

| Case | Round | With skills | Without | `egui-ui-design` loaded | Cost with | Cost without |
|---|---|---|---|---|---|---|
| `delete-confirm` | 1 | 3/3 | 0/3 | 0/3 | $1.38 | $0.54 |
| `delete-confirm` | 2 | 3/3 | 2/3 | 3/3 | $0.89 | $0.53 |
| `load-error` | 1 | 3/3 | 3/3 | 0/3 | $0.62 | $0.48 |
| `load-error` | 2 | 3/3 | 3/3 | 0/3 | $0.63 | $0.43 |

What this shows:

- **The description decides whether the skill helps at all.** With the old
  description, `egui-ui-design` never loaded, not even for a confirmation
  dialog. The new one loaded it in every `delete-confirm` run.
- **`delete-confirm`**: all 6 runs with skills used an `egui::Modal`; 4 of 6
  runs without skills built the confirmation some other way. Round 1 runs with
  skills used a `Modal` without loading `egui-ui-design`, so part of the gain
  comes from elsewhere in the plugin. No run in either arm labelled buttons
  "Yes", "No", or "OK", so the wording rule was not tested by this case.
- **`load-error`**: no difference, and `egui-ui-design` did not load. The model
  already uses `error_fg_color` and avoids apologies without help, so this case
  measures nothing yet.
- Loading skills costs tokens: on these small tasks, runs with skills cost more.

Limits: three runs per round is a small sample, the `Modal` check follows the
skill's own advice, and runs with skills sometimes also load Claude Code's
built-in `run` skill, which tries to launch the app and is denied.
