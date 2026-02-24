use std::{collections::HashMap, fmt, fs, path::Path};

use serde::Deserialize;

use crate::{Color, Modifier, Style};

#[derive(Clone, Debug, Default)]
pub struct Theme {
    tokens: HashMap<String, Style>,
}

impl Theme {
    pub fn from_json_str(input: &str) -> Result<Self, ThemeError> {
        let file: ThemeFile = serde_json::from_str(input)
            .map_err(|err| ThemeError::Parse(format!("invalid theme JSON: {err}")))?;

        let mut tokens = HashMap::with_capacity(file.tokens.len());
        for (name, spec) in file.tokens {
            tokens.insert(name, spec.into_style()?);
        }

        Ok(Self { tokens })
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ThemeError> {
        let path = path.as_ref();
        let data = fs::read_to_string(path)
            .map_err(|err| ThemeError::Io(format!("failed to read {}: {err}", path.display())))?;
        Self::from_json_str(&data)
    }

    pub fn style(&self, token: &str) -> Option<Style> {
        self.tokens.get(token).copied()
    }
}

#[derive(Debug)]
pub enum ThemeError {
    Io(String),
    Parse(String),
    Invalid(String),
}

impl fmt::Display for ThemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThemeError::Io(message) => write!(f, "{message}"),
            ThemeError::Parse(message) => write!(f, "{message}"),
            ThemeError::Invalid(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for ThemeError {}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ThemeFile {
    tokens: HashMap<String, StyleSpec>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StyleSpec {
    fg: Option<ColorSpec>,
    bg: Option<ColorSpec>,
    modifiers: Option<Vec<ModifierSpec>>,
}

impl StyleSpec {
    fn into_style(self) -> Result<Style, ThemeError> {
        let mut style = Style::new();

        if let Some(color) = self.fg {
            style = style.fg(color.into_color()?);
        }

        if let Some(color) = self.bg {
            style = style.bg(color.into_color()?);
        }

        if let Some(modifiers) = self.modifiers {
            for modifier in modifiers {
                style = style.modifier(modifier.into_modifier());
            }
        }

        Ok(style)
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ColorSpec {
    Default(DefaultColorSpec),
    Ansi(AnsiColorSpec),
    Rgb(RgbColorSpec),
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DefaultColorSpec {
    default: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AnsiColorSpec {
    ansi: u8,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RgbColorSpec {
    rgb: [u8; 3],
}

impl ColorSpec {
    fn into_color(self) -> Result<Color, ThemeError> {
        match self {
            ColorSpec::Default(DefaultColorSpec { default: true }) => Ok(Color::Default),
            ColorSpec::Default(DefaultColorSpec { default: false }) => Err(ThemeError::Invalid(
                "`default` color must be true".to_string(),
            )),
            ColorSpec::Ansi(AnsiColorSpec { ansi }) => Ok(Color::Ansi(ansi)),
            ColorSpec::Rgb(RgbColorSpec { rgb }) => Ok(Color::Rgb(rgb[0], rgb[1], rgb[2])),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ModifierSpec {
    Bold,
    Dim,
    Italic,
    Underline,
    Reverse,
}

impl ModifierSpec {
    fn into_modifier(self) -> Modifier {
        match self {
            ModifierSpec::Bold => Modifier::Bold,
            ModifierSpec::Dim => Modifier::Dim,
            ModifierSpec::Italic => Modifier::Italic,
            ModifierSpec::Underline => Modifier::Underline,
            ModifierSpec::Reverse => Modifier::Reverse,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Color, Modifier};

    use super::Theme;

    #[test]
    fn parses_theme_with_ansi_rgb_and_modifiers() {
        let input = r#"
        {
          "tokens": {
            "list.item": { "fg": { "ansi": 252 } },
            "list.selected": {
              "fg": { "rgb": [255, 255, 255] },
              "bg": { "ansi": 39 },
              "modifiers": ["bold", "underline"]
            }
          }
        }
        "#;

        let theme = Theme::from_json_str(input).expect("theme should parse");
        let selected = theme.style("list.selected").expect("token should exist");

        assert_eq!(selected.fg, Some(Color::Rgb(255, 255, 255)));
        assert_eq!(selected.bg, Some(Color::Ansi(39)));
        assert!(selected.modifiers.contains(Modifier::Bold));
        assert!(selected.modifiers.contains(Modifier::Underline));
    }

    #[test]
    fn token_lookup_returns_none_when_missing() {
        let input = r#"{ "tokens": {} }"#;
        let theme = Theme::from_json_str(input).expect("theme should parse");

        assert!(theme.style("unknown").is_none());
    }

    #[test]
    fn invalid_modifier_fails_strictly() {
        let input = r#"
        {
          "tokens": {
            "x": { "modifiers": ["blink"] }
          }
        }
        "#;

        assert!(Theme::from_json_str(input).is_err());
    }

    #[test]
    fn default_false_is_rejected() {
        let input = r#"
        {
          "tokens": {
            "x": { "fg": { "default": false } }
          }
        }
        "#;

        assert!(Theme::from_json_str(input).is_err());
    }

    #[test]
    fn missing_tokens_field_fails_strictly() {
        let input = r#"{ "palette": {} }"#;
        assert!(Theme::from_json_str(input).is_err());
    }

    #[test]
    fn unknown_style_field_fails_strictly() {
        let input = r#"
        {
          "tokens": {
            "x": { "fg": { "ansi": 2 }, "unknown": 1 }
          }
        }
        "#;

        assert!(Theme::from_json_str(input).is_err());
    }

    #[test]
    fn mixed_color_shape_fails_strictly() {
        let input = r#"
        {
          "tokens": {
            "x": { "fg": { "ansi": 2, "rgb": [1, 2, 3] } }
          }
        }
        "#;

        assert!(Theme::from_json_str(input).is_err());
    }

    #[test]
    fn missing_token_uses_fallback_at_call_site() {
        let input = r#"{ "tokens": {} }"#;
        let theme = Theme::from_json_str(input).expect("theme should parse");
        let fallback = Color::Ansi(99);

        let resolved = theme
            .style("statusbar.bg")
            .and_then(|s| s.fg)
            .unwrap_or(fallback);
        assert_eq!(resolved, fallback);
    }
}
