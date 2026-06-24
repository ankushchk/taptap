use rdev::{listen, EventType};

use crate::audio::SoundPlayer;
use crate::keycode;
use crate::soundpack::SoundPack;

pub fn start(player: SoundPlayer, pack: SoundPack) {
    listen(move |event| {
        if let EventType::KeyPress(key) = event.event_type {
            if let Some(code) = keycode::key_to_code(key) {
                if let Some([start, len]) = pack.defines.get(&code) {
                    player.play_clip(*start, *len);
                }
            }
        }
    })
    .unwrap();
}