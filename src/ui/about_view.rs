use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::banner::ascii_art_widget;
use super::theme::*;

pub fn render_about(f: &mut Frame, area: Rect) {
    let (logo_rect, info_rect) = if area.width >= 96 {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(56), // Logo width: 52 chars + borders
                Constraint::Min(36),   // Info & shortcuts
            ])
            .split(area);
        (chunks[0], chunks[1])
    } else {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(28),
                Constraint::Min(10),
            ])
            .split(area);
        (chunks[0], chunks[1])
    };

    // Left / Top: Non-stretched orange ASCII logo
    let ascii_text = ascii_art_widget();
    let logo_block = Block::default()
        .borders(Borders::ALL)
        .border_style(style_border_active())
        .title(" OpenWire ");
    let logo_p = Paragraph::new(ascii_text)
        .block(logo_block)
        .alignment(Alignment::Center);
    f.render_widget(logo_p, logo_rect);

    // Right / Bottom: Clean info and controls (without emojis or unnecessary details)
    let info_lines = vec![
        Line::from(vec![
            Span::styled("OpenWire CLI", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(Span::styled("Саундпад и эффекты голоса для Linux", Style::default().fg(CYAN_INFO))),
        Line::raw(""),
        Line::from(Span::styled("Пресеты голоса:", Style::default().fg(ORANGE_LIGHT).add_modifier(Modifier::BOLD))),
        Line::from(Span::styled("  Конфигурации эффектов хранятся в формате TOML.", Style::default().fg(FG_TEXT))),
        Line::from(Span::styled("  Можно переключать пресеты (N), сохранять текущие (S)", Style::default().fg(FG_TEXT))),
        Line::from(Span::styled("  или возвращаться к чистому голосу (R).", Style::default().fg(FG_TEXT))),
        Line::raw(""),
        Line::from(Span::styled("Горячие клавиши:", Style::default().fg(ORANGE_LIGHT).add_modifier(Modifier::BOLD))),
        Line::from(vec![
            Span::styled("  [1..5 / Tab]     ", style_key_badge()),
            Span::raw("Вкладки (Пады, Эффекты, Пресеты, Мониторы)"),
        ]),
        Line::from(vec![
            Span::styled("  [Пробел / Enter] ", style_key_badge()),
            Span::raw("Воспроизведение пада / выбор"),
        ]),
        Line::from(vec![
            Span::styled("  [S]              ", style_key_badge()),
            Span::raw("Остановить выбранный пад"),
        ]),
        Line::from(vec![
            Span::styled("  [Esc]            ", style_key_badge()),
            Span::raw("Паника — заглушить все звуки"),
        ]),
        Line::from(vec![
            Span::styled("  [I]              ", style_key_badge()),
            Span::raw("Импорт аудиофайла в выбранный пад"),
        ]),
        Line::from(vec![
            Span::styled("  [B]              ", style_key_badge()),
            Span::raw("Сменить банк звуков"),
        ]),
        Line::from(vec![
            Span::styled("  [M]              ", style_key_badge()),
            Span::raw("Слышать себя / переключить режим"),
        ]),
        Line::from(vec![
            Span::styled("  [Q]              ", style_key_badge()),
            Span::raw("Выход из программы"),
        ]),
    ];

    let info_block = Block::default()
        .borders(Borders::ALL)
        .border_style(style_border_inactive())
        .title(" Справка ");
    let info_p = Paragraph::new(info_lines).block(info_block);
    f.render_widget(info_p, info_rect);
}
