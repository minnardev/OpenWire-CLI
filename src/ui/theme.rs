use ratatui::style::{Color, Modifier, Style};

pub const ORANGE: Color = Color::Rgb(255, 140, 0);          // #FF8C00
pub const ORANGE_LIGHT: Color = Color::Rgb(255, 175, 55);   // #FFAF37
pub const ORANGE_DARK: Color = Color::Rgb(180, 85, 0);      // #B45500
pub const BG_DARK: Color = Color::Rgb(18, 18, 20);          // #121214
pub const BG_PANEL: Color = Color::Rgb(28, 28, 32);         // #1C1C20
pub const BG_HIGHLIGHT: Color = Color::Rgb(45, 45, 55);     // #2D2D37
pub const FG_TEXT: Color = Color::Rgb(230, 230, 235);       // #E6E6EB
pub const FG_MUTED: Color = Color::Rgb(140, 140, 150);      // #8C8C96
pub const GREEN_PLAY: Color = Color::Rgb(60, 210, 120);     // #3CD278
pub const RED_STOP: Color = Color::Rgb(240, 70, 70);        // #F04646
pub const CYAN_INFO: Color = Color::Rgb(80, 190, 240);      // #50BEF0

pub fn style_header() -> Style {
    Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)
}

pub fn style_tab_active() -> Style {
    Style::default()
        .fg(Color::Black)
        .bg(ORANGE)
        .add_modifier(Modifier::BOLD)
}

pub fn style_tab_inactive() -> Style {
    Style::default().fg(FG_MUTED)
}

pub fn style_border_active() -> Style {
    Style::default().fg(ORANGE)
}

pub fn style_border_inactive() -> Style {
    Style::default().fg(Color::Rgb(70, 70, 80))
}

pub fn style_item_selected() -> Style {
    Style::default()
        .fg(Color::Black)
        .bg(ORANGE_LIGHT)
        .add_modifier(Modifier::BOLD)
}

pub fn style_item_normal() -> Style {
    Style::default().fg(FG_TEXT)
}

pub fn style_status_playing() -> Style {
    Style::default().fg(GREEN_PLAY).add_modifier(Modifier::BOLD)
}

pub fn style_status_stopped() -> Style {
    Style::default().fg(FG_MUTED)
}

pub fn style_key_badge() -> Style {
    Style::default().fg(ORANGE_LIGHT).add_modifier(Modifier::BOLD)
}
