use crate::{
    asset::{AssetLoader, Command},
    entity::Entity,
    mesh::Mesh,
    registry::Registry,
    world::{chunks::Chunk, Chunks},
};

pub struct World {
    chunks: Chunks,
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: Chunks::new(16, 32, 0.1),
        }
    }

    pub fn generate_around(
        &mut self,
        position: [f32; 3],
        meshes: &mut Registry<Mesh>,
        entities: &mut Registry<Entity>,
        asset_loader: &mut AssetLoader,
    ) {
        self.chunks.clear_just_added();
        self.chunks.set_position(position);
        let diff = self.chunks.range_diff();
        for added in diff.added.iter() {
            for z in added[2].clone() {
                for y in added[1].clone() {
                    for x in added[0].clone() {
                        if let Some(previous_chunk) = self.chunks.get_chunk([x, y, z]) {
                            if previous_chunk.location.0 != x
                                || previous_chunk.location.1 != y
                                || previous_chunk.location.2 != z
                            {
                                asset_loader.request(Command::Load(x, y, z));
                                if let Some(previous_entity) = entities.get(&previous_chunk.entity) {
                                    meshes.remove(previous_entity.mesh_handle.clone());
                                    entities.remove(previous_chunk.entity.clone());
                                }
                            }
                        } else {
                            asset_loader.request(Command::Load(x, y, z));
                        }
                    }
                }
            }
        }
        if let Some((mesh, transform, location)) = asset_loader.try_retrieve() {
            let mesh_handle = meshes.add(mesh);
            let chunk = Chunk {
                location,
                entity: entities.add(Entity {
                    mesh_handle,
                    collision_shape: None,
                    transform,
                }),
                just_added: true,
            };
            self.chunks.set_chunk([location.0, location.1, location.2], Some(chunk));
        }
    }
}
