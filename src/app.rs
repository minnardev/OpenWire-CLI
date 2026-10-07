use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::backend::Backend;
use ratatui::Terminal;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

use crate::presets::voice_config::VoicePreset;
use crate::state::AppState;
use crate::ui::dsp_view::{get_dsp_params, ParamKind, DSP_PARAM_COUNT};
use crate::ui::*;

pub struct App {
    pub state: AppState,
    pub ui: UiState,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        let state = AppState::bootstrap()?;
        Ok(Self {
            state,
            ui: UiState::default(),
            should_quit: false,
        })
    }

    pub fn set_notification(&mut self, text: impl Into<String>) {
        self.ui.status_notification = Some((text.into(), Instant::now()));
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        let tick_rate = Duration::from_millis(50);
        let mut last_tick = Instant::now();

        while !self.should_quit {
            terminal.draw(|f| render_ui(f, &self.state, &self.ui))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_millis(0));

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key(key);
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }

        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        // If modal input prompt is active, route all input there
        if self.ui.input_prompt.is_some() {
            self.handle_modal_key(key);
            return;
        }

        // Global shortcuts
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.should_quit = true;
            return;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.should_quit = true;
                return;
            }
            KeyCode::Tab => {
                self.ui.current_tab = self.ui.current_tab.next();
                return;
            }
            KeyCode::BackTab => {
                self.ui.current_tab = self.ui.current_tab.prev();
                return;
            }
            KeyCode::F(1) | KeyCode::Char('1') => {
                self.ui.current_tab = ActiveTab::Soundboard;
                return;
            }
            KeyCode::F(2) | KeyCode::Char('2') => {
                self.ui.current_tab = ActiveTab::Dsp;
                return;
            }
            KeyCode::F(3) | KeyCode::Char('3') => {
                self.ui.current_tab = ActiveTab::Presets;
                return;
            }
            KeyCode::F(4) | KeyCode::Char('4') => {
                self.ui.current_tab = ActiveTab::Monitor;
                return;
            }
            KeyCode::F(5) | KeyCode::Char('5') => {
                self.ui.current_tab = ActiveTab::About;
                return;
            }
            _ => {}
        }

        // Dispatch based on active tab
        match self.ui.current_tab {
            ActiveTab::Soundboard => self.handle_soundboard_key(key),
            ActiveTab::Dsp => self.handle_dsp_key(key),
            ActiveTab::Presets => self.handle_presets_key(key),
            ActiveTab::Monitor => self.handle_monitor_key(key),
            ActiveTab::About => self.handle_about_key(key),
        }
    }

    fn handle_modal_key(&mut self, key: KeyEvent) {
        let Some(prompt) = self.ui.input_prompt.as_mut() else {
            return;
        };

        match key.code {
            KeyCode::Esc => {
                self.ui.input_prompt = None;
            }
            KeyCode::Enter => {
                let text = prompt.buffer.trim().to_string();
                let action = std::mem::replace(&mut prompt.action, PromptAction::SavePreset);
                self.ui.input_prompt = None;

                match action {
                    PromptAction::ImportPadFile { bank, slot } => {
                        if !text.is_empty() {
                            let path = PathBuf::from(&text);
                            match self.state.sound.import(bank, slot, path) {
                                Ok(pad) => {
                                    self.set_notification(format!("Пад #{}: импортирован \"{}\"", slot + 1, pad.name));
                                }
                                Err(err) => {
                                    self.set_notification(format!("Ошибка импорта: {err}"));
                                }
                            }
                        }
                    }
                    PromptAction::SavePreset => {
                        if !text.is_empty() {
                            let snap = self.state.dsp.snapshot();
                            let meta = crate::presets::voice_config::Meta {
                                name: text.clone(),
                                description: Some("Пользовательский пресет OpenWire CLI".to_string()),
                                author: Some("OpenWire CLI".to_string()),
                                version: 1,
                            };
                            let preset = VoicePreset::from_dto(meta, &snap, &text);
                            if let Ok(toml_str) = toml::to_string_pretty(&preset) {
                                match self.state.presets.save_preset(&text, &toml_str) {
                                    Ok(_) => {
                                        *self.state.active_preset.write().expect("preset lock") = Some(text.clone());
                                        self.set_notification(format!("Пресет \"{}\" успешно сохранён!", text));
                                    }
                                    Err(err) => {
                                        self.set_notification(format!("Ошибка сохранения: {err}"));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            KeyCode::Backspace => {
                prompt.buffer.pop();
            }
            KeyCode::Char(c) => {
                prompt.buffer.push(c);
            }
            _ => {}
        }
    }

    fn handle_soundboard_key(&mut self, key: KeyEvent) {
        let bank_idx = self.ui.soundboard.selected_bank;
        let slot_idx = self.ui.soundboard.selected_slot;

        match key.code {
            KeyCode::Esc => {
                self.state.sound.stop_all();
                self.set_notification("Остановлены ВСЕ звуки");
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                if let Err(e) = self.state.sound.play(bank_idx, slot_idx) {
                    self.set_notification(format!("Ошибка воспроизведения: {e}"));
                } else {
                    self.set_notification(format!("Пад #{:02}: Воспроизведение", slot_idx + 1));
                }
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                self.state.sound.stop_pad(bank_idx, slot_idx);
                self.set_notification(format!("Пад #{:02}: Остановлен", slot_idx + 1));
            }
            KeyCode::Char('b') | KeyCode::Char('B') => {
                let banks_count = self.state.sound.snapshot().banks.len();
                if banks_count > 0 {
                    self.ui.soundboard.selected_bank = (bank_idx + 1) % banks_count;
                }
            }
            KeyCode::Char('i') | KeyCode::Char('I') => {
                self.ui.input_prompt = Some(InputPrompt {
                    title: format!("Импорт аудио в Пад #{}", slot_idx + 1),
                    prompt: "Укажите абсолютный или относительный путь к файлу (.mp3, .wav, .flac, .ogg):".to_string(),
                    buffer: String::new(),
                    action: PromptAction::ImportPadFile { bank: bank_idx, slot: slot_idx },
                });
            }
            KeyCode::Char('m') | KeyCode::Char('M') => {
                let board = self.state.sound.snapshot();
                if let Some(bank) = board.banks.get(bank_idx) {
                    if let Some(mut pad) = bank.pads.get(slot_idx).cloned() {
                        pad.play_mode = if pad.play_mode == "loop" { "oneshot".into() } else { "loop".into() };
                        let mode = pad.play_mode.clone();
                        let _ = self.state.sound.update_pad(bank_idx, pad);
                        self.set_notification(format!("Пад #{:02}: режим переключен на {}", slot_idx + 1, mode));
                    }
                }
            }
            KeyCode::Delete | KeyCode::Char('d') => {
                let _ = self.state.sound.clear_pad(bank_idx, slot_idx);
                self.set_notification(format!("Пад #{:02}: очищен", slot_idx + 1));
            }
            KeyCode::Up => {
                if slot_idx >= 3 {
                    self.ui.soundboard.selected_slot -= 3;
                }
            }
            KeyCode::Down => {
                if slot_idx + 3 < 15 {
                    self.ui.soundboard.selected_slot += 3;
                }
            }
            KeyCode::Left => {
                if slot_idx > 0 {
                    self.ui.soundboard.selected_slot -= 1;
                }
            }
            KeyCode::Right => {
                if slot_idx + 1 < 15 {
                    self.ui.soundboard.selected_slot += 1;
                }
            }
            _ => {}
        }
    }

    fn handle_dsp_key(&mut self, key: KeyEvent) {
        let selected = self.ui.dsp.selected_param;
        let mut dto = self.state.dsp_dto();
        let mut monitor_enabled = self.state.settings.read().map(|s| s.monitor_enabled).unwrap_or(true);

        match key.code {
            KeyCode::Up => {
                if selected > 0 {
                    self.ui.dsp.selected_param -= 1;
                }
            }
            KeyCode::Down => {
                if selected + 1 < DSP_PARAM_COUNT {
                    self.ui.dsp.selected_param += 1;
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.state.disable_effects();
                *self.state.active_preset.write().expect("lock") = Some("Обычный голос".to_string());
                self.set_notification("Эффекты сброшены к чистому голосу");
            }
            KeyCode::Char('m') | KeyCode::Char('M') => {
                monitor_enabled = !monitor_enabled;
                self.state.dsp.monitor_enabled.store(monitor_enabled, Ordering::Relaxed);
                if let Ok(mut settings) = self.state.settings.write() {
                    settings.monitor_enabled = monitor_enabled;
                    let _ = settings.save();
                }
                self.set_notification(if monitor_enabled { "Слышать себя: ВКЛ" } else { "Слышать себя: ВЫКЛ" });
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                // Toggle boolean param
                match selected {
                    0 => dto.pitch_enabled = !dto.pitch_enabled,
                    7 => dto.radio_enabled = !dto.radio_enabled,
                    10 => dto.noise_gate_enabled = !dto.noise_gate_enabled,
                    12 => dto.ducking_enabled = !dto.ducking_enabled,
                    14 => {
                        monitor_enabled = !monitor_enabled;
                        self.state.dsp.monitor_enabled.store(monitor_enabled, Ordering::Relaxed);
                        if let Ok(mut settings) = self.state.settings.write() {
                            settings.monitor_enabled = monitor_enabled;
                            let _ = settings.save();
                        }
                    }
                    _ => {}
                }
                dto.apply(&self.state.dsp);
            }
            KeyCode::Left | KeyCode::Right | KeyCode::Char('[') | KeyCode::Char(']') => {
                let sign = if key.code == KeyCode::Left || key.code == KeyCode::Char('[') { -1.0 } else { 1.0 };
                let mult = if key.code == KeyCode::Char('[') || key.code == KeyCode::Char(']') { 3.0 } else { 1.0 };

                match selected {
                    1 => dto.pitch_semitones = (dto.pitch_semitones + sign * 0.5 * mult).clamp(-12.0, 12.0),
                    2 => dto.formant_shift = (dto.formant_shift + sign * 0.05 * mult).clamp(0.5, 2.0),
                    3 => dto.eq_low_db = (dto.eq_low_db + sign * 1.0 * mult).clamp(-24.0, 24.0),
                    4 => dto.eq_mid_gain_db = (dto.eq_mid_gain_db + sign * 1.0 * mult).clamp(-24.0, 24.0),
                    5 => dto.eq_mid_freq = (dto.eq_mid_freq + sign * 100.0 * mult).clamp(200.0, 8000.0),
                    6 => dto.eq_high_db = (dto.eq_high_db + sign * 1.0 * mult).clamp(-24.0, 24.0),
                    8 => dto.radio_drive = (dto.radio_drive + sign * 0.05 * mult).clamp(0.0, 1.0),
                    9 => dto.radio_noise_db = (dto.radio_noise_db + sign * 2.0 * mult).clamp(-90.0, -20.0),
                    11 => dto.noise_gate_db = (dto.noise_gate_db + sign * 2.0 * mult).clamp(-90.0, 0.0),
                    13 => dto.duck_atten_db = (dto.duck_atten_db + sign * 1.0 * mult).clamp(-40.0, 0.0),
                    15 => dto.stream_volume = (dto.stream_volume + sign * 0.05 * mult).clamp(0.0, 2.0),
                    _ => {}
                }
                dto.apply(&self.state.dsp);
            }
            _ => {}
        }
    }

    fn handle_presets_key(&mut self, key: KeyEvent) {
        let mut names = self.state.presets.list_presets().unwrap_or_default();
        if !names.contains(&"Обычный голос".to_string()) {
            names.insert(0, "Обычный голос".to_string());
        }

        let selected = self.ui.presets.selected_index.min(names.len().saturating_sub(1));

        match key.code {
            KeyCode::Up => {
                if selected > 0 {
                    self.ui.presets.selected_index = selected - 1;
                }
            }
            KeyCode::Down => {
                if selected + 1 < names.len() {
                    self.ui.presets.selected_index = selected + 1;
                }
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                if let Some(name) = names.get(selected) {
                    if let Err(e) = self.state.apply_preset(name) {
                        self.set_notification(format!("Ошибка применения пресета: {e}"));
                    } else {
                        self.set_notification(format!("Активирован пресет: \"{}\"", name));
                    }
                }
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                self.state.apply_next_preset();
                let cur = self.state.active_preset.read().ok().and_then(|c| c.clone()).unwrap_or_default();
                self.set_notification(format!("Переключен пресет: \"{}\"", cur));
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                self.ui.input_prompt = Some(InputPrompt {
                    title: "Сохранить пресет".to_string(),
                    prompt: "Введите название нового голосового пресета:".to_string(),
                    buffer: String::new(),
                    action: PromptAction::SavePreset,
                });
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                if let Some(name) = names.get(selected) {
                    if name == "Обычный голос" {
                        self.set_notification("Нельзя удалить встроенный пресет \"Обычный голос\"");
                    } else if let Err(e) = self.state.presets.delete_preset(name) {
                        self.set_notification(format!("Ошибка удаления: {e}"));
                    } else {
                        self.set_notification(format!("Удалён пресет \"{}\"", name));
                        if selected > 0 {
                            self.ui.presets.selected_index -= 1;
                        }
                    }
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                let _ = self.state.apply_preset("Обычный голос");
                self.set_notification("Активирован чистый голос");
            }
            _ => {}
        }
    }

    fn handle_monitor_key(&mut self, _key: KeyEvent) {}

    fn handle_about_key(&mut self, _key: KeyEvent) {}
}
