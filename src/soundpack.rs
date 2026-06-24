use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct SoundPack {
    pub sound: String,
    pub defines: HashMap<String, [u32; 2]>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SoundPackInfo {
    pub id: String,
    pub name: String,
    pub path: String,
}

pub fn get_taptap_dir() -> std::path::PathBuf {
    if let Some(mut path) = dirs::data_dir() {
        path.push("taptap");
        path
    } else {
        std::path::PathBuf::from(".taptap")
    }
}

pub fn get_soundpacks_dir() -> std::path::PathBuf {
    let mut dir = get_taptap_dir();
    dir.push("soundpacks");
    dir
}

pub fn load_soundpack(path: &str) -> SoundPack {
    let json = std::fs::read_to_string(path)
        .expect("Couldn't read config");

    let mut pack: SoundPack = serde_json::from_str(&json)
        .expect("Couldn't parse config");

    // Resolve the sound file path relative to the config directory
    if let Some(parent) = std::path::Path::new(path).parent() {
        let resolved_sound = parent.join(&pack.sound);
        pack.sound = resolved_sound.to_string_lossy().into_owned();
    }

    pack
}

pub fn scan_soundpacks() -> Vec<SoundPackInfo> {
    let mut packs = Vec::new();

    // 1. Scan global user data directory
    let global_dir = get_soundpacks_dir();
    let _ = std::fs::create_dir_all(&global_dir);
    scan_dir(&global_dir, &mut packs);

    // 2. Scan local current working directory (for local testing/dev)
    let local_dir = std::path::PathBuf::from("soundpacks");
    if local_dir.exists() && local_dir.is_dir() {
        scan_dir(&local_dir, &mut packs);
    }

    // De-duplicate by ID
    packs.sort_by(|a, b| a.id.cmp(&b.id));
    packs.dedup_by(|a, b| a.id == b.id);

    packs
}

fn scan_dir(dir_path: &std::path::Path, packs: &mut Vec<SoundPackInfo>) {
    let dir = match std::fs::read_dir(dir_path) {
        Ok(dir) => dir,
        Err(_) => return,
    };

    for entry in dir.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            let config_path = path.join("config.json");
            if config_path.exists() {
                if let Ok(json_str) = std::fs::read_to_string(&config_path) {
                    #[derive(Deserialize)]
                    struct Meta {
                        id: String,
                        name: String,
                    }
                    if let Ok(meta) = serde_json::from_str::<Meta>(&json_str) {
                        packs.push(SoundPackInfo {
                            id: meta.id,
                            name: meta.name,
                            path: path.to_string_lossy().into_owned(),
                        });
                    }
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub default_soundpack_id: Option<String>,
    pub volume: u32,
}

impl AppConfig {
    pub fn load() -> Self {
        let path = get_taptap_dir().join("config.json");
        if path.exists() {
            if let Ok(content) = std::fs::read_to_string(path) {
                if let Ok(config) = serde_json::from_str(&content) {
                    return config;
                }
            }
        }
        // Default settings
        Self {
            default_soundpack_id: None,
            volume: 80,
        }
    }

    pub fn save(&self) {
        let path = get_taptap_dir().join("config.json");
        let _ = std::fs::create_dir_all(get_taptap_dir());
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(path, json);
        }
    }
}

pub fn download_default_soundpacks() -> Result<(), String> {
    let soundpacks_dir = get_soundpacks_dir();
    let _ = std::fs::create_dir_all(&soundpacks_dir);

    println!("Downloading CherryMX Black ABS soundpack...");
    let pack_dir = soundpacks_dir.join("cherrymx-black-abs");
    let _ = std::fs::create_dir_all(&pack_dir);

    // Download config.json
    let config_url = "https://raw.githubusercontent.com/ankushchk/kirat-assignment/master/cohort-3/web3/rustkeys/config.json";
    let output_config = pack_dir.join("config.json");
    
    let status = std::process::Command::new("curl")
        .arg("-L")
        .arg(config_url)
        .arg("-o")
        .arg(&output_config)
        .status();
        
    if status.is_err() || !status.unwrap().success() {
        return Err("Failed to download config.json using curl".to_string());
    }

    // Download sound.ogg
    let sound_url = "https://raw.githubusercontent.com/ankushchk/kirat-assignment/master/cohort-3/web3/rustkeys/sound.ogg";
    let output_sound = pack_dir.join("sound.ogg");
    
    println!("Downloading sound.ogg (this may take a moment)...");
    let status = std::process::Command::new("curl")
        .arg("-L")
        .arg(sound_url)
        .arg("-o")
        .arg(&output_sound)
        .status();

    if status.is_err() || !status.unwrap().success() {
        return Err("Failed to download sound.ogg using curl".to_string());
    }

    println!("Successfully downloaded CherryMX Black ABS soundpack!");
    Ok(())
}