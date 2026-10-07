use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::state::AppState;
use super::theme::*;

#[derive(Default)]
pub struct PresetsViewState {
    pub selected_index: usize,
}

pub fn render_presets(
    f: &mut Frame,
    area: Rect,
    state: &AppState,
    view_state: &PresetsViewState,
) {
    let mut names = state.presets.list_presets().unwrap_or_default();
    if !names.contains(&"Обычный голос".to_string()) {
        names.insert(0, "Обычный голос".to_string());
    }

    let active_name = state.active_preset.read()
        .ok()
        .and_then(|n| n.clone())
        .unwrap_or_else(|| "Обычный голос".to_string());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(8),    // Presets list
            Constraint::Length(8), // Preset details
            Constraint::Length(4), // Shortcuts
        ])
        .split(area);

    let mut list_lines = Vec::new();

    for (idx, name) in names.iter().enumerate() {
        let is_selected = idx == view_state.selected_index;
        let is_active = name == &active_name;

        let prefix = if is_selected { " ▸ " } else { "   " };
        let status_badge = if is_active {
            Span::styled(" [АКТИВЕН] ", Style::default().fg(Color::Black).bg(ORANGE).add_modifier(Modifier::BOLD))
        } else {
            Span::styled("          ", Style::default().fg(FG_MUTED))
        };

        let style = if is_selected {
            Style::default().fg(Color::Black).bg(ORANGE_LIGHT).add_modifier(Modifier::BOLD)
        } else if is_active {
            Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(FG_TEXT)
        };

        list_lines.push(Line::from(vec![
            Span::styled(prefix, style),
            status_badge,
            Span::raw(" "),
            Span::styled(format!("{:<28}", name), style),
        ]));
    }

    let list_block = Block::default()
        .borders(Borders::ALL)
        .border_style(style_border_active())
        .title(format!(" 🎭 Доступные пресеты голоса ({}) ", names.len()));
    let list_p = Paragraph::new(list_lines).block(list_block);
    f.render_widget(list_p, chunks[0]);

    // Selected preset details
    let selected_name = names.get(view_state.selected_index).cloned().unwrap_or_default();
    let detail_lines = if selected_name == "Обычный голос" {
        vec![
            Line::from(vec![
                Span::styled("Пресет: ", Style::default().fg(FG_MUTED)),
                Span::styled("Обычный голос (Встроенный)", Style::default().fg(ORANGE_LIGHT).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("Описание: ", Style::default().fg(FG_MUTED)),
                Span::styled("Чистый голос без эффектов (DSP отключен)", Style::default().fg(FG_TEXT)),
            ]),
            Line::from(vec![
                Span::styled("Путь: ", Style::default().fg(FG_MUTED)),
                Span::styled("Встроенная конфигурация по умолчанию", Style::default().fg(FG_MUTED)),
            ]),
        ]
    } else if let Ok(preset) = state.presets.load_preset(&selected_name) {
        vec![
            Line::from(vec![
                Span::styled("Пресет: ", Style::default().fg(FG_MUTED)),
                Span::styled(preset.meta.name, Style::default().fg(ORANGE_LIGHT).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("Описание: ", Style::default().fg(FG_MUTED)),
                Span::styled(preset.meta.description.unwrap_or_else(|| "—".to_string()), Style::default().fg(FG_TEXT)),
            ]),
            Line::from(vec![
                Span::styled("Автор: ", Style::default().fg(FG_MUTED)),
                Span::styled(preset.meta.author.unwrap_or_else(|| "OpenWire".to_string()), Style::default().fg(FG_TEXT)),
            ]),
            Line::from(vec![
                Span::styled("Каталог: ", Style::default().fg(FG_MUTED)),
                Span::styled(state.presets.dir().display().to_string(), Style::default().fg(FG_MUTED)),
            ]),
        ]
    } else {
        vec![
            Line::from(Span::styled("Не удалось прочитать параметры пресета", Style::default().fg(RED_STOP))),
        ]
    };

    let details_block = Block::default()
        .borders(Borders::ALL)
        .border_style(style_border_inactive())
        .title(" Информация о пресете ");
    let details_p = Paragraph::new(detail_lines).block(details_block);
    f.render_widget(details_p, chunks[1]);

    // Shortcuts
    let shortcuts_line = Line::from(vec![
        Span::styled(" [Enter/Space] ", style_key_badge()),
        Span::raw("Применить пресет  "),
        Span::styled(" [N] ", style_key_badge()),
        Span::raw("Следующий пресет  "),
        Span::styled(" [S] ", style_key_badge()),
        Span::raw("Сохранить текущие эффекты как пресет  "),
        Span::styled(" [D] ", style_key_badge()),
        Span::raw("Удалить  "),
        Span::styled(" [R] ", style_key_badge()),
        Span::raw("Чистый голос"),
    ]);
    let shortcuts_p = Paragraph::new(shortcuts_line).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(style_border_inactive())
            .title(" Горячие клавиши "),
    );
    f.render_widget(shortcuts_p, chunks[2]);
}
