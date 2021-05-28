use futures::StreamExt;
use std::{
    sync::{
        mpsc,
        mpsc::{Receiver, Sender},
    },
    thread,
    thread::JoinHandle,
};

enum Command {
    Quit,
    Load(i32, i32, i32),
}

pub struct Chunk;

enum Result {
    Chunk(Chunk),
}

pub struct AssetLoader {
    send_load: Sender<Command>,
    receive_result: Receiver<Result>,
    join_handle: JoinHandle<()>,
}

impl AssetLoader {
    pub fn new() -> Self {
        let (send_load, receive_load): (Sender<Command>, Receiver<Command>) = mpsc::channel();
        let (send_result, receive_result): (Sender<Result>, Receiver<Result>) = mpsc::channel();
        let join_handle = thread::spawn(move || loop {
            match receive_load.recv().unwrap() {
                Command::Quit => {
                    return ();
                }
                Command::Load(x, y, z) => {
                    send_result.send(Result::Chunk(Chunk));
                }
            }
        });
        Self {
            send_load,
            receive_result,
            join_handle,
        }
    }

    pub fn request(&mut self, x: i32, y: i32, z: i32) {
        self.send_load.send(Command::Load(x, y, z));
    }

    pub fn try_retrieve(&mut self) -> Option<Chunk> {
        if let Ok(result) = self.receive_result.try_recv() {
            match result {
                Result::Chunk(chunk) => Some(chunk),
            }
        } else {
            None
        }
    }

    pub fn quit_join(self) {
        self.send_load.send(Command::Quit);
        self.join_handle.join().unwrap();
    }
}
