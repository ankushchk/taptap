use rodio::OutputStream;
use std::io::{self, Write};

mod audio;
mod keycode;
mod listener;
mod soundpack;

fn print_usage() {
    println!("taptap - Mechanical Keyboard Sounds CLI");
    println!();
    println!("Usage:");
    println!("  taptap                 Interactive selection menu");
    println!("  taptap play <index>    Play a soundpack by index directly");
    println!("  taptap list            List all installed soundpacks");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Check for flags/subcommands
    if args.len() > 1 {
        match args[1].as_str() {
            "--help" | "-h" | "help" => {
                print_usage();
                return;
            }
            "list" => {
                let packs = soundpack::scan_soundpacks();
                println!("Installed soundpacks ({}):", packs.len());
                for (i, p) in packs.iter().enumerate() {
                    println!("  [{}] {}", i, p.name);
                }
                return;
            }
            "play" => {
                if args.len() > 2 {
                    if let Ok(idx) = args[2].parse::<usize>() {
                        let packs = soundpack::scan_soundpacks();
                        if idx < packs.len() {
                            let selected = &packs[idx];
                            // Allow volume as optional third argument (default to 80)
                            let volume = if args.len() > 3 {
                                args[3].parse::<u32>().unwrap_or(80).min(100)
                            } else {
                                80
                            };
                            run_pack(&format!("{}/config.json", selected.path), &selected.name, volume);
                            return;
                        } else {
                            eprintln!("Error: Soundpack index {} does not exist. Run 'taptap list' to see available packs.", idx);
                            return;
                        }
                    }
                }
                eprintln!("Error: Please provide a soundpack index to play (e.g., 'taptap play 0').");
                return;
            }
            other => {
                if let Ok(idx) = other.parse::<usize>() {
                    let packs = soundpack::scan_soundpacks();
                    if idx < packs.len() {
                        let selected = &packs[idx];
                        run_pack(&format!("{}/config.json", selected.path), &selected.name, 80);
                        return;
                    }
                }
                eprintln!("Unknown command/flag: {}", other);
                print_usage();
                return;
            }
        }
    }

    // Default Interactive Run
    let packs = soundpack::scan_soundpacks();
    if packs.is_empty() {
        println!("No soundpacks found. Please create a 'soundpacks/' folder in the current directory and place your switch sound folders inside it.");
        return;
    }

    println!("Discovered {} soundpacks:", packs.len());
    for (i, p) in packs.iter().enumerate() {
        println!("  [{}] {}", i, p.name);
    }

    print!("Choose soundpack index [0-{}]: ", packs.len() - 1);
    let _ = io::stdout().flush();
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);

    let selected_idx = input.trim().parse::<usize>().unwrap_or(0);
    let chosen_idx = if selected_idx >= packs.len() {
        println!("Invalid index. Defaulting to index 0.");
        0
    } else {
        selected_idx
    };

    let selected_pack = &packs[chosen_idx];
    run_pack(&format!("{}/config.json", selected_pack.path), &selected_pack.name, 80);
}

fn run_pack(config_path: &str, pack_name: &str, volume: u32) {
    println!("Loading soundpack: {}", pack_name);
    let pack = soundpack::load_soundpack(config_path);

    // Keep the output stream alive so audio plays back properly
    let (_stream, stream_handle) = OutputStream::try_default()
        .expect("Failed to open default audio output device");

    let player = audio::SoundPlayer::new(&pack.sound, stream_handle);
    player.set_volume(volume);

    println!("Set player volume to: {}%", player.volume());
    println!("Listening for key events... Press keys to play mechanical keyboard sounds!");

    listener::start(player, pack);
}