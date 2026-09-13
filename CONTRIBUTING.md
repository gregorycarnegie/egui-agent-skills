# Contributing

A skill is a folder under `skills/` that contains a `SKILL.md` file. Everything
else is optional.

## Add a skill

1. Create `skills/<name>/SKILL.md`. Start the name with `egui-`, or with
   `eframe-` for a skill that is only about eframe. Use lowercase letters and
   hyphens, at most 64 characters, and match the folder name exactly.
2. Start the file with this frontmatter:

   ```yaml
   ---
   name: egui-my-topic
   description: >-
     What the skill does and when an agent should use it, including the words
     a user would say. Put the key information in the first 250 characters;
     some tools cut off the rest.
   license: BSD-3-Clause
   compatibility: >-
     Designed for Claude Code, Codex CLI, GitHub Copilot, and similar agents.
   metadata:
     author: egui-skills
     version: "1.0"
     egui-version: "0.36"
     category: conceptual   # conceptual | review | process | tool
   ---
   ```

3. Add a row to the skills table in `README.md`.

## Write the body

- Keep `SKILL.md` under 500 lines. Move long checklists to a `references/`
  folder and link to them from `SKILL.md`.
- Cover what models get wrong, not what they already do well.
- Show the correct pattern in a short code example, and say why it is correct.
- **Check every API name against the crate source.** egui changes its API
  between minor versions, and models remember old names. After `cargo fetch`,
  the source is in `~/.cargo/registry/src/*/egui-<version>/src/`. Search it
  before you name a function, type, field, or feature flag.
- When a skill moves to a new egui version, update `egui-version` in its
  frontmatter and the API table in `skills/egui/SKILL.md`.

## Test

1. Paste the frontmatter into a YAML validator.
2. Check that every file the skill links to exists.
3. Install the skill in an agent. Give it a real egui task that uses the trigger
   words from the description. Check that the skill loads and that the code it
   writes compiles against the egui version in the frontmatter.

## Release

The plugin version appears in four places: `.claude-plugin/plugin.json`,
`.claude-plugin/marketplace.json` (twice), and `gemini-extension.json`. Change
all four together.
