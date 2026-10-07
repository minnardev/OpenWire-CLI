use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::state::AppState;
use super::theme::*;

fn format_vu_meter(peak: f32, width: usize) -> (Vec<Span<'static>>, String) {
    let db = if peak > 1e-5 {
        20.0 * peak.log10()
    } else {
        -90.0
    };
    let norm = ((db + 60.0) / 60.0).clamp(0.0, 1.0);
    let filled = (norm * width as f32).round() as usize;

    let mut spans = Vec::new();
    for i in 0..width {
        if i < filled {
            let ratio = i as f32 / width as f32;
            let col = if ratio > 0.85 {
                RED_STOP
            } else if ratio > 0.65 {
                ORANGE
            } else {
                GREEN_PLAY
            };
            spans.push(Span::styled("█", Style::default().fg(col)));
        } else {
            spans.push(Span::styled("░", Style::default().fg(Color::Rgb(50, 50, 60))));
        }
    }
    let db_str = format!("{:>5.1} dB", db);
    (spans, db_str)
}

pub fn render_monitor(f: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8), // VU Meters
            Constraint::Length(9), // PipeWire engine details
            Constraint::Min(6),    // Linux routing tips
        ])
        .split(area);

    let mic_peak = state.meters.mic();
    let out_peak = state.meters.out();
    let meter_width = (area.width as usize).saturating_sub(30).clamp(10, 48);

    let (mic_spans, mic_db) = format_vu_meter(mic_peak, meter_width);
    let (out_spans, out_db) = format_vu_meter(out_peak, meter_width);

    // Combine mic line
    let mut mic_line_spans = vec![
        Span::styled(" Mic In:  [", Style::default().fg(FG_TEXT)),
    ];
    mic_line_spans.extend(mic_spans);
    mic_line_spans.push(Span::styled(format!("] {mic_db}"), Style::default().fg(ORANGE_LIGHT)));

    let mut out_line_spans = vec![
        Span::styled(" Out Mic: [", Style::default().fg(FG_TEXT)),
    ];
    out_line_spans.extend(out_spans);
    out_line_spans.push(Span::styled(format!("] {out_db}"), Style::default().fg(ORANGE_LIGHT)));

    let vu_lines = vec![
        Line::raw(""),
        Line::from(mic_line_spans),
        Line::raw(""),
        Line::from(out_line_spans),
        Line::raw(""),
    ];

    let vu_block = Block::default()
        .borders(Borders::ALL)
        .border_style(style_border_active())
        .title(" Пиковые VU-индикаторы громкости в реальном времени ");
    let vu_p = Paragraph::new(vu_lines).block(vu_block);
    f.render_widget(vu_p, chunks[0]);

    // PipeWire engine details
    let version = state.meters.version();
    let quantum = state.meters.quantum();
    let latency = state.meters.latency_ms();

    let pw_lines = vec![
        Line::from(vec![
            Span::styled("Статус сервера:        ", Style::default().fg(FG_MUTED)),
            Span::styled(format!("PipeWire {version}"), Style::default().fg(ORANGE_LIGHT).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Текущий квант буфера:  ", Style::default().fg(FG_MUTED)),
            Span::styled(format!("{quantum} сэмплов"), Style::default().fg(CYAN_INFO)),
        ]),
        Line::from(vec![
            Span::styled("Оценочная задержка:    ", Style::default().fg(FG_MUTED)),
            Span::styled(format!("{latency:.2} мс @ 48 кГц"), Style::default().fg(GREEN_PLAY)),
        ]),
        Line::from(vec![
            Span::styled("Виртуальный микрофон:  ", Style::default().fg(FG_MUTED)),
            Span::styled("openwire.virtual-mic (доступен в Discord, OBS, играх)", Style::default().fg(FG_TEXT)),
        ]),
        Line::from(vec![
            Span::styled("Локальный мониторинг:  ", Style::default().fg(FG_MUTED)),
            Span::styled("openwire.monitor (вывод в наушники)", Style::default().fg(FG_TEXT)),
        ]),
    ];

    let pw_block = Block::default()
        .borders(Borders::ALL)
        .border_style(style_border_inactive())
        .title(" Параметры аудиосервера Linux PipeWire ");
    let pw_p = Paragraph::new(pw_lines).block(pw_block);
    f.render_widget(pw_p, chunks[1]);

    // Routing tips
    let tips_lines = vec![
        Line::from(vec![
            Span::styled("1. Discord / Telegram / Браузер: ", Style::default().fg(ORANGE_LIGHT).add_modifier(Modifier::BOLD)),
            Span::styled("Выберите устройство ввода \"OpenWire Virtual Mic\".", Style::default().fg(FG_TEXT)),
        ]),
        Line::from(vec![
            Span::styled("2. Графический роутинг:          ", Style::default().fg(ORANGE_LIGHT).add_modifier(Modifier::BOLD)),
            Span::styled("Используйте Helvum или Qpwgraph для управления связями узлов PipeWire.", Style::default().fg(FG_TEXT)),
        ]),
        Line::from(vec![
            Span::styled("3. Проверка работы PipeWire:    ", Style::default().fg(ORANGE_LIGHT).add_modifier(Modifier::BOLD)),
            Span::styled("Команды: pw-top, pw-cli info all, wpctl status.", Style::default().fg(FG_TEXT)),
        ]),
    ];

    let tips_block = Block::default()
        .borders(Borders::ALL)
        .border_style(style_border_inactive())
        .title(" Подсказки по настройке аудиопотоков в Linux ");
    let tips_p = Paragraph::new(tips_lines).block(tips_block);
    f.render_widget(tips_p, chunks[2]);
}
