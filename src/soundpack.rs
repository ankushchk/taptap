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

    // Scan local current working directory (for local testing/dev)
    let local_dir = std::path::PathBuf::from("soundpacks");
    if local_dir.exists() && local_dir.is_dir() {
        scan_dir(&local_dir, &mut packs);
    }

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