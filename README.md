# taptap 🦀

`taptap` is an ultra-lightweight, high-performance mechanical keyboard sound simulator CLI written in Rust. It intercepts system-wide keyboard events and plays real mechanical switch sound clips in real-time.

It compiles to an extremely small binary (**~800 KB**) consumes minimal resources and features sub-millisecond playback latency.

---

## Features

*   ⚡ **Sub-Millisecond Latency**: Pre-decodes audio files into raw PCM buffers (`Vec<f32>`) at startup, completely removing runtime disk I/O and decoding overhead.
*   🎵 **Concurrently Mixed Overlaps**: Audio slices are routed directly to the system hardware mixer. Typing fast produces natural, overlapping switch sounds (polyphony).
*   🪶 **Ultra Lightweight**: Zero heavy dependencies, compiling down to under 800 KB and utilizing less than 10MB of RAM.
*   📂 **Soundpack Discovery**: Automatically scans folders in your local `soundpacks/` directory or your global `~/.taptap/soundpacks/` directory, and parses switch profiles (fully compatible with Mechvibes configurations).

---

## Installation

### Method 1: Curl Global Installer
```bash
curl -fsSL https://raw.githubusercontent.com/ankushchk/taptap/master/install.sh | sh
```

### Method 2: Local Cargo Installation (Recommended for Development)
To build and install the binary globally on your system directly from source:
```bash
cargo install --path .
```
This compiles the optimized release binary and installs it to your Cargo bin folder (`~/.cargo/bin/`), making the `taptap` command globally available.



---

## Quick Start & Usage

1. Open your terminal inside the project directory (which contains the `soundpacks/` folder) and run:
   ```bash
   taptap
   ```
2. You will be prompted to pick a keyboard switch profile:
   ```text
   Discovered 8 soundpacks:
     [0] CherryMX Red - PBT keycaps
     [1] CherryMX Black - ABS keycaps
     [2] CherryMX Blue - PBT keycaps
     ...
   Choose soundpack index [0-7]: 1
   ```
3. Type the index of your choice and press **Enter**.
4. Start typing anywhere on your computer to hear the sounds! Press `Ctrl + C` in the terminal to stop.

### CLI Commands & Flags
*   **Show help menu**:
    ```bash
    taptap help
    ```
*   **List all available local soundpacks**:
    ```bash
    taptap list
    ```
*   **Play a soundpack directly (skips the menu)**:
    ```bash
    taptap play <index> [optional_volume_0_100]
    # Example: Play pack 2 at 90% volume
    taptap play 2 90
    ```

---

## Adding Custom Soundpacks

You can add custom soundpacks by dropping their folders into either the local `soundpacks/` directory (relative to your current working directory) or the global `~/.taptap/soundpacks/` directory. Each soundpack folder must contain:
1. `config.json`: The mapping of scan codes to sound clips (Mechvibes standard).
2. `sound.ogg` (or `.mp3` / `.wav`): The combined audio sprite file containing all sound clips.

```text
soundpacks/
  ├── cherrymx-black-abs/
  │     ├── config.json
  │     └── sound.ogg
  └── my-custom-switch/
        ├── config.json
        └── sound.ogg
```
