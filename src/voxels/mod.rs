use noise::{NoiseFn, OpenSimplex};

pub use chunks::Chunks;

use crate::lighting::lightmap::Lightmap;

pub mod chunks;

#[derive(Clone, Copy, Debug)]
pub struct Voxel {
    pub id: u8,
}

pub const CHUNK_W: usize = 16;
pub const CHUNK_H: usize = 16;
pub const CHUNK_D: usize = 16;
pub const CHUNK_VOL: usize = CHUNK_W * CHUNK_H * CHUNK_D;

#[derive(Clone)]
pub struct Chunk {
    pub x: isize,
    pub y: isize,
    pub z: isize,
    pub modified: bool,
    pub voxels: [Voxel; CHUNK_VOL],
    pub lightmap: Lightmap 
}

impl Chunk {
    pub fn new(x_pos: isize, y_pos: isize, z_pos: isize) -> Self {
        let mut voxels = [Voxel {id: 0}; CHUNK_VOL];
        let perlin = OpenSimplex::new(1);
        for z in 0..CHUNK_D as isize {
            for x in 0..CHUNK_W as isize {
                let real_x = x + x_pos * CHUNK_W as isize;
                let real_z = z + z_pos * CHUNK_D as isize;
                //let height = perlin.get([(x as f64) * 0.0125, (z as f64) * 0.0125]);
                for y in 0..CHUNK_H as isize {
                    let real_y = y as isize + y_pos * CHUNK_H as isize;
                    let id = perlin.get([(real_x as f64) * 0.0125, (real_y as f64) * 0.0125, (real_z as f64) * 0.0125]) > 0.1;
                    let chunk_index = ((y * CHUNK_D as isize + z) * CHUNK_W as isize + x) as usize;
                    if real_y <= 2 {
                        voxels[chunk_index].id = 2;
                    } else {
                        voxels[chunk_index].id = id as u8;
                    }
                }
            }
        }
        Chunk { x: x_pos, y: y_pos, z: z_pos, modified: true, voxels, lightmap: Lightmap::new() }
    }
}


// Block

const BLOCK_COUNT: usize = 256;

#[derive(Clone)]
pub struct Block {
    pub id: u32,
    pub texture_faces: [i32; 6],
    pub emission: [u8; 3],
    pub draw_group: u8,
    pub light_passing: bool,
}

pub struct BlockRegistry {
    pub blocks: Vec<Option<Block>>,
}

impl BlockRegistry {
    pub fn new() -> Self {
        Self { blocks: vec![None;BLOCK_COUNT]  }
    } 
    pub fn get(&self, id: u8) -> Option<&Block> {
        self.blocks[id as usize].as_ref()
    }
}

impl Block {
    pub fn new(id: u32, texture: i32) -> Self {
        Self {
            id,
            texture_faces: [texture; 6],
            emission: [0; 3],
            draw_group: 0,
            light_passing: false,
        }
    }
}