use anyhow::{Context, Result};
use rodio::Sink;
use std::io::{BufReader, Cursor};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;
use std::fs::File;
use log::error;



#[derive(Clone, Copy, Debug)]
pub enum MusicCommand {
    Pause,
    Play,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MusicState {
    Playing,
    Paused,
}

pub fn spawn(song: String) -> Sender<MusicCommand> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        if let Err(e) = play(&rx, song) {
            error!("Music thread crashed: {:#}", e)
        }
    });
    tx
}

fn play(rx: &Receiver<MusicCommand>, song: String) -> Result<()> {
    let (_stream, stream_handle) =
        rodio::OutputStream::try_default().context("Failed to get output stream")?;
    let sink = Sink::try_new(&stream_handle).context("Failed to create Sink")?;
    let mut state = MusicState::Playing;
    loop {
        if let Ok(cmd) = rx.try_recv() {
            match cmd {
                MusicCommand::Pause => {
                    state = MusicState::Paused;
                    sink.pause()
                }
                MusicCommand::Play => {
                    state = MusicState::Playing;
                    sink.play()
                }
            }
        }
        
        if state == MusicState::Playing && sink.empty() {
            let file = File::open(song.clone()).unwrap();
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
            sink.append(source);
        }
        thread::sleep(Duration::from_millis(100));
    }
}

fn main(){
   let a =  spawn("./src/song.mp3".to_string());
   thread::sleep(Duration::from_secs(5));
   a.send(MusicCommand::Pause).expect("1");
   thread::sleep(Duration::from_secs(5));
   a.send(MusicCommand::Play).expect("2");
   thread::sleep(Duration::from_secs(5));
}