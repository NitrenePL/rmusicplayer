use std::{fs::File, path::Path};

use rodio::{Decoder, OutputStream, Sink};

pub struct Playback {
    speed: f32,
    volume: f32,
    pub sink: Box<Sink>,
}

impl Playback {
    pub fn new(speed: f32, volume: f32) -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.set_speed(speed);
        sink.set_volume(volume);
        Self {
            speed,
            volume,
            sink: Box::new(sink),
        }
    }

    pub fn list_append(&mut self, path: &Path) {
        let source = Decoder::new(File::open(path).unwrap()).unwrap();
        self.sink.append(source);
    }

    pub fn play(&self) {
        self.sink.sleep_until_end();
    }
}
