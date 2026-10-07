# OpenWire CLI

```text
                                            ████████████                                            
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
                                            ████████████                                            
```

**OpenWire CLI** is a lightweight, pure Rust terminal soundpad and real-time DSP voice effects processor for Linux powered by PipeWire and Ratatui.

It is an independent CLI / TUI fork of the **OpenWire** soundboard, free from any Tauri, WebKit, Node.js, or browser engine dependencies.

---

## Features

- **Pure Native Rust**: Zero web bloat, ultra-low resource usage, sub-millisecond startup.
- **Full Interactive TUI**: Rich keyboard-driven terminal user interface styled with OpenWire's orange theme.
- **Soundboard Grid**: 15 pads per bank with independent headphones / stream gain, trim, oneshot/loop modes.
- **Real-Time Voice Effects**:
  - Pitch & Formant Shifting
  - 3-Band Parametric Equalizer
  - Radio FX (band-pass filter, saturation drive, noise floor)
  - Noise Gate
  - Automatic Mic Ducking during pad playback
  - Local monitoring ("Hear Myself")
- **TOML Presets**: Seamlessly load and save voice presets compatible with OpenWire GUI.
- **PipeWire Integration**: Connects directly to PipeWire audio graph creating virtual mic (`openwire.virtual-mic`) and monitor nodes.
- **Panic Stop**: Instantly silence all active pads with `Esc`.

---

## Building & Installation

```bash
git clone https://github.com/openwire/openwire-cli.git
cd openwire-cli
cargo build --release
./target/release/openwire-cli
```

### Packaging for Arch Linux:
```bash
cd packaging
./build.sh
sudo pacman -U openwire-cli-*.pkg.tar.zst
```

---

## Keyboard Controls

| Key | Action |
|---|---|
| `1 .. 5` / `F1 .. F5` | Switch tab (1: Pads, 2: DSP, 3: Presets, 4: Monitor, 5: About) |
| `Tab` / `Shift+Tab` | Next / previous tab |
| `Arrow Keys` | Navigate pad grid or DSP sliders |
| `Space` / `Enter` | Play/stop pad or toggle option |
| `S` | Stop selected pad |
| `Esc` | **Panic stop**: kill all playing sounds |
| `I` | Import audio file into selected pad |
| `B` | Next bank |
| `M` | Toggle loop mode or voice monitor |
| `R` | Reset effects to clean voice |
| `N` | Cycle next voice preset |
| `Del` | Clear pad |
| `Q` / `Ctrl+C` | Quit |
