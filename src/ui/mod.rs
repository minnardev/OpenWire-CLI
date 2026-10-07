pub mod about_view;
pub mod dsp_view;
pub mod monitor_view;
pub mod presets_view;
pub mod soundboard_view;
pub mod theme;

use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

use crate::state::AppState;
use about_view::render_about;
use dsp_view::{render_dsp, DspViewState};
use monitor_view::render_monitor;
use presets_view::{render_presets, PresetsViewState};
use soundboard_view::{render_soundboard, SoundboardState};
use theme::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTab {
    Soundboard,
    Dsp,
    Presets,
    Monitor,
    About,
}

impl ActiveTab {
    pub fn next(self) -> Self {
        match self {
            Self::Soundboard => Self::Dsp,
            Self::Dsp => Self::Presets,
            Self::Presets => Self::Monitor,
            Self::Monitor => Self::About,
            Self::About => Self::Soundboard,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Soundboard => Self::About,
            Self::Dsp => Self::Soundboard,
            Self::Presets => Self::Dsp,
            Self::Monitor => Self::Presets,
            Self::About => Self::Monitor,
        }
    }
}

pub struct UiState {
    pub current_tab: ActiveTab,
    pub soundboard: SoundboardState,
    pub dsp: DspViewState,
    pub presets: PresetsViewState,
    pub input_prompt: Option<InputPrompt>,
    pub status_notification: Option<(String, std::time::Instant)>,
}

pub struct InputPrompt {
    pub title: String,
    pub prompt: String,
    pub buffer: String,
    pub action: PromptAction,
}

pub enum PromptAction {
    ImportPadFile { bank: usize, slot: usize },
    SavePreset,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            current_tab: ActiveTab::Soundboard,
            soundboard: SoundboardState::default(),
            dsp: DspViewState::default(),
            presets: PresetsViewState::default(),
            input_prompt: None,
            status_notification: None,
        }
    }
}

pub fn render_ui(f: &mut Frame, state: &AppState, ui_state: &UiState) {
    let size = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Top Navigation Bar & Title
            Constraint::Min(10),   // Active tab contents
            Constraint::Length(2), // Bottom Global Status Bar
        ])
        .split(size);

    // 1. Top Header Tabs
    let tabs = [
        (ActiveTab::Soundboard, "[1] 🔊 Пады"),
        (ActiveTab::Dsp, "[2] 🎙️ Эффекты"),
        (ActiveTab::Presets, "[3] 🎭 Пресеты"),
        (ActiveTab::Monitor, "[4] 📊 Мониторы"),
        (ActiveTab::About, "[5] ℹ️ О программе"),
    ];

    let mut tab_spans = vec![
        Span::styled(" OpenWire CLI ", style_header()),
        Span::raw(" │ "),
    ];

    for (tab, label) in tabs {
        let is_active = ui_state.current_tab == tab;
        let style = if is_active {
            style_tab_active()
        } else {
            style_tab_inactive()
        };
        tab_spans.push(Span::raw(" "));
        tab_spans.push(Span::styled(format!(" {label} "), style));
        tab_spans.push(Span::raw(" "));
    }

    let top_bar = Paragraph::new(Line::from(tab_spans)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(style_border_active()),
    );
    f.render_widget(top_bar, chunks[0]);

    // 2. Tab Content
    match ui_state.current_tab {
        ActiveTab::Soundboard => {
            render_soundboard(f, chunks[1], state, &ui_state.soundboard);
        }
        ActiveTab::Dsp => {
            render_dsp(f, chunks[1], state, &ui_state.dsp);
        }
        ActiveTab::Presets => {
            render_presets(f, chunks[1], state, &ui_state.presets);
        }
        ActiveTab::Monitor => {
            render_monitor(f, chunks[1], state);
        }
        ActiveTab::About => {
            render_about(f, chunks[1]);
        }
    }

    // 3. Bottom Global Status Bar
    let active_preset = state.active_preset.read()
        .ok()
        .and_then(|n| n.clone())
        .unwrap_or_else(|| "Обычный голос".to_string());

    let monitor_on = state.settings.read().map(|s| s.monitor_enabled).unwrap_or(true);
    let mon_badge = if monitor_on {
        Span::styled(" [Монитор: ВКЛ] ", Style::default().fg(GREEN_PLAY).add_modifier(Modifier::BOLD))
    } else {
        Span::styled(" [Монитор: ВЫКЛ] ", Style::default().fg(FG_MUTED))
    };

    let notif_span = if let Some((msg, created)) = &ui_state.status_notification {
        if created.elapsed().as_secs() < 4 {
            Span::styled(format!(" 🔔 {msg} "), Style::default().fg(ORANGE_LIGHT).add_modifier(Modifier::BOLD))
        } else {
            Span::raw("")
        }
    } else {
        Span::raw("")
    };

    let status_line = Line::from(vec![
        Span::styled(" Пресет: ", Style::default().fg(FG_MUTED)),
        Span::styled(format!("\"{}\"", active_preset), Style::default().fg(ORANGE_LIGHT).add_modifier(Modifier::BOLD)),
        Span::raw(" │ "),
        mon_badge,
        Span::raw(" │ "),
        notif_span,
        Span::raw(" │ "),
        Span::styled(" [Tab] Вкладки  [Пробел] Играть  [Esc] Стоп всё  [Q] Выход ", Style::default().fg(FG_MUTED)),
    ]);

    let status_bar = Paragraph::new(status_line).alignment(Alignment::Left);
    f.render_widget(status_bar, chunks[2]);

    // 4. Modal Input Prompt (if active)
    if let Some(prompt) = &ui_state.input_prompt {
        render_input_modal(f, size, prompt);
    }
}

fn render_input_modal(f: &mut Frame, area: Rect, prompt: &InputPrompt) {
    let modal_width = 70.min(area.width.saturating_sub(4));
    let modal_height = 8.min(area.height.saturating_sub(2));

    let x = (area.width.saturating_sub(modal_width)) / 2;
    let y = (area.height.saturating_sub(modal_height)) / 2;
    let modal_rect = Rect::new(x, y, modal_width, modal_height);

    f.render_widget(Clear, modal_rect);

    let lines = vec![
        Line::styled(&prompt.prompt, Style::default().fg(FG_TEXT)),
        Line::raw(""),
        Line::from(vec![
            Span::styled(" ▸ ", Style::default().fg(ORANGE)),
            Span::styled(format!("{}_", prompt.buffer), Style::default().fg(ORANGE_LIGHT).add_modifier(Modifier::BOLD)),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled(" [Enter] ", style_key_badge()),
            Span::raw("Подтвердить   "),
            Span::styled(" [Esc] ", style_key_badge()),
            Span::raw("Отмена"),
        ]),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ORANGE).add_modifier(Modifier::BOLD))
        .title(format!(" {} ", prompt.title));

    let p = Paragraph::new(lines).block(block);
    f.render_widget(p, modal_rect);
}
