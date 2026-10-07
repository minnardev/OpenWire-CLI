#![allow(dead_code)]

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::state::{AppState, DspDto};
use super::theme::*;

#[derive(Default)]
pub struct DspViewState {
    pub selected_param: usize,
}

pub const DSP_PARAM_COUNT: usize = 16;

pub enum ParamKind {
    Toggle(bool),
    Float { val: f32, min: f32, max: f32, step: f32, unit: &'static str },
}

pub fn get_dsp_params(dto: &DspDto, monitor_enabled: bool) -> Vec<(&'static str, &'static str, ParamKind)> {
    vec![
        // Pitch & Formant
        ("Pitch Shift", "Сдвиг высоты тона (вкл/выкл)", ParamKind::Toggle(dto.pitch_enabled)),
        ("Pitch Semitones", "Полутоны высоты тона", ParamKind::Float { val: dto.pitch_semitones, min: -12.0, max: 12.0, step: 0.5, unit: "st" }),
        ("Formant Shift", "Формантный сдвиг тембра", ParamKind::Float { val: dto.formant_shift, min: 0.5, max: 2.0, step: 0.05, unit: "x" }),
        // Equalizer
        ("EQ Low Shelf", "Низкие частоты (бас)", ParamKind::Float { val: dto.eq_low_db, min: -24.0, max: 24.0, step: 1.0, unit: "dB" }),
        ("EQ Mid Gain", "Средние частоты (усиление)", ParamKind::Float { val: dto.eq_mid_gain_db, min: -24.0, max: 24.0, step: 1.0, unit: "dB" }),
        ("EQ Mid Freq", "Центральная частота СЧ", ParamKind::Float { val: dto.eq_mid_freq, min: 200.0, max: 8000.0, step: 100.0, unit: "Hz" }),
        ("EQ High Shelf", "Высокие частоты (яркость)", ParamKind::Float { val: dto.eq_high_db, min: -24.0, max: 24.0, step: 1.0, unit: "dB" }),
        // Radio FX
        ("Radio FX", "Эффект рации / рацийный перегруз", ParamKind::Toggle(dto.radio_enabled)),
        ("Radio Drive", "Насыщение / дисторшн рации", ParamKind::Float { val: dto.radio_drive, min: 0.0, max: 1.0, step: 0.05, unit: "" }),
        ("Radio Noise", "Шумовой фон рации", ParamKind::Float { val: dto.radio_noise_db, min: -90.0, max: -20.0, step: 2.0, unit: "dB" }),
        // Noise Gate
        ("Noise Gate", "Шумоподавление (вкл/выкл)", ParamKind::Toggle(dto.noise_gate_enabled)),
        ("Gate Threshold", "Порог срабатывания шумодава", ParamKind::Float { val: dto.noise_gate_db, min: -90.0, max: 0.0, step: 2.0, unit: "dB" }),
        // Ducking
        ("Mic Ducking", "Приглушение микрофона при падах", ParamKind::Toggle(dto.ducking_enabled)),
        ("Duck Attenuation", "Глубина приглушения", ParamKind::Float { val: dto.duck_atten_db, min: -40.0, max: 0.0, step: 1.0, unit: "dB" }),
        // Levels & Monitoring
        ("Hear Myself", "Слышать себя в наушниках", ParamKind::Toggle(monitor_enabled)),
        ("Stream Volume", "Общая громкость в микрофон", ParamKind::Float { val: dto.stream_volume, min: 0.0, max: 2.0, step: 0.05, unit: "x" }),
    ]
}

pub fn render_dsp(
    f: &mut Frame,
    area: Rect,
    state: &AppState,
    view_state: &DspViewState,
) {
    let dto = state.dsp_dto();
    let monitor_enabled = state.settings.read().map(|s| s.monitor_enabled).unwrap_or(true);
    let params = get_dsp_params(&dto, monitor_enabled);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(12),   // Params list
            Constraint::Length(4), // Controls & Shortcuts
        ])
        .split(area);

    let active_preset_name = state.active_preset.read()
        .ok()
        .and_then(|n| n.clone())
        .unwrap_or_else(|| "Обычный голос".to_string());

    let mut lines = Vec::new();

    for (idx, (name, desc, kind)) in params.iter().enumerate() {
        let is_selected = idx == view_state.selected_param;
        let prefix = if is_selected { " ▸ " } else { "   " };

        let (value_str, bar_str) = match kind {
            ParamKind::Toggle(on) => {
                let badge = if *on {
                    Span::styled("[ ВКЛ / ON ]", Style::default().fg(GREEN_PLAY).add_modifier(Modifier::BOLD))
                } else {
                    Span::styled("[ ВЫКЛ / OFF ]", Style::default().fg(FG_MUTED))
                };
                (badge, "".to_string())
            }
            ParamKind::Float { val, min, max, unit, .. } => {
                let norm = ((val - min) / (max - min)).clamp(0.0, 1.0);
                let bar_width: usize = 16;
                let filled = (norm * bar_width as f32).round() as usize;
                let bar: String = "█".repeat(filled) + &"░".repeat(bar_width.saturating_sub(filled));
                let text = format!("{:>6.1} {:<3}", val, unit);
                (
                    Span::styled(format!("{text}  [{bar}]"), Style::default().fg(ORANGE_LIGHT)),
                    "".to_string(),
                )
            }
        };

        let row_style = if is_selected {
            Style::default().fg(Color::Black).bg(ORANGE_LIGHT).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(FG_TEXT)
        };

        lines.push(Line::from(vec![
            Span::styled(format!("{prefix}{:<18}", name), row_style),
            Span::raw(" "),
            value_str,
            Span::raw("  "),
            Span::styled(format!("— {desc}{bar_str}"), Style::default().fg(FG_MUTED)),
        ]));
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(style_border_active())
        .title(format!(" 🎛️ DSP Процессор Голоса (Текущий пресет: \"{}\") ", active_preset_name));

    let list_p = Paragraph::new(lines).block(block);
    f.render_widget(list_p, chunks[0]);

    // Bottom action help
    let help_line = Line::from(vec![
        Span::styled(" [↑/↓] ", style_key_badge()),
        Span::raw("Выбор параметра  "),
        Span::styled(" [←/→] ", style_key_badge()),
        Span::raw("Настройка значения  "),
        Span::styled(" [Enter/Space] ", style_key_badge()),
        Span::raw("Переключить ВКЛ/ВЫКЛ  "),
        Span::styled(" [R] ", style_key_badge()),
        Span::raw("Сброс эффектов (Чистый голос)  "),
        Span::styled(" [M] ", style_key_badge()),
        Span::raw("Слышать себя"),
    ]);

    let help_p = Paragraph::new(help_line).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(style_border_inactive())
            .title(" Управление эффектами "),
    );
    f.render_widget(help_p, chunks[1]);
}
