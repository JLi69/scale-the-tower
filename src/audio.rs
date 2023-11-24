use std::{io::BufReader, fs::File};
use rodio::{OutputStream, source::Buffered, Decoder, OutputStreamHandle, Source};

type Sfx = Buffered<Decoder<BufReader<File>>>;

pub mod sfx_ids {
    pub const JUMP: usize = 0;
    pub const COIN: usize = 1;
    pub const POWERUP: usize = 2;
    pub const PLAYER_HIT: usize = 3;
    pub const ENEMY_HIT: usize = 4;
    pub const EXPLODE: usize = 5;
    pub const SELECT: usize = 6;
}

pub struct SfxPlayer {
    sources: Vec<Sfx>,
    stream: Option<(OutputStream, OutputStreamHandle)>,
}

fn sfx_from_file(path: &str) -> Result<Sfx, String> {
    let file = BufReader::new(File::open(path).map_err(|e| e.to_string())?);
    let decoder = Decoder::new(file).map_err(|e| e.to_string())?;
    Ok(decoder.buffered())
}

fn load_sounds() -> Vec<Sfx> {
    let mut sounds = vec![];

    let paths = [ 
        "assets/audio/jump.wav",
        "assets/audio/coin.wav",
        "assets/audio/powerup.wav",
        "assets/audio/player_hit.wav",
        "assets/audio/enemy_hit.wav",
        "assets/audio/explode.wav",
        "assets/audio/select.wav",
    ];

    for path in paths {
        match sfx_from_file(path) {
            Ok(sfx) => sounds.push(sfx),
            Err(msg) => eprintln!("{msg}"),
        }
    }

    sounds
}

impl SfxPlayer { 
    pub fn init() -> Self {
        match OutputStream::try_default() {
            Ok((stream, stream_handle)) => {
                Self { 
                    sources: load_sounds(),
                    stream: Some((stream, stream_handle)),
                }
            }
            Err(msg) => {
                eprintln!("{msg}");
                Self {
                    sources: vec![],
                    stream: None,
                }
            }
        }
    }

    pub fn play(&self, index: usize) {
        if let Some((_, stream_handle)) = &self.stream {
            if index < self.sources.len() {
                let res = stream_handle.play_raw(self.sources[index].clone().convert_samples());
                if let Err(msg) = res {
                    eprintln!("{msg}");
                } 
            }
        }
    }
}
