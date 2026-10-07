use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::soundboard::bank::Pad;
use crate::state::AppState;
use super::theme::*;

pub struct SoundboardState {
    pub selected_bank: usize,
    pub selected_slot: usize,
}

impl Default for SoundboardState {
    fn default() -> Self {
        Self {
            selected_bank: 0,
            selected_slot: 0,
        }
    }
}

pub fn render_soundboard(
    f: &mut Frame,
    area: Rect,
    state: &AppState,
    sb_state: &SoundboardState,
) {
    let board = state.sound.snapshot();
    let bank_count = board.banks.len();
    let current_bank_idx = sb_state.selected_bank.min(bank_count.saturating_sub(1));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Bank tabs
            Constraint::Min(10),   // Pad grid
            Constraint::Length(5), // Selected pad info & shortcuts
        ])
        .split(area);

    // 1. Bank tabs
    let mut bank_spans = Vec::new();
    for (idx, bank) in board.banks.iter().enumerate() {
        let is_current = idx == current_bank_idx;
        let style = if is_current {
            Style::default().fg(Color::Black).bg(ORANGE).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(FG_MUTED)
        };
        bank_spans.push(Span::raw(" "));
        bank_spans.push(Span::styled(format!(" [{}: {}] ", idx + 1, bank.name), style));
        bank_spans.push(Span::raw(" "));
    }
    let banks_bar = Paragraph::new(Line::from(bank_spans)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(style_border_inactive())
            .title(" 📁 Аудио-банки (B: сменить банк, 1-9: быстрый выбор) "),
    );
    f.render_widget(banks_bar, chunks[0]);

    // 2. Pad Grid (3 columns x 5 rows)
    let current_bank = board.banks.get(current_bank_idx);
    let pads: &[Pad] = current_bank.map(|b| b.pads.as_slice()).unwrap_or(&[]);

    // Split into 5 rows
    let row_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(1, 5),
            Constraint::Ratio(1, 5),
            Constraint::Ratio(1, 5),
            Constraint::Ratio(1, 5),
            Constraint::Ratio(1, 5),
        ])
        .split(chunks[1]);

    for row in 0..5 {
        let col_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ])
            .split(row_chunks[row]);

        for col in 0..3 {
            let slot_idx = row * 3 + col;
            let pad = pads.get(slot_idx);
            let is_selected = slot_idx == sb_state.selected_slot;

            let border_style = if is_selected {
                Style::default().fg(ORANGE_LIGHT).add_modifier(Modifier::BOLD)
            } else {
                style_border_inactive()
            };

            let (title, path_str, hotkey_str, mode_str) = match pad {
                Some(p) => {
                    let has_file = p.path.is_some();
                    let name = if p.name.trim().is_empty() {
                        if has_file { "Аудиофайл".to_string() } else { "— Пустой пад —".to_string() }
                    } else {
                        p.name.clone()
                    };
                    let file_label = p.path.as_ref()
                        .and_then(|path| path.file_name())
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_else(|| "Нет файла".to_string());
                    let hotkey = p.hotkey.as_deref().unwrap_or("—");
                    let mode = format!("{} | Vol: {:.0}%", p.play_mode, p.stream_volume * 100.0);
                    (name, file_label, hotkey.to_string(), mode)
                }
                None => ("— Пусто —".to_string(), "Нет файла".to_string(), "—".to_string(), "".to_string()),
            };

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(format!(" Пад #{:02} [{}] ", slot_idx + 1, hotkey_str));

            let lines = vec![
                Line::from(vec![
                    Span::styled(
                        if is_selected { format!(" ▸ {title}") } else { format!("   {title}") },
                        if is_selected {
                            Style::default().fg(ORANGE_LIGHT).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(FG_TEXT)
                        },
                    ),
                ]),
                Line::from(vec![
                    Span::styled(format!("   📎 {path_str}"), Style::default().fg(FG_MUTED)),
                ]),
                Line::from(vec![
                    Span::styled(format!("   ⚙ {mode_str}"), Style::default().fg(CYAN_INFO)),
                ]),
            ];

            let pad_p = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });
            f.render_widget(pad_p, col_chunks[col]);
        }
    }

    // 3. Selected Pad Action Bar
    let selected_pad = pads.get(sb_state.selected_slot);
    let selected_desc = match selected_pad {
        Some(p) => {
            let path_display = p.path.as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "файл не назначен".to_string());
            format!(
                "Выбран Пад #{}: \"{}\" | Путь: {} | Хоткей: [{}] | Режим: {} | Монитор: {:.0}% | Стрим: {:.0}%",
                sb_state.selected_slot + 1,
                p.name,
                path_display,
                p.hotkey.as_deref().unwrap_or("—"),
                p.play_mode,
                p.monitor_volume * 100.0,
                p.stream_volume * 100.0,
            )
        }
        None => "Пад не выбран".to_string(),
    };

    let controls_line = Line::from(vec![
        Span::styled(" [Пробел/Enter] ", style_key_badge()),
        Span::raw("Играть  "),
        Span::styled(" [S] ", style_key_badge()),
        Span::raw("Стоп  "),
        Span::styled(" [Esc] ", style_key_badge()),
        Span::raw("ПАНИКА (стоп всё)  "),
        Span::styled(" [I] ", style_key_badge()),
        Span::raw("Импорт  "),
        Span::styled(" [M] ", style_key_badge()),
        Span::raw("1-Shot/Loop  "),
        Span::styled(" [B] ", style_key_badge()),
        Span::raw("Банк  "),
        Span::styled(" [Del] ", style_key_badge()),
        Span::raw("Очистить"),
    ]);

    let footer_text = vec![
        Line::from(Span::styled(selected_desc, Style::default().fg(ORANGE_LIGHT))),
        controls_line,
    ];

    let footer = Paragraph::new(footer_text)
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(style_border_active())
                .title(" Управление выбранным падом "),
        );
    f.render_widget(footer, chunks[2]);
}
