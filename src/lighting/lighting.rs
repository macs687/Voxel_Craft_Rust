use crate::lighting::LightSolver;
use crate::voxels::{Blocks, Chunks, CHUNK_D, CHUNK_H, CHUNK_VOL, CHUNK_W};

pub struct Lighting {
    pub solver_r: LightSolver,
    pub solver_g: LightSolver,
    pub solver_b: LightSolver,
    pub solver_s: LightSolver,
}


impl Lighting {
    pub fn init() -> Self {
        let solver_r = LightSolver::new(0);
        println!("init red");
        let solver_g = LightSolver::new(1);
        println!("init green");
        let solver_b = LightSolver::new(2);
        println!("init blue");
        let solver_s = LightSolver::new(3);
        println!("init sun");

        Self {
            solver_r,
            solver_g,
            solver_b,
            solver_s
        }
    }


    pub fn clear(&mut self, chunks: &mut Chunks) {
        for y in 0..chunks.h {
            for z in 0..chunks.d {
                for x in 0..chunks.w {
                    if let Some(chunk) = chunks.get_mut_chunk(x as isize, y as isize, z as isize) {
                        for i in 0..CHUNK_VOL {
                            chunk.light_map.map[i] = 0;
                        }
                    }
                }
            }
        }
    }


    pub fn on_world_loaded(&mut self, blocks: &Blocks, chunks: &mut Chunks) {
        let h = chunks.h;
        let d = chunks.d;
        let w = chunks.w;

        for y in 0..h * CHUNK_H {
            for z in 0..d * CHUNK_D {
                for x in 0..w * CHUNK_W {
                    let vox = chunks.get_voxel(x as isize, y as isize, z as isize);
                    if let Some(vox) = vox {
                        if vox.id == 3 {
                            self.solver_r.add(x as isize, y as isize, z as isize, Some(15), chunks);
                            self.solver_g.add(x as isize, y as isize, z as isize, Some(15), chunks);
                            self.solver_b.add(x as isize, y as isize, z as isize, Some(15), chunks);
                        }
                    }
                    //println!("light init: {}", z * x * y);
                }
            }
        }

        for z in 0..d * CHUNK_D {
            for x in 0..w * CHUNK_W {
                for y in 0..h * CHUNK_H {
                    let vox = chunks.get_voxel(x as isize, y as isize, z as isize);
                    if let Some(vox) = vox {
                        if vox.id != 0 {
                            break;
                        }

                        let chunk = chunks.get_mut_chunk_by_voxel(x as isize, y as isize, z as isize);

                        if let Some(chunk) = chunk {
                            chunk.light_map.set_s((x % CHUNK_W) as isize, (y % CHUNK_H) as isize, (z % CHUNK_D) as isize, 0xf);
                        }
                    }
                    //println!("light init: {}", z * x * y);
                }
            }
        }

        for z in 0..d * CHUNK_D {
            for x in 0..w * CHUNK_W {
                for y in (0..=h * CHUNK_H - 1).rev() {
                    let vox = chunks.get_voxel(x as isize, y as isize, z as isize);
                    if let Some(vox) = vox {
                        if vox.id != 0 {
                            break;
                        }

                        if
                            chunks.get_light((x as isize) - 1, y as isize, z as isize, 3) == 0 ||
                            chunks.get_light((x as isize) + 1, y as isize, z as isize, 3) == 0 ||
                            chunks.get_light(x as isize, (y as isize) - 1, z as isize, 3) == 0 ||
                            chunks.get_light(x as isize, (y as isize) + 1, z as isize, 3) == 0 ||
                            chunks.get_light(x as isize, y as isize, (z as isize) - 1, 3) == 0 ||
                            chunks.get_light(x as isize, y as isize, (z as isize) + 1, 3) == 0
                        {
                            self.solver_s.add(x as isize, y as isize, z as isize, None, chunks);
                        }

                        if let Some(chunk) = chunks.get_mut_chunk_by_voxel(x as isize, y as isize, z as isize) {
                            chunk.light_map.set_s(x as isize % CHUNK_W, y as isize % CHUNK_H, z as isize % CHUNK_D, 0xf);
                        }
                    }
                    //println!("lighting inited: {}", z * x * y);
                }
            }
        }

        self.solver_r.solve(&blocks, chunks);
        self.solver_g.solve(&blocks, chunks);
        self.solver_b.solve(&blocks, chunks);
        self.solver_s.solve(&blocks, chunks);
    }


    pub fn on_block_set(
        &mut self,
        x: isize,
        y: isize,
        z: isize,
        id: u8,
        blocks: &Blocks,
        chunks: &mut Chunks
    ) {
        if id == 0 {
            self.solver_r.remove(x, y, z, chunks);
            self.solver_g.remove(x, y, z, chunks);
            self.solver_b.remove(x, y, z, chunks);

            self.solver_r.solve(&blocks, chunks);
            self.solver_g.solve(&blocks, chunks);
            self.solver_b.solve(&blocks, chunks);

            if chunks.get_light(x, y + 1, z, 3) == 0xf {
                for i in (0..=y).rev() {
                    let voxel = chunks.get_voxel(x, i, z);
                    if let Some(voxel) = voxel {
                        if voxel.id != 0 {
                            break;
                        }
                        self.solver_s.add(x, i, z, Some(0xf), chunks);
                    }
                }
            }

            let (x, y, z) = (x, y, z);

            self.solver_r.add(x, y + 1, z, None, chunks);
            self.solver_g.add(x, y + 1, z, None, chunks);
            self.solver_b.add(x, y + 1, z, None, chunks);
            self.solver_s.add(x, y + 1, z, None, chunks);
            self.solver_r.add(x, y - 1, z, None, chunks);
            self.solver_g.add(x, y - 1, z, None, chunks);
            self.solver_b.add(x, y - 1, z, None, chunks);
            self.solver_s.add(x, y - 1, z, None, chunks);
            self.solver_r.add(x + 1, y, z, None, chunks);
            self.solver_g.add(x + 1, y, z, None, chunks);
            self.solver_b.add(x + 1, y, z, None, chunks);
            self.solver_s.add(x + 1, y, z, None, chunks);
            self.solver_r.add(x - 1, y, z, None, chunks);
            self.solver_g.add(x - 1, y, z, None, chunks);
            self.solver_b.add(x - 1, y, z, None, chunks);
            self.solver_s.add(x - 1, y, z, None, chunks);
            self.solver_r.add(x, y, z + 1, None, chunks);
            self.solver_g.add(x, y, z + 1, None, chunks);
            self.solver_b.add(x, y, z + 1, None, chunks);
            self.solver_s.add(x, y, z + 1, None, chunks);
            self.solver_r.add(x, y, z - 1, None, chunks);
            self.solver_g.add(x, y, z - 1, None, chunks);
            self.solver_b.add(x, y, z - 1, None, chunks);
            self.solver_s.add(x, y, z - 1, None, chunks);

            self.solver_r.solve(&blocks, chunks);
            self.solver_g.solve(&blocks, chunks);
            self.solver_b.solve(&blocks, chunks);
            self.solver_s.solve(&blocks, chunks);
        } else {
            self.solver_r.remove(x, y, z, chunks);
            self.solver_g.remove(x, y, z, chunks);
            self.solver_b.remove(x, y, z, chunks);
            self.solver_s.remove(x, y, z, chunks);

            for i in (0..=y - 1).rev() {
                self.solver_s.remove(x, i, z, chunks);
                if let Some(voxel) = chunks.get_voxel(x, i - 1, z) {
                    if i == 0 || voxel.id != 0 {
                        break;
                    }
                }
            }

            self.solver_r.solve(&blocks, chunks);
            self.solver_g.solve(&blocks, chunks);
            self.solver_b.solve(&blocks, chunks);
            self.solver_s.solve(&blocks, chunks);

            let block = blocks.get(id);

            if let Some(block) = block {
                if block.emission[0] != 0 || block.emission[0] != 0 || block.emission[0] != 0 {
                    let (x, y, z) = (x, y, z);

                    self.solver_r.add(x, y, z, Some(block.emission[0]), chunks);
                    self.solver_g.add(x, y, z, Some(block.emission[1]), chunks);
                    self.solver_b.add(x, y, z, Some(block.emission[2]), chunks);

                    self.solver_r.solve(&blocks, chunks);
                    self.solver_g.solve(&blocks, chunks);
                    self.solver_b.solve(&blocks, chunks);
                }
            }
        }
    }
}