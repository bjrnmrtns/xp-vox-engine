use crate::{registry::Registry, vox, vox::Vox, world::Chunker};
use futures::StreamExt;
use std::{
    sync::{
        mpsc,
        mpsc::{Receiver, Sender},
    },
    thread,
    thread::JoinHandle,
};

pub enum Command {
    Quit,
    Load(i32, i32, i32),
    LoadVox(&'static str),
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
        let join_handle = thread::spawn(move || {
            let mut vox_models = Registry::new();
            let mut world = Chunker::new();
            loop {
                match receive_load.recv().unwrap() {
                    Command::Quit => {
                        return ();
                    }
                    Command::Load(x, y, z) => {
                        send_result.send(Result::Chunk(Chunk));
                    }
                    Command::LoadVox(path) => {
                        let tree_house_hanlde = vox::load_vox(
                            &dot_vox::load_bytes(std::fs::read(path).unwrap().as_slice()).unwrap(),
                            &mut vox_models,
                        );
                        world.add(tree_house_hanlde.clone(), [0, 0, 0], &vox_models);
                        world.add(tree_house_hanlde.clone(), [0, 0, 128], &vox_models);
                        world.add(tree_house_hanlde.clone(), [128, 0, 128], &vox_models);
                        world.add(tree_house_hanlde.clone(), [128, 0, 0], &vox_models);

                        world.add(tree_house_hanlde.clone(), [128, 0, 0], &vox_models);
                        world.add(tree_house_hanlde.clone(), [128, 0, 128], &vox_models);
                        world.add(tree_house_hanlde.clone(), [256, 0, 128], &vox_models);
                        world.add(tree_house_hanlde.clone(), [256, 0, 0], &vox_models);

                        world.add(tree_house_hanlde.clone(), [0, 0, 128], &vox_models);
                        world.add(tree_house_hanlde.clone(), [0, 0, 256], &vox_models);
                        world.add(tree_house_hanlde.clone(), [128, 0, 256], &vox_models);
                        world.add(tree_house_hanlde.clone(), [128, 0, 128], &vox_models);

                        world.add(tree_house_hanlde.clone(), [128, 0, 128], &vox_models);
                        world.add(tree_house_hanlde.clone(), [128, 0, 256], &vox_models);
                        world.add(tree_house_hanlde.clone(), [256, 0, 256], &vox_models);
                        world.add(tree_house_hanlde.clone(), [256, 0, 128], &vox_models);
                    }
                }
            }
        });
        Self {
            send_load,
            receive_result,
            join_handle,
        }
    }

    pub fn request(&mut self, command: Command) {
        self.send_load.send(command);
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
