#![allow(dead_code)]

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};

/// The exact ASCII art requested by the user, representing OpenWire.
pub const OPENWIRE_ASCII_ART: &str = r#"                                            ████████████                                            
                                         ██████████████████                                         
                                       ██████████████████████                                       
                                     ██████████████████████████                                     
                                   ██████████████████████████████                                   
                                  ████████████████████████████████                                  
                                 ███████████    ████    ███████████                                 
                                █████████       ████       █████████                                
                               ████████         ████         ████████                               
                              ████████          ████          ████████                              
                             ████████          ██████          ████████                             
                             ███████           ██████           ███████                             
                            ███████            ██████            ███████                            
                           ███████             ██████             ███████                           
                           ██████             ████████             ██████                           
                          ███████             ████████             ███████                          
                          ██████              ████████              ██████                          
                          ██████              ████████              ██████                          
                         ██████               ████████               ██████                         
                         ██████              ██████████              ██████                         
                         ██████              ██████████              ██████                         
                         █████               ██████████               █████                         
                         █████               ██████████               █████                         
                         ██████████████      ██████████      ██████████████                         
                        ████████████████    ████████████    ████████████████                        
                        ████████████████    █████  █████    ████████████████                        
                         ███████████████    █████  █████    ███████████████                         
                         ████████████████   █████  █████   ████████████████                         
                         █████     ██████  ██████  ██████  ██████     █████                         
                         █████      █████  ██████  ██████  █████      █████                         
                         ██████     ██████ █████    █████ ██████     ██████                         
                         ██████      █████ █████    █████ █████      ██████                         
                         ██████      █████ █████    █████ █████      ██████                         
                          ██████     ███████████    ███████████     ██████                          
                          ██████      █████████      █████████      ██████                          
                          ███████     █████████      █████████     ███████                          
                           ██████     █████████      █████████     ██████                           
                            ██████     ████████      ████████     ██████                            
                            ███████    ████████      ████████    ███████                            
                             ███████   ███████        ███████   ███████                             
                             ████████   ██████        ██████   ████████                             
                              ████████  ██████        ██████  ████████                              
                               ████████ ██████        ██████ ████████                               
                                █████████████          █████████████                                
                                 ████████████          ████████████                                 
                                  ███████████████ ████████████████                                  
                                    ████████████████████████████                                    
                                     ██████████████████████████                                     
                                       ██████████████████████                                       
                                         ██████████████████                                         
                                            ████████████                                            "#;

/// Compact ASCII logo for smaller terminal viewports or headers
pub const OPENWIRE_COMPACT_LOGO: &str = r#"   ___                    _      ___            ___  _     ___ 
  / _ \ _ __   ___  _ __ | |    / / (_)_ __ ___/ __\| |   |_ _|
 | | | | '_ \ / _ \| '_ \| | /\/ /| | '__/ _ \ /  | |    | | 
 | |_| | |_) |  __/| | | | \V  V / | | | |  __/ /___| |___ | | 
  \___/| .__/ \___||_| |_|  \_/\_/  |_|_|  \___\____/_____|___|
       |_|                                                     "#;

/// Primary orange accent color for OpenWire brand
pub const ORANGE_COLOR: Color = Color::Rgb(255, 140, 0);       // Rich vibrant orange (#FF8C00)
pub const ORANGE_ACCENT: Color = Color::Rgb(255, 170, 50);     // Lighter warm orange
pub const ORANGE_DARK: Color = Color::Rgb(180, 80, 0);         // Deep rust/orange

/// Returns the ASCII art styled in orange as a Ratatui Text
pub fn ascii_art_widget() -> Text<'static> {
    let style = Style::default().fg(ORANGE_COLOR).add_modifier(Modifier::BOLD);
    let lines: Vec<Line<'static>> = OPENWIRE_ASCII_ART
        .lines()
        .map(|line| Line::from(Span::styled(line.to_string(), style)))
        .collect();
    Text::from(lines)
}

/// Returns the compact logo styled in orange
pub fn compact_logo_widget() -> Text<'static> {
    let style = Style::default().fg(ORANGE_COLOR).add_modifier(Modifier::BOLD);
    let lines: Vec<Line<'static>> = OPENWIRE_COMPACT_LOGO
        .lines()
        .map(|line| Line::from(Span::styled(line.to_string(), style)))
        .collect();
    Text::from(lines)
}

/// Prints the ASCII art in ANSI orange directly to standard output
pub fn print_orange_banner() {
    let orange_ansi = "\x1b[38;2;255;140;0m\x1b[1m";
    let reset_ansi = "\x1b[0m";
    println!("{orange_ansi}{OPENWIRE_ASCII_ART}{reset_ansi}");
}
