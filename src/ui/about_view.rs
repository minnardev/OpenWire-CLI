use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::banner::ascii_art_widget;
use super::theme::*;

pub fn render_about(f: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(55), // Full Orange ASCII Art Logo
            Constraint::Percentage(45), // Info, Features & Shortcuts
        ])
        .split(area);

    // Left: The user's exact Orange ASCII Art Logo
    let ascii_text = ascii_art_widget();
    let logo_block = Block::default()
        .borders(Borders::ALL)
        .border_style(style_border_active())
        .title(" 📻 OpenWire Logo ");
    let logo_p = Paragraph::new(ascii_text)
        .block(logo_block)
        .alignment(Alignment::Center);
    f.render_widget(logo_p, chunks[0]);

    // Right: Info and Hotkeys
    let info_lines = vec![
        Line::from(vec![
            Span::styled("OpenWire CLI / TUI ", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
            Span::styled("v0.1.0", Style::default().fg(FG_MUTED)),
        ]),
        Line::from(Span::styled("Native soundpad and real-time DSP voice effects for Linux", Style::default().fg(CYAN_INFO))),
        Line::raw(""),
        Line::from(vec![
            Span::styled("⚡ Архитектура: ", Style::default().fg(ORANGE_LIGHT).add_modifier(Modifier::BOLD)),
            Span::styled("Чистый Rust (без WebKit / Tauri / Node.js)", Style::default().fg(FG_TEXT)),
        ]),
        Line::from(vec![
            Span::styled("🎧 Аудиостек:   ", Style::default().fg(ORANGE_LIGHT).add_modifier(Modifier::BOLD)),
            Span::styled("Linux PipeWire + LibSPA низколатентный граф", Style::default().fg(FG_TEXT)),
        ]),
        Line::from(vec![
            Span::styled("💾 Пресеты:     ", Style::default().fg(ORANGE_LIGHT).add_modifier(Modifier::BOLD)),
            Span::styled("Полная совместимость с TOML-пресетами OpenWire", Style::default().fg(FG_TEXT)),
        ]),
        Line::raw(""),
        Line::from(Span::styled("─── Горячие клавиши навигации ───", Style::default().fg(ORANGE_DARK).add_modifier(Modifier::BOLD))),
        Line::from(vec![
            Span::styled("  [F1..F5 / 1..5]  ", style_key_badge()),
            Span::raw("Переключение вкладок"),
        ]),
        Line::from(vec![
            Span::styled("  [Tab]            ", style_key_badge()),
            Span::raw("Следующая вкладка"),
        ]),
        Line::from(vec![
            Span::styled("  [Пробел / Enter] ", style_key_badge()),
            Span::raw("Воспроизведение / Действие"),
        ]),
        Line::from(vec![
            Span::styled("  [Esc]            ", style_key_badge()),
            Span::raw("ПАНИКА: заглушить все звуки"),
        ]),
        Line::from(vec![
            Span::styled("  [I]              ", style_key_badge()),
            Span::raw("Импортировать аудиофайл в пад"),
        ]),
        Line::from(vec![
            Span::styled("  [B]              ", style_key_badge()),
            Span::raw("Сменить звуковой банк"),
        ]),
        Line::from(vec![
            Span::styled("  [N]              ", style_key_badge()),
            Span::raw("Следующий голосовой пресет"),
        ]),
        Line::from(vec![
            Span::styled("  [R]              ", style_key_badge()),
            Span::raw("Сброс эффектов (Чистый голос)"),
        ]),
        Line::from(vec![
            Span::styled("  [M]              ", style_key_badge()),
            Span::raw("Слышать себя (Мониторинг)"),
        ]),
        Line::from(vec![
            Span::styled("  [Q]              ", style_key_badge()),
            Span::raw("Выход из OpenWire CLI"),
        ]),
        Line::raw(""),
        Line::from(Span::styled("─── Аудио-движок ───", Style::default().fg(ORANGE_DARK).add_modifier(Modifier::BOLD))),
        Line::from(Span::styled("• Сверхнизкая задержка (~1.3 мс при quantum 64)", Style::default().fg(FG_MUTED))),
        Line::from(Span::styled("• Виртуальный микрофон: openwire.virtual-mic", Style::default().fg(FG_MUTED))),
        Line::from(Span::styled("• Авто-приглушение микрофона при воспроизведении падов", Style::default().fg(FG_MUTED))),
    ];

    let info_block = Block::default()
        .borders(Borders::ALL)
        .border_style(style_border_inactive())
        .title(" Справка и управление ");
    let info_p = Paragraph::new(info_lines).block(info_block);
    f.render_widget(info_p, chunks[1]);
}
