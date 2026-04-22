use crate::config::TuiThemeConfig;
use crate::tui::app::state::App;
use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone)]
pub struct TuiTheme {
    pub background: Color,
    pub surface: Color,
    pub surface_alt: Color,
    pub text: Color,
    pub muted_text: Color,
    pub subtle_text: Color,
    pub border: Color,
    pub accent: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub info: Color,
    pub selection_bg: Color,
    pub selection_fg: Color,
}

impl Default for TuiTheme {
    fn default() -> Self {
        Self {
            background: Color::Rgb(17, 19, 24),
            surface: Color::Rgb(26, 31, 41),
            surface_alt: Color::Rgb(34, 41, 54),
            text: Color::Rgb(232, 236, 243),
            muted_text: Color::Rgb(182, 191, 203),
            subtle_text: Color::Rgb(122, 133, 150),
            border: Color::Rgb(127, 179, 213),
            accent: Color::Rgb(120, 194, 173),
            success: Color::Rgb(139, 195, 74),
            warning: Color::Rgb(230, 180, 80),
            danger: Color::Rgb(229, 115, 115),
            info: Color::Rgb(100, 181, 246),
            selection_bg: Color::Rgb(45, 79, 103),
            selection_fg: Color::Rgb(247, 251, 255),
        }
    }
}

impl TuiTheme {
    pub fn from_app(app: &App) -> Self {
        Self::from_config(app.config.tui_theme.as_ref())
    }

    pub fn from_config(config: Option<&TuiThemeConfig>) -> Self {
        let defaults = Self::default();
        let Some(config) = config else {
            return defaults;
        };

        Self {
            background: parse_theme_color(config.background.as_deref()).unwrap_or(defaults.background),
            surface: parse_theme_color(config.surface.as_deref()).unwrap_or(defaults.surface),
            surface_alt: parse_theme_color(config.surface_alt.as_deref()).unwrap_or(defaults.surface_alt),
            text: parse_theme_color(config.text.as_deref()).unwrap_or(defaults.text),
            muted_text: parse_theme_color(config.muted_text.as_deref()).unwrap_or(defaults.muted_text),
            subtle_text: parse_theme_color(config.subtle_text.as_deref()).unwrap_or(defaults.subtle_text),
            border: parse_theme_color(config.border.as_deref()).unwrap_or(defaults.border),
            accent: parse_theme_color(config.accent.as_deref()).unwrap_or(defaults.accent),
            success: parse_theme_color(config.success.as_deref()).unwrap_or(defaults.success),
            warning: parse_theme_color(config.warning.as_deref()).unwrap_or(defaults.warning),
            danger: parse_theme_color(config.danger.as_deref()).unwrap_or(defaults.danger),
            info: parse_theme_color(config.info.as_deref()).unwrap_or(defaults.info),
            selection_bg: parse_theme_color(config.selection_bg.as_deref()).unwrap_or(defaults.selection_bg),
            selection_fg: parse_theme_color(config.selection_fg.as_deref()).unwrap_or(defaults.selection_fg),
        }
    }

    pub fn selection_style(&self) -> Style {
        Style::default()
            .fg(self.selection_fg)
            .bg(self.selection_bg)
            .add_modifier(Modifier::BOLD)
    }

    pub fn active_field_style(&self) -> Style {
        Style::default().fg(self.text).bg(self.surface_alt)
    }

    pub fn inactive_field_style(&self) -> Style {
        Style::default().fg(self.muted_text)
    }

    pub fn tag_style_for_hex(&self, hex_color: &str) -> Style {
        let Some(color) = parse_theme_color(Some(hex_color)) else {
            return Style::default().fg(self.text);
        };

        let brightness = brightness(color);
        if (self.is_dark() && brightness < 0.30) || (!self.is_dark() && brightness > 0.80) {
            Style::default().fg(color).bg(self.surface)
        } else {
            let fg = if brightness > 0.60 { Color::Black } else { Color::White };
            Style::default().fg(fg).bg(color)
        }
    }

    pub fn priority_color(&self, priority: i64) -> Color {
        match priority {
            5 => self.danger,
            4 => Color::Rgb(255, 165, 0),
            3 => self.warning,
            2 => self.info,
            1 => self.accent,
            _ => self.text,
        }
    }

    pub fn due_date_color(&self, diff_days: i64) -> Color {
        if diff_days < 0 {
            self.danger
        } else if diff_days == 0 {
            self.warning
        } else if diff_days <= 3 {
            self.info
        } else {
            self.text
        }
    }

    fn is_dark(&self) -> bool {
        brightness(self.background) < 0.5
    }
}

pub fn parse_theme_color(input: Option<&str>) -> Option<Color> {
    let input = input?.trim();
    let lowered = input.to_ascii_lowercase();

    match lowered.as_str() {
        "black" => return Some(Color::Black),
        "red" => return Some(Color::Red),
        "green" => return Some(Color::Green),
        "yellow" => return Some(Color::Yellow),
        "blue" => return Some(Color::Blue),
        "magenta" => return Some(Color::Magenta),
        "cyan" => return Some(Color::Cyan),
        "gray" | "grey" => return Some(Color::Gray),
        "darkgray" | "darkgrey" => return Some(Color::DarkGray),
        "white" => return Some(Color::White),
        _ => {}
    }

    let hex = input.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(Color::Rgb(r, g, b))
}

fn brightness(color: Color) -> f32 {
    let (r, g, b) = match color {
        Color::Rgb(r, g, b) => (r as f32, g as f32, b as f32),
        Color::Black => (0.0, 0.0, 0.0),
        Color::White => (255.0, 255.0, 255.0),
        Color::Gray => (128.0, 128.0, 128.0),
        Color::DarkGray => (64.0, 64.0, 64.0),
        Color::Red => (255.0, 0.0, 0.0),
        Color::Green => (0.0, 255.0, 0.0),
        Color::Yellow => (255.0, 255.0, 0.0),
        Color::Blue => (0.0, 0.0, 255.0),
        Color::Magenta => (255.0, 0.0, 255.0),
        Color::Cyan => (0.0, 255.0, 255.0),
        _ => (127.0, 127.0, 127.0),
    };

    (r * 0.299 + g * 0.587 + b * 0.114) / 255.0
}