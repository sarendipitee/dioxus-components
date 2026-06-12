use crate::components::{
    button::{Button, ButtonSize, ButtonVariant},
    color_input::ColorInput,
    input::TextInput,
    slider::Slider,
    tabs::{TabContent, TabList, TabTrigger, Tabs, TabsVariant},
    textarea::{Textarea, TextareaVariant},
};
use crate::Route;
use dioxus::prelude::{dioxus_router::Link, *};
use dioxus_components::{InputSize, InputWrapper};
use dioxus_primitives::color_picker::Color;
use palette::{encoding, FromColor, Hsv, IntoColor, Srgb};
use std::sync::OnceLock;

type ThemeColor = Hsv<encoding::Srgb, f64>;

const STORAGE_KEY: &str = "dx_preview_theme_overrides_css";
const DEFAULT_THEME_CSS: &str = include_str!("../assets/dx-components-theme.css");
const THEME_SECTIONS: &[&str] = &[
    "Core", "Surface", "Input", "Accent", "Status", "Focus", "Layout",
];
static DEFAULT_THEME_TOKENS: OnceLock<Vec<ThemeToken>> = OnceLock::new();

#[derive(Clone, PartialEq)]
enum TokenMode {
    Dual { light: String, dark: String },
    Single { value: String },
}

#[derive(Clone, PartialEq)]
struct ThemeToken {
    name: String,
    section: &'static str,
    mode: TokenMode,
}

#[derive(Clone, Copy)]
struct ThemeCustomizerContext {
    open: Signal<bool>,
    tokens: Signal<Vec<ThemeToken>>,
    loaded: Signal<bool>,
}

fn use_theme_customizer() -> ThemeCustomizerContext {
    use_context::<ThemeCustomizerContext>()
}

fn token_section(name: &str) -> &'static str {
    if matches!(name, "bg" | "fg" | "fg-muted" | "fg-faint") {
        "Core"
    } else if name.starts_with("surface")
        || name == "overlay"
        || name == "overlay-fg"
        || name == "overlay-border"
    {
        "Surface"
    } else if name.starts_with("input") {
        "Input"
    } else if name.starts_with("accent") {
        "Accent"
    } else if matches!(
        name,
        "success"
            | "success-fg"
            | "success-border"
            | "success-subtle"
            | "success-subtle-fg"
            | "success-subtle-border"
            | "warning"
            | "warning-fg"
            | "warning-border"
            | "warning-subtle"
            | "warning-subtle-fg"
            | "warning-subtle-border"
            | "danger"
            | "danger-fg"
            | "danger-border"
            | "danger-hover"
            | "danger-hover-fg"
            | "danger-hover-border"
            | "danger-active"
            | "danger-active-fg"
            | "danger-active-border"
            | "danger-subtle"
            | "danger-subtle-fg"
            | "danger-subtle-border"
            | "info"
            | "info-fg"
            | "info-border"
            | "info-subtle"
            | "info-subtle-fg"
            | "info-subtle-border"
    ) {
        "Status"
    } else if name.starts_with("focus") {
        "Focus"
    } else {
        "Layout"
    }
}

fn parse_dual_value(value: &str) -> Option<(String, String)> {
    fn parse_branch<'a>(input: &'a str, prefix: &str) -> Option<(String, &'a str)> {
        let trimmed = input.trim_start();
        let rest = trimmed.strip_prefix(prefix)?;
        let mut depth = 1usize;
        for (index, ch) in rest.char_indices() {
            match ch {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some((rest[..index].trim().to_string(), &rest[index + 1..]));
                    }
                }
                _ => {}
            }
        }
        None
    }

    let (light, rest) = parse_branch(value, "var(--is-light, ")?;
    let (dark, tail) = parse_branch(rest, "var(--is-dark, ")?;
    if tail.trim().is_empty() {
        Some((light, dark))
    } else {
        None
    }
}

fn is_reserved_theme_token(name: &str) -> bool {
    matches!(name, "ON" | "OFF" | "is-light" | "is-dark")
}

fn parse_theme_tokens(css: &str) -> Vec<ThemeToken> {
    let mut tokens = Vec::new();
    let mut in_root = false;

    for line in css.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(":root {") {
            in_root = true;
            continue;
        }
        if in_root && trimmed == "}" {
            break;
        }
        if !in_root || !trimmed.starts_with("--") {
            continue;
        }

        let Some((name, value)) = trimmed
            .strip_suffix(';')
            .and_then(|line| line.split_once(':'))
        else {
            continue;
        };

        let name = name.trim().trim_start_matches("--").to_string();
        if is_reserved_theme_token(&name) {
            continue;
        }
        let value = value.trim();
        let mode = if let Some((light, dark)) = parse_dual_value(value) {
            TokenMode::Dual { light, dark }
        } else {
            TokenMode::Single {
                value: value.to_string(),
            }
        };

        tokens.push(ThemeToken {
            section: token_section(&name),
            name,
            mode,
        });
    }

    tokens
}

fn default_theme_tokens() -> &'static [ThemeToken] {
    DEFAULT_THEME_TOKENS
        .get_or_init(|| parse_theme_tokens(DEFAULT_THEME_CSS))
        .as_slice()
}

fn build_theme_css(tokens: &[ThemeToken]) -> String {
    let mut css = String::from(
        "/* This file contains the theme variables for the styled dioxus-components components. You only\n * need to import this file once in your project root.\n */\n\n:root {\n  /* Theme gates. \"initial\" enables a fallback; an empty value disables it.\n     Keep --OFF empty. Formatters must not remove the declaration. */\n  --ON: initial;\n  --OFF: ;\n  --is-light: var(--ON);\n  --is-dark: var(--OFF);\n\n  accent-color: var(--accent);\n  color-scheme: var(--is-light, light) var(--is-dark, dark);\n\n",
    );

    for token in tokens {
        match &token.mode {
            TokenMode::Dual { light, dark } => {
                css.push_str(&format!(
                    "  --{}: var(--is-light, {}) var(--is-dark, {});\n",
                    token.name, light, dark
                ));
            }
            TokenMode::Single { value } => {
                css.push_str(&format!("  --{}: {};\n", token.name, value));
            }
        }
    }

    css.push_str(
        "}\n\n@media (prefers-color-scheme: dark) {\n  :root:not([data-theme=\"light\"], [data-theme=\"dark\"]) {\n    --is-light: var(--OFF);\n    --is-dark: var(--ON);\n  }\n}\n\n:root[data-theme=\"light\"],\nhtml[data-theme=\"light\"] {\n  --is-light: var(--ON);\n  --is-dark: var(--OFF);\n}\n\n:root[data-theme=\"dark\"],\nhtml[data-theme=\"dark\"] {\n  --is-light: var(--OFF);\n  --is-dark: var(--ON);\n}\n",
    );
    css
}

fn is_css_dimension_number(value: &str) -> bool {
    let Some(rest) = value.strip_prefix(['+', '-']) else {
        return is_css_dimension_number_unsigned(value);
    };

    !rest.is_empty() && is_css_dimension_number_unsigned(rest)
}

fn is_css_dimension_number_unsigned(value: &str) -> bool {
    let mut seen_digit = false;
    let mut seen_dot = false;

    for ch in value.chars() {
        if ch.is_ascii_digit() {
            seen_digit = true;
            continue;
        }
        if ch == '.' && !seen_dot {
            seen_dot = true;
            continue;
        }
        return false;
    }

    seen_digit
}

fn normalize_numeric_css_value(value: &str) -> Option<String> {
    let trimmed = value.trim();
    let split_at =
        trimmed.find(|ch: char| !(ch.is_ascii_digit() || matches!(ch, '+' | '-' | '.')))?;
    let (number, suffix) = trimmed.split_at(split_at);
    let suffix = suffix.trim();
    if number.is_empty()
        || suffix.is_empty()
        || !is_css_dimension_number(number)
        || !suffix
            .chars()
            .all(|ch| ch.is_ascii_alphabetic() || ch == '%')
    {
        return None;
    }

    let parsed = number.parse::<f64>().ok()?;
    let mut normalized = parsed.to_string();
    if normalized == "-0" {
        normalized = "0".to_string();
    }

    Some(format!("{normalized}{suffix}"))
}

fn normalize_theme_value(value: &str) -> String {
    let trimmed = value.trim();
    if let Some(color) = color_from_hex(trimmed) {
        return hex_from_color(color).to_ascii_lowercase();
    }
    if let Some(numeric) = normalize_numeric_css_value(trimmed) {
        return numeric;
    }

    trimmed.to_string()
}

fn normalize_theme_token(token: &ThemeToken) -> ThemeToken {
    let mode = match &token.mode {
        TokenMode::Dual { light, dark } => TokenMode::Dual {
            light: normalize_theme_value(light),
            dark: normalize_theme_value(dark),
        },
        TokenMode::Single { value } => TokenMode::Single {
            value: normalize_theme_value(value),
        },
    };

    ThemeToken {
        name: token.name.clone(),
        section: token.section,
        mode,
    }
}

fn compute_theme_override_css(tokens: &[ThemeToken]) -> Option<String> {
    let normalized_tokens: Vec<_> = tokens.iter().map(normalize_theme_token).collect();
    let normalized_defaults: Vec<_> = default_theme_tokens()
        .iter()
        .map(normalize_theme_token)
        .collect();

    if normalized_tokens == normalized_defaults {
        None
    } else {
        Some(build_theme_css(tokens))
    }
}

fn sync_draft_with_committed_value(
    last_synced: &mut String,
    draft: &mut String,
    current: &str,
) -> bool {
    if last_synced == current {
        return false;
    }

    *last_synced = current.to_string();
    *draft = current.to_string();
    true
}

fn color_from_hex(hex: &str) -> Option<ThemeColor> {
    let hex = hex.trim().strip_prefix('#')?;
    let hex = match hex.len() {
        3 if hex.bytes().all(|byte| byte.is_ascii_hexdigit()) => {
            let mut expanded = String::with_capacity(6);
            for ch in hex.chars() {
                expanded.push(ch);
                expanded.push(ch);
            }
            expanded
        }
        6 if hex.bytes().all(|byte| byte.is_ascii_hexdigit()) => hex.to_string(),
        _ => return None,
    };

    let red = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let green = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let blue = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some(
        Color::new(red, green, blue)
            .into_format::<f64>()
            .into_color(),
    )
}

fn hex_from_color(color: ThemeColor) -> String {
    let rgb: Color = Srgb::<f64>::from_color(color).into_format();
    format!("#{rgb:X}")
}

fn is_hex_color(value: &str) -> bool {
    color_from_hex(value).is_some()
}

fn slider_spec(name: &str, value: &str) -> Option<(f64, f64, f64, &'static str, f64)> {
    let numeric = value.trim().strip_suffix("rem")?.parse::<f64>().ok()?;
    match name {
        "radius" | "radius-surface" | "radius-input" | "radius-control" => {
            Some((0.0, 2.0, 0.05, "rem", numeric))
        }
        "space" => Some((0.125, 1.0, 0.0625, "rem", numeric)),
        "control-height-sm" | "control-height-md" | "control-height-lg" | "input-height-sm"
        | "input-height-md" | "input-height-lg" => Some((1.0, 3.0, 0.125, "rem", numeric)),
        _ => None,
    }
}

fn update_token(tokens: &mut Signal<Vec<ThemeToken>>, index: usize, mode: TokenMode) {
    let mut next = tokens();
    if let Some(token) = next.get_mut(index) {
        if token.mode == mode {
            return;
        }
        token.mode = mode;
    }
    tokens.set(next);
}

fn update_dual_branch(
    tokens: &mut Signal<Vec<ThemeToken>>,
    index: usize,
    is_dark: bool,
    value: String,
) {
    let current = tokens()
        .get(index)
        .cloned()
        .expect("theme token index should exist");
    if let TokenMode::Dual { light, dark } = current.mode {
        let mode = if is_dark {
            TokenMode::Dual { light, dark: value }
        } else {
            TokenMode::Dual { light: value, dark }
        };
        update_token(tokens, index, mode);
    }
}

fn update_single_value(tokens: &mut Signal<Vec<ThemeToken>>, index: usize, value: String) {
    update_token(tokens, index, TokenMode::Single { value });
}

fn token_display_value(token: &ThemeToken, is_dark: bool) -> String {
    match &token.mode {
        TokenMode::Dual { light, dark } => {
            if is_dark {
                dark.clone()
            } else {
                light.clone()
            }
        }
        TokenMode::Single { value } => value.clone(),
    }
}

/// Provides preview-wide structured theme editor state and applies overrides at the document root.
#[component]
pub(crate) fn ThemeCustomizerProvider(children: Element) -> Element {
    let mut ctx = ThemeCustomizerContext {
        open: use_signal(|| false),
        tokens: use_signal(|| default_theme_tokens().to_vec()),
        loaded: use_signal(|| false),
    };
    let mut load_started = use_signal(|| false);
    use_context_provider(|| ctx);
    let theme_override_css = use_memo(move || compute_theme_override_css(&(ctx.tokens)()));

    let current_route: Route = router().current();
    use_effect(use_reactive!(|current_route| {
        if matches!(current_route, Route::Theme { .. }) && !(ctx.open)() {
            ctx.open.set(true);
        }
    }));

    use_effect(move || {
        if load_started() {
            return;
        }
        load_started.set(true);

        spawn(async move {
            let mut eval = document::eval(&format!(
                "dioxus.send(window.localStorage.getItem('{STORAGE_KEY}') ?? '');"
            ));
            if let Ok(saved) = eval.recv::<String>().await {
                if !saved.trim().is_empty() {
                    let current_tokens = (ctx.tokens)();
                    if current_tokens == default_theme_tokens() {
                        ctx.tokens.set(parse_theme_tokens(&saved));
                    }
                }
            }
            ctx.loaded.set(true);
        });
    });

    let loaded = (ctx.loaded)();
    let css_for_storage = theme_override_css();
    use_effect(use_reactive!(|(loaded, css_for_storage)| {
        if !loaded {
            return;
        }

        let eval = document::eval(&format!(
            "let state = await dioxus.recv();
            if (state === null) {{
              window.localStorage.removeItem('{STORAGE_KEY}');
            }} else {{
              window.localStorage.setItem('{STORAGE_KEY}', state);
            }}"
        ));
        let _ = eval.send(css_for_storage);
    }));

    let css_for_dom = theme_override_css();
    use_effect(use_reactive!(|css_for_dom| {
        let eval = document::eval(
            "let css = await dioxus.recv();
            let style = document.getElementById('dx-preview-theme-overrides');
            if (css === null) {
              if (style) style.remove();
              return;
            }
            if (!style) {
              style = document.createElement('style');
              style.id = 'dx-preview-theme-overrides';
              document.head.appendChild(style);
            }
            style.textContent = css;",
        );
        let _ = eval.send(css_for_dom);
    }));

    let in_iframe = Route::in_iframe().unwrap_or_default();

    rsx! {
      {children}
      if !in_iframe {
        ThemeStudio {}
      }
    }
}

#[component]
pub(crate) fn ThemePage() -> Element {
    let mut ctx = use_theme_customizer();

    rsx! {
      main { class: "dx-home-page", role: "main",
        section { class: "dx-home-section",
          header { class: "dx-section-header",
            span { class: "dx-section-eyebrow", "Theme" }
            h1 { class: "dx-section-title", "Theme studio" }
            p { class: "dx-section-summary",
              "Edit the full token set with structured controls instead of a fragile subset. Light and dark values stay paired, and the current page updates at the document root."
            }
          }
          div { class: "dx-theme-page-actions",
            div { class: "dx-theme-page-button-row",
              Button { onclick: move |_| ctx.open.set(true), "Open theme studio" }
              Link { to: Route::docs(),
                Button { variant: ButtonVariant::Outline, "Browse docs" }
              }
              Link { to: Route::demos(),
                Button { variant: ButtonVariant::Outline, "Browse demos" }
              }
            }
            p { class: "dx-theme-page-note",
              "Reset removes the injected override stylesheet and clears persisted edits."
            }
          }
        }
      }
    }
}

#[component]
fn ThemeStudio() -> Element {
    let mut ctx = use_theme_customizer();
    let tokens = (ctx.tokens)();
    let css_output = build_theme_css(&tokens);
    let copy_css = css_output.clone();

    rsx! {
      Button {
        size: ButtonSize::Lg,
        class: "dx-theme-studio-toggle",
        onclick: move |_| ctx.open.set(!(ctx.open)()),
        if (ctx.open)() {
          "Close theme studio"
        } else {
          "Open theme studio"
        }
      }

      if (ctx.open)() {
        aside {
          aria_label: "Theme studio",
          class: "dx-theme-studio-panel",
          div { class: "dx-theme-studio-content",
            div { class: "dx-theme-studio-header",
              div { class: "dx-theme-studio-heading",
                h2 { class: "dx-theme-studio-title", "Theme studio" }
                p { class: "dx-theme-studio-summary",
                  "Color pickers and sliders are used where the token type supports them. Derived expressions stay editable as text."
                }
              }
              Button {
                variant: ButtonVariant::Ghost,
                size: ButtonSize::Sm,
                onclick: move |_| ctx.open.set(false),
                "Close"
              }
            }

            div { class: "dx-theme-studio-toolbar",
              Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| ctx.tokens.set(default_theme_tokens().to_vec()),
                "Reset theme"
              }
              Button {
                variant: ButtonVariant::Secondary,
                onclick: move |_| {
                    let eval = document::eval(
                        "let value = await dioxus.recv(); navigator.clipboard.writeText(value);",
                    );
                    let _ = eval.send(copy_css.clone());
                },
                "Copy CSS"
              }
            }

            Tabs {
              default_value: "tokens",
              variant: TabsVariant::Ghost,
              width: "100%",
              TabList { aria_label: "Theme studio panels",
                TabTrigger { value: "tokens", index: 0usize, "Tokens" }
                TabTrigger { value: "css", index: 1usize, "CSS" }
              }
              TabContent { value: "tokens", index: 0usize,
                div { class: "dx-theme-section-list",
                  for section in THEME_SECTIONS {
                    ThemeSection {
                      section,
                      tokens: tokens.clone(),
                    }
                  }
                }
              }
              TabContent { value: "css", index: 1usize,
                Textarea {
                  variant: TextareaVariant::Default,
                  value: css_output,
                  readonly: true,
                  class: "dx-theme-css-output",
                }
              }
            }
          }
        }
      }
    }
}

#[component]
fn ThemeSection(section: &'static str, tokens: Vec<ThemeToken>) -> Element {
    let ctx = use_theme_customizer();

    rsx! {
      section { class: "dx-theme-section-card",
        h3 { class: "dx-theme-section-title", "{section}" }
        div { class: "dx-theme-token-list",
          for (index , token) in tokens.iter().enumerate().filter(|(_, token)| token.section == section) {
            ThemeTokenField {
              index,
              token: token.clone(),
              tokens_signal: ctx.tokens,
            }
          }
        }
      }
    }
}

#[component]
fn ThemeTokenField(
    index: usize,
    token: ThemeToken,
    tokens_signal: Signal<Vec<ThemeToken>>,
) -> Element {
    let name = token.name.clone();
    let current_light = token_display_value(&token, false);
    let current_dark = token_display_value(&token, true);
    let mut light_draft = use_signal(|| current_light.clone());
    let mut dark_draft = use_signal(|| current_dark.clone());
    let mut light_last_synced = use_signal(|| current_light.clone());
    let mut dark_last_synced = use_signal(|| current_dark.clone());

    use_effect(use_reactive!(|current_light| {
        let mut last_synced = light_last_synced();
        let mut draft = light_draft();
        if sync_draft_with_committed_value(&mut last_synced, &mut draft, &current_light) {
            light_last_synced.set(last_synced);
            light_draft.set(draft);
        }
    }));

    use_effect(use_reactive!(|current_dark| {
        let mut last_synced = dark_last_synced();
        let mut draft = dark_draft();
        if sync_draft_with_committed_value(&mut last_synced, &mut draft, &current_dark) {
            dark_last_synced.set(last_synced);
            dark_draft.set(draft);
        }
    }));

    match token.mode {
        TokenMode::Dual { light, dark } => {
            let light_is_picker = is_hex_color(&light);
            let dark_is_picker = is_hex_color(&dark);
            rsx! {
              div { class: "dx-theme-token-field",
                div { class: "dx-theme-token-controls-dual",
                  span { class: "dx-theme-token-name", "--{name}" }
                  div { class: "dx-theme-token-branch",
                    if light_is_picker {
                      ColorInput {
                        size: InputSize::Sm,
                        label: rsx! { "Light" },
                        color: color_from_hex(&light)
                            .unwrap_or_else(|| Color::new(43, 127, 255).into_format::<f64>().into_color()),
                        on_color_change: move |value| {
                            let mut signal = tokens_signal;
                            update_dual_branch(&mut signal, index, false, hex_from_color(value));
                        },
                      }
                    } else {
                      TextInput {
                        label: rsx! { "Light" },
                        value: "{light_draft}",
                        oninput: move |event: FormEvent| {
                            light_draft.set(event.value());
                        },
                        onchange: move |event: FormEvent| {
                            let mut signal = tokens_signal;
                            update_dual_branch(&mut signal, index, false, event.value());
                        },
                      }
                    }
                  }

                  div { class: "dx-theme-token-branch",
                    if dark_is_picker {
                      ColorInput {
                        label: rsx! { "Dark" },
                        color: color_from_hex(&dark)
                            .unwrap_or_else(|| Color::new(43, 127, 255).into_format::<f64>().into_color()),
                        on_color_change: move |value| {
                            let mut signal = tokens_signal;
                            update_dual_branch(&mut signal, index, true, hex_from_color(value));
                        },
                      }
                    } else {
                      TextInput {
                        label: rsx! { "Dark" },
                        value: "{dark_draft}",
                        oninput: move |event: FormEvent| {
                            dark_draft.set(event.value());
                        },
                        onchange: move |event: FormEvent| {
                            let mut signal = tokens_signal;
                            update_dual_branch(&mut signal, index, true, event.value());
                        },
                      }
                    }
                  }
                }
              }
            }
        }
        TokenMode::Single { value } => {
            if let Some((min, max, step, suffix, current)) = slider_spec(&name, &value) {
                rsx! {
                  div { class: "dx-theme-token-field",
                    div { class: "dx-theme-token-header",
                      span { class: "dx-theme-token-name", "--{name}" }
                      span { class: "dx-theme-token-meta", "{current:.2}{suffix}" }
                    }
                    Slider {
                      label: name.clone(),
                      value: current,
                      min,
                      max,
                      step,
                      on_value_change: move |next| {
                          let mut signal = tokens_signal;
                          update_single_value(&mut signal, index, format!("{next:.4}{suffix}"));
                      },
                    }
                    TextInput {
                      value: "{light_draft}",
                      oninput: move |event: FormEvent| {
                          light_draft.set(event.value());
                      },
                      onchange: move |event: FormEvent| {
                          let mut signal = tokens_signal;
                          update_single_value(&mut signal, index, event.value());
                      },
                    }
                  }
                }
            } else {
                rsx! {
                  TextInput {
                    label: rsx! { "--{name}" },
                    value: "{light_draft}",
                    oninput: move |event: FormEvent| {
                        light_draft.set(event.value());
                    },
                    onchange: move |event: FormEvent| {
                        let mut signal = tokens_signal;
                        update_single_value(&mut signal, index, event.value());
                    },
                  }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_tokens_do_not_emit_override_css() {
        assert_eq!(compute_theme_override_css(default_theme_tokens()), None);
    }

    #[test]
    fn changed_tokens_emit_override_css() {
        let mut tokens = default_theme_tokens().to_vec();
        let TokenMode::Single { value } = &mut tokens
            .iter_mut()
            .find(|token| token.name == "radius")
            .expect("default theme should include radius")
            .mode
        else {
            panic!("radius token should be single-valued");
        };
        *value = "0.875rem".to_string();

        assert!(compute_theme_override_css(&tokens).is_some());
    }

    #[test]
    fn restoring_default_color_with_different_hex_case_clears_override() {
        let mut tokens = default_theme_tokens().to_vec();
        let TokenMode::Dual { light, .. } = &mut tokens
            .iter_mut()
            .find(|token| token.name == "accent")
            .expect("default theme should include accent")
            .mode
        else {
            panic!("accent token should be dual-valued");
        };
        *light = "#2B7FFF".to_string();

        assert_eq!(compute_theme_override_css(&tokens), None);
    }

    #[test]
    fn restoring_default_numeric_value_with_equivalent_format_clears_override() {
        let mut tokens = default_theme_tokens().to_vec();
        let TokenMode::Single { value } = &mut tokens
            .iter_mut()
            .find(|token| token.name == "control-height-sm")
            .expect("default theme should include control-height-sm")
            .mode
        else {
            panic!("control-height-sm token should be single-valued");
        };
        *value = "1.5000rem".to_string();

        assert_eq!(compute_theme_override_css(&tokens), None);
    }

    #[test]
    fn malformed_numeric_value_does_not_normalize_to_default() {
        let mut tokens = default_theme_tokens().to_vec();
        let TokenMode::Single { value } = &mut tokens
            .iter_mut()
            .find(|token| token.name == "control-height-sm")
            .expect("default theme should include control-height-sm")
            .mode
        else {
            panic!("control-height-sm token should be single-valued");
        };
        *value = "1.5rem/*comment*/".to_string();

        assert!(compute_theme_override_css(&tokens).is_some());
    }

    #[test]
    fn draft_sync_does_not_overwrite_active_edits_without_token_change() {
        let mut last_synced = "#111111".to_string();
        let mut draft = "#222222".to_string();

        let changed = sync_draft_with_committed_value(&mut last_synced, &mut draft, "#111111");

        assert!(!changed);
        assert_eq!(last_synced, "#111111");
        assert_eq!(draft, "#222222");
    }

    #[test]
    fn draft_sync_updates_after_committed_token_change() {
        let mut last_synced = "#111111".to_string();
        let mut draft = "#222222".to_string();

        let changed = sync_draft_with_committed_value(&mut last_synced, &mut draft, "#333333");

        assert!(changed);
        assert_eq!(last_synced, "#333333");
        assert_eq!(draft, "#333333");
    }
}
