use crate::{chunker::Chunker, mesh::Mesh, registry::Registry, transform::Transform, vox};
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

enum Result {
    Chunk(Mesh, Transform, (i32, i32, i32)),
}

pub struct AssetLoader {
    pub send_load: Sender<Command>,
    receive_result: Receiver<Result>,
    pub join_handle: Option<JoinHandle<()>>,
}

impl AssetLoader {
    pub fn new(chunk_size: usize) -> Self {
        let (send_load, receive_load): (Sender<Command>, Receiver<Command>) = mpsc::channel();
        let (send_result, receive_result): (Sender<Result>, Receiver<Result>) = mpsc::channel();
        let join_handle = thread::spawn(move || {
            let mut vox_models = Registry::new();
            let mut chunker = Chunker::new(chunk_size);
            loop {
                match receive_load.recv().unwrap() {
                    Command::Quit => {
                        return ();
                    }
                    Command::Load(x, y, z) => {
                        let (mesh, transform) = chunker.generate_chunk(&vox_models, (x, y, z));
                        if let Some(mesh) = mesh {
                            send_result.send(Result::Chunk(mesh, transform, (x, y, z))).unwrap();
                        }
                    }
                    Command::LoadVox(path) => {
                        let tree_house_hanlde = vox::load_vox(
                            &dot_vox::load_bytes(std::fs::read(path).unwrap().as_slice()).unwrap(),
                            &mut vox_models,
                        );
                        chunker.add(tree_house_hanlde.clone(), [0, 0, 0], &vox_models);
                        chunker.add(tree_house_hanlde.clone(), [0, 0, 128], &vox_models);
                        chunker.add(tree_house_hanlde.clone(), [128, 0, 128], &vox_models);
                        chunker.add(tree_house_hanlde.clone(), [128, 0, 0], &vox_models);

                        chunker.add(tree_house_hanlde.clone(), [128, 0, 0], &vox_models);
                        chunker.add(tree_house_hanlde.clone(), [128, 0, 128], &vox_models);
                        chunker.add(tree_house_hanlde.clone(), [256, 0, 128], &vox_models);
                        chunker.add(tree_house_hanlde.clone(), [256, 0, 0], &vox_models);

                        chunker.add(tree_house_hanlde.clone(), [0, 0, 128], &vox_models);
                        chunker.add(tree_house_hanlde.clone(), [0, 0, 256], &vox_models);
                        chunker.add(tree_house_hanlde.clone(), [128, 0, 256], &vox_models);
                        chunker.add(tree_house_hanlde.clone(), [128, 0, 128], &vox_models);

                        chunker.add(tree_house_hanlde.clone(), [128, 0, 128], &vox_models);
                        chunker.add(tree_house_hanlde.clone(), [128, 0, 256], &vox_models);
                        chunker.add(tree_house_hanlde.clone(), [256, 0, 256], &vox_models);
                        chunker.add(tree_house_hanlde.clone(), [256, 0, 128], &vox_models);
                    }
                }
            }
        });
        Self {
            send_load,
            receive_result,
            join_handle: Some(join_handle),
        }
    }

    pub fn request(&mut self, command: Command) {
        self.send_load.send(command).unwrap();
    }

    pub fn try_retrieve(&mut self) -> Option<(Mesh, Transform, (i32, i32, i32))> {
        if let Ok(result) = self.receive_result.try_recv() {
            match result {
                Result::Chunk(mesh, transform, (x, y, z)) => Some((mesh, transform, (x, y, z))),
            }
        } else {
            None
        }
    }

    pub fn quit_join(&mut self) {
        self.send_load.send(Command::Quit).unwrap();
        self.join_handle.take().map(JoinHandle::join);
    }
}
