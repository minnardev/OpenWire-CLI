# OpenWire CLI

```text
                    ████████████                    
               ██████████████████████               
           ██████████████████████████████           
         ███████████    ████    ███████████         
       ████████         ████         ████████       
     ████████          ██████          ████████     
    ███████            ██████            ███████    
   ██████             ████████             ██████   
  ██████              ████████              ██████  
 ██████               ████████               ██████ 
 ██████              ██████████              ██████ 
 █████               ██████████               █████ 
████████████████    ████████████    ████████████████
 ███████████████    █████  █████    ███████████████ 
 █████     ██████  ██████  ██████  ██████     █████ 
 ██████     ██████ █████    █████ ██████     ██████ 
 ██████      █████ █████    █████ █████      ██████ 
  ██████      █████████      █████████      ██████  
   ██████     █████████      █████████     ██████   
    ███████    ████████      ████████    ███████    
     ████████   ██████        ██████   ████████     
       ████████ ██████        ██████ ████████       
         ████████████          ████████████         
            ████████████████████████████            
               ██████████████████████               
                    ████████████                    
```

**OpenWire CLI** is a terminal soundpad and voice DSP processor for Linux (PipeWire).

CLI/TUI port of **OpenWire**.

---

## Features

### Soundboard
- 3x5 pad grid (15 pads per bank);
- Bank support;
- Playback formats: MP3, WAV, FLAC, OGG, AAC;
- Modes: oneshot and loop;
- Independent headphones and mic volume;
- Panic stop key: Esc.

### Voice DSP
- Pitch Shift and Formant Shift;
- 3-band parametric equalizer;
- Radio FX (band-pass filter, saturation drive, noise floor);
- Noise Gate;
- Mic Ducking during pad playback;
- Local monitoring ("Hear Myself").

### Presets
- Cycle presets with N;
- Save presets as TOML with S;
- Reset to clean voice with R;
- OpenWire presets compatibility.

### Monitoring
- Live VU meters;
- PipeWire quantum and latency display;
- Virtual microphone status.

---

## Building

```bash
cargo build --release
```

Run:
```bash
./target/release/openwire-cli
```

Build without PipeWire (offline mode):
```bash
cargo build --release --no-default-features
```

---

## Keybindings

| Key | Action |
|---|---|
| **1 .. 5** / **Tab** | Switch tabs (Pads, Effects, Presets, Monitors, About) |
| **Arrow keys** | Navigate |
| **Space** / **Enter** | Play / toggle |
| **S** | Stop selected pad |
| **Esc** | Stop all sounds |
| **I** | Import audio file |
| **B** | Next bank |
| **M** | Toggle loop mode or voice monitor |
| **R** | Reset effects to clean voice |
| **N** | Next preset |
| **Del** | Clear pad |
| **Q** | Quit |

---

## Authorship and License

- **Original Project:** [OpenWire](https://github.com/AdrescorGiti/OpenWire)
- **Original Author:** **GitiAdrescor**
- **License:** [GNU General Public License v3.0 (GPLv3)](LICENSE).

In accordance with the GNU General Public License v3.0, this CLI port retains the original GPLv3 license and credits the creator of the original project.
