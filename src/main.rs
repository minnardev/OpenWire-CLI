use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

mod app;
mod audio;
mod banner;
mod input;
mod presets;
mod soundboard;
mod state;
mod ui;

use app::App;
use banner::print_orange_banner;

fn print_help() {
    print_orange_banner();
    println!(
        r#"OpenWire CLI / TUI — native soundpad and real-time DSP voice effects for Linux

ИСПОЛЬЗОВАНИЕ:
    openwire-cli [ОПЦИИ]

ОПЦИИ:
    -h, --help       Показать эту справку
    -v, --version    Показать версию OpenWire CLI
    -b, --banner     Вывести фирменный оранжевый ASCII-арт логотип OpenWire

УПРАВЛЕНИЕ В TUI:
    [1..5 / F1..F5]  Переключение между вкладками:
                     1: 🔊 Пады (Звуковая доска)
                     2: 🎙️ Эффекты (DSP процессор)
                     3: 🎭 Пресеты (Пресеты голоса)
                     4: 📊 Мониторы (PipeWire и VU метры)
                     5: ℹ️ О программе (Логотип и справка)

    [Пробел/Enter]   Воспроизвести выбранный пад / применить настройку
    [Esc]            ПАНИКА — немедленно остановить все играющие звуки
    [Tab / BackTab]  Следующая / предыдущая вкладка
    [Q / Ctrl+C]     Выход из приложения
"#
    );
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "-b" | "--banner" => {
                print_orange_banner();
                return Ok(());
            }
            "-h" | "--help" => {
                print_help();
                return Ok(());
            }
            "-v" | "--version" => {
                println!("OpenWire CLI v0.1.0 (PipeWire Linux native)");
                return Ok(());
            }
            _ => {}
        }
    }

    // Set up custom panic hook to always restore terminal on unhandled panics
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        original_hook(panic_info);
    }));

    // Initialize terminal for full-screen TUI
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run application loop
    let mut app = match App::new() {
        Ok(app) => app,
        Err(err) => {
            // Restore terminal before reporting bootstrap error
            disable_raw_mode()?;
            execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
            terminal.show_cursor()?;
            eprintln!("\x1b[31mОшибка инициализации OpenWire CLI: {err}\x1b[0m");
            std::process::exit(1);
        }
    };

    let result = app.run(&mut terminal);

    // Restore terminal cleanly
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}
