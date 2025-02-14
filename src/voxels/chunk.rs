use crate::voxels::Voxel;

/// размер чанка по X
pub const CHUNK_W: isize = 16;
/// размер чанка по Y
pub const CHUNK_H: isize = 16;
/// размер чанка по Z
pub const CHUNK_D: isize = 16;
/// суммарное колличество блоков в чанке
pub const CHUNK_VOL: usize = (CHUNK_W * CHUNK_H * CHUNK_D) as usize;


#[derive(Debug, Clone)]
pub struct Chunk {
    pub voxels: Box<[Voxel; CHUNK_VOL]>,
    pub(crate) x: isize,
    pub(crate) y: isize,
    pub(crate) z: isize,
    pub(crate) modified: bool
}


impl Chunk {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        let mut voxels = Box::new([Voxel { id: 0 }; CHUNK_VOL]);
        for y in 0..CHUNK_H {
            for z in 0..CHUNK_D {
                for x in 0..CHUNK_W {
                    let id = if y as f32 <= (f32::sin(x as f32 * 0.8) * 0.5 + 0.5) * 10.0 {
                        1
                    } else {
                        0
                    };
                    if y <= 2  {
                        voxels[((y * CHUNK_D + z) * CHUNK_W + x) as usize].id = 4;
                    }
                    if y >= 3 && y <= 5 {
                        voxels[((y * CHUNK_D + z) * CHUNK_W + x) as usize].id = 6;
                    }
                    else {
                        voxels[((y * CHUNK_D + z) * CHUNK_W + x) as usize].id = id;
                    }
                }
            }
        }

        Chunk { voxels, x, y, z, modified: true }
    }
}