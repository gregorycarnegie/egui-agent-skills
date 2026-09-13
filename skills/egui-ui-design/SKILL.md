---
name: egui-ui-design
description: >-
  Designs or audits user interfaces for egui and eframe apps on desktop and the
  web. Use when creating screens, layouts, navigation, dialogs, or themes, or
  when auditing UX and accessibility. Maps design rules to egui's Style,
  Visuals, TextStyle, and layout tools.
license: BSD-3-Clause
compatibility: >-
  Designed for Claude Code, Codex CLI, GitHub Copilot, and similar agents.
metadata:
  author: egui-skills
  version: "1.0"
  egui-version: "0.36"
  category: conceptual
---

# egui UI Design

Before designing a screen, confirm the items in section 0. Look in the
conversation and the project first, and ask only for what is still missing. If
the user cannot answer, choose a sensible default, name it in the response, and
continue.

Small edits such as "move this button right", "rename this label", or "make it
red" skip section 0. Apply section 1 silently, and check contrast from section 2
where it matters.

For an audit of an existing UI, go to section 4.

egui runs on desktop (Windows, macOS, Linux) and in web browsers. It does not
target microcontroller displays, and this skill does not cover them.

## 0. Context check

1. **Platform**: native desktop, web, or both? Mouse and keyboard, touch, or both?
2. **Window size**: the smallest and the typical window size the app must work at.
3. **Existing style**: does the app already customise `egui::Style` or
   `Visuals`? Search for `style_mut`, `all_styles_mut`, and `set_visuals`. If so,
   reuse its values. If not, start from egui's default dark and light visuals.
4. **Content priority**: what is primary, secondary, and tertiary on this screen?
5. **Theme**: dark, light, or follow the OS (`egui::ThemePreference::System`)?
   Design both themes unless the user rules one out.
6. **Languages**: which languages ship? Any Chinese, Japanese, Korean, Arabic, or
   Hebrew text?

## 1. Design principles

Apply these while designing. Do not ask the user about each one.

**Content and layout**
- Progressive disclosure: show what the current step needs. Put advanced options
  in a `CollapsingHeader`, a separate panel, or a settings window.
- Most important information first (top-left for left-to-right languages).
- Split complex flows into panels, tabs, or steps.
- Of two equally good designs, choose the simpler.
- Fewer steps mean more users finish the task.

**Perception and interaction**
- Follow patterns users know from other desktop apps: menu bar, toolbar, side
  panel, status bar.
- Controls look like what they do. Something clickable is a button or link, not
  a plain label.
- More choices mean slower decisions. Limit primary actions per screen.
- Group items into chunks of about seven.
- Show options instead of making users remember them.
- Related controls sit together and look alike; separate groups with
  `ui.group(..)`, an `egui::Frame`, or `ui.separator()`.
- One element that stands out draws attention. Use that sparingly.
- Respond within 400 ms, or show progress.
- Users always know where they are: selected tab, highlighted list item, window title.

**Buttons**
- One primary action per group; other actions look secondary.
- Keep OK and Cancel together, in the same order everywhere in the app.

**Error prevention**
- Guide correct input: limit numeric fields to their valid range, and use a
  `ComboBox` instead of free text when the set of values is fixed.
- Offer undo where possible.
- Confirm destructive or irreversible actions, for example in an `egui::Modal`.

**Window sizes**
- Design for the smallest supported window first.
- `ui.available_width()` gives the space left. Below a width you choose, switch
  from side-by-side to stacked layout.
- Give side `Panel`s a `min_size`, a `max_size`, and `resizable(true)`. Consider
  `show_collapsible` so users can hide them in small windows.
- Put anything that can overflow inside a `ScrollArea`.

### 1.1 Motion

- Motion must explain a change, such as a panel sliding in or a section
  expanding. Never add it as decoration.
- egui animates collapsing sections, panels, and windows over
  `style.animation_time` seconds. Keep it short, around 0.1–0.2 s.
- For custom animation use `ctx.animate_bool_responsive(id, value)` or
  `ctx.animate_value_with_time(id, target, seconds)`. They keep repainting only
  while the value moves.
- egui does not read the OS "reduce motion" setting. If the app offers such a
  setting, set `style.animation_time = 0.0` when it is on.
- Animate at most one or two things at once.

### 1.2 Type scale

egui sizes text through `TextStyle`; each style maps to a `FontId` in
`style.text_styles`. Set the scale once, in the app constructor, for both themes:

```rust
use egui::{FontFamily, FontId, TextStyle};

ctx.all_styles_mut(|style| {
    style.text_styles = [
        (TextStyle::Heading, FontId::new(22.0, FontFamily::Proportional)),
        (TextStyle::Body, FontId::new(15.0, FontFamily::Proportional)),
        (TextStyle::Button, FontId::new(15.0, FontFamily::Proportional)),
        (TextStyle::Monospace, FontId::new(14.0, FontFamily::Monospace)),
        (TextStyle::Small, FontId::new(11.0, FontFamily::Proportional)),
    ]
    .into();
});
```

- Derive the sizes from a modular scale: choose a body size and a ratio, multiply
  by the ratio for each larger step, and divide for each smaller step.

  | Ratio | Factor | Suits |
  |---|---|---|
  | Major second | 1.125 | Dense tools and data-heavy screens |
  | Minor third | 1.200 | General desktop apps |
  | Major third | 1.250 | Roomier layouts, onboarding |
  | Perfect fourth | 1.333 | Dashboards, strong hierarchy |

- Sizes are in points. egui scales them for the screen's pixel density and the
  zoom factor, so never multiply sizes by the DPI yourself.
- Use at most three or four sizes on one screen.
- Body text: 14–16 pt for reading; dense tools can use 13 pt. Never use `Small`
  for primary content.
- Let users zoom (`egui::gui_zoom::zoom_in(ctx)`, `zoom_out(ctx)`, or
  `ctx.set_zoom_factor(..)`), and check that layouts still work at 150%.
- Keep prose lines to about 45–75 characters by limiting the width of text areas.
- egui's default fonts do not include Chinese, Japanese, or Korean characters.
  For those languages, add a font with `ctx.set_fonts(..)` in the constructor.

### 1.3 Keyboard and input

- Every action is reachable by keyboard. Tab moves focus between egui widgets;
  press Tab through each screen and check that the order follows reading order.
- Tab to every custom widget built with `ui.allocate_response(..)`. If focus
  skips one, check in your egui version's `sense.rs` which `Sense` values make a
  widget focusable.
- Focus must be visible. After changing `Visuals`, tab through the UI in both
  themes and confirm the focused widget stands out.
- Dialogs close from the keyboard. For `egui::Modal`, check
  `modal_response.should_close()` and close the dialog when it returns `true`.
- Hover text (`response.on_hover_text(..)`) is extra help only. Anything
  essential must be visible without hovering; touch screens cannot hover.
- Shortcuts use `egui::Modifiers::COMMAND` (Ctrl on Windows and Linux, Cmd on
  macOS) and are shown in the menu next to the action.

### 1.4 Semantic colour

Take colours for a meaning from `Visuals`, never a fixed value:

| Meaning | Use |
|---|---|
| Error | `ui.visuals().error_fg_color` |
| Warning | `ui.visuals().warn_fg_color` |
| Link | `ui.visuals().hyperlink_color` |
| Selection or accent | `ui.visuals().selection.bg_fill` |
| Panel and window backgrounds | `panel_fill`, `window_fill` |
| Text field and code backgrounds | `extreme_bg_color`, `code_bg_color` |
| Striped rows | `faint_bg_color` |

- Set brand colours for both themes with
  `ctx.set_visuals_of(egui::Theme::Dark, ..)` and `egui::Theme::Light`, or with
  `ctx.style_mut_of(..)`.
- A colour that means "clickable" is not also used for decoration.
- Colour never carries state alone. Pair success, warning, and error colours with
  an icon or text.
- Test both themes. With `egui_kittest`, build one harness with
  `.with_theme(egui::Theme::Dark)` and one with `egui::Theme::Light`.

### 1.5 Languages

- egui has no translation system. Keep user-visible strings in one place, or use
  a localisation crate, so they can be translated.
- Translated text is often 30–40% longer than English. Avoid fixed-width buttons
  and labels; let widgets size to their text.
- Format dates, times, and numbers for the user's locale with a crate. Never
  hardcode a format such as `DD/MM/YYYY`.
- Right-to-left scripts: egui 0.36 shapes text with harfrust but has no
  bidirectional-text dependency, and its layouts run left to right. Test real
  Arabic or Hebrew strings, including strings mixed with numbers and Latin text,
  before promising right-to-left support. `Layout::right_to_left` changes the
  order widgets are placed in; it does not reorder text.
- Do not put text inside images or icons.

## 2. Accessibility (WCAG 2.2)

Check every design against these:

- Contrast at least 4.5:1 for text, and 3:1 for large text and UI components.
  Check again after changing any `Visuals` colour, in both themes.
- Colour is never the only sign of state.
- Every interactive element works from the keyboard, with visible focus.
- Labels, instructions, and error messages use plain language.
- eframe's AccessKit support (on by default) exposes widgets with text to screen
  readers. Icon-only buttons need text too, for example `ui.button("🗑 Delete")`.
- At 150% zoom, no text is clipped and nothing overlaps.
- With a colour-blindness simulator (deuteranopia is the most common type), every
  state change is still visible.

## 3. AI features

When a screen includes AI features:

- Users can start, stop, and change AI actions. Offer undo or regenerate.
- Show why the AI did something when it affects the user.
- When the AI fails, offer a manual way to finish the task.
- Mark uncertain output ("suggested", "check before use"); never present it as fact.
- Show streamed text as it arrives, with a busy indicator at the insertion point,
  and request a repaint as each piece arrives.
- For long operations, show a placeholder shaped like the expected result rather
  than a lone spinner. Show background work as a small status indicator, not a
  blocking dialog.
- Never blank the whole window while waiting.
- Say before the AI reads, sends, or changes the user's data.

## 4. Audit

Sort each finding:

1. **Critical**: breaks a WCAG rule or blocks a task. Must fix.
2. **Warning**: adds friction or mental effort. Should fix.
3. **Opportunity**: an improvement. Consider.

Checklist:

- [ ] Works at the smallest supported window size and at 150% zoom.
- [ ] Every action works from the keyboard, with visible focus in both themes.
- [ ] Colours come from `Visuals`; no state is shown by colour alone; contrast passes in both themes.
- [ ] At most three or four text sizes per screen, all set through `TextStyle`.
- [ ] Destructive actions ask for confirmation or can be undone.
- [ ] Long work shows progress and never freezes the UI.
- [ ] Strings can grow 40% without clipping, and fonts cover every shipped language.
- [ ] AI features follow section 3.

For each finding, give the severity, the location (`file:line` or screen area),
the problem, and the fix.

## References

- Nielsen Norman Group, 10 usability heuristics: https://www.nngroup.com/articles/ten-usability-heuristics/
- Laws of UX: https://lawsofux.com/
- WCAG 2.2: https://www.w3.org/TR/WCAG22/
- Modular scale calculator: https://www.modularscale.com/
- egui web demo, including the style editor: https://www.egui.rs/
