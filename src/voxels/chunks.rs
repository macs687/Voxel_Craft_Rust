use glam::Vec3;
use crate::voxels::chunk::{CHUNK_D, CHUNK_H, CHUNK_W};
use crate::voxels::voxel::Voxel;
use super::Chunk;


#[derive(Clone)]
pub struct Chunks {
    pub chunks: Vec<Chunk>,
    pub volume: usize,
    pub w: isize,
    pub h: isize,
    pub d: isize,
}


impl Chunks {
    pub fn new(w: isize, h: isize, d: isize) -> Self {
        let volume = (w * h * d) as usize;
        let mut chunks = Vec::with_capacity(volume);

        for y in 0..h {
            for z in 0..d {
                for x in 0..w {
                    let chunk = Chunk::new(x, y, z);
                    chunks.push(chunk);
                }
            }
        }

        Chunks {
            chunks,
            volume,
            w,
            h,
            d,
        }
    }


    pub fn get(&self, x: isize, y: isize, z: isize) -> Option<&Voxel> {
        let mut cx = x / CHUNK_W;
        let mut cy = y / CHUNK_H;
        let mut cz = z / CHUNK_D;

        if x < 0 {
            cx -= 1;
        }
        if y < 0 {
            cy -= 1;
        }
        if z < 0 {
            cz -= 1;
        }

        if
        cx < 0 ||
            cy < 0 ||
            cz < 0 ||
            cx >= (self.w) ||
            cy >= (self.h) ||
            cz >= (self.d)
        {
            return None;
        }

        let chunk_index = ((cy * (self.d) + cz) * (self.w) + cx) as usize;
        let chunk = &self.chunks[chunk_index];

        let lx = x - cx * (CHUNK_W);
        let ly = y - cy * (CHUNK_H);
        let lz = z - cz * (CHUNK_D);

        let voxel_index = ((ly * (CHUNK_D) + lz) * (CHUNK_W) + lx) as usize;
        Some(&chunk.voxels[voxel_index])
    }


    pub fn get_chunk(&self, x: isize, y: isize, z: isize) -> Option<&Chunk> {
        if
        x < 0 ||
            y < 0 ||
            z < 0 ||
            x >= (self.w) ||
            y >= (self.h) ||
            z >= (self.d)
        {
            return None;
        }
        let chunk_index = (y * (self.d) + z) * (self.w) + x;
        Some(&self.chunks[chunk_index as usize])
    }


    pub fn get_mut_chunk(&mut self, x: isize, y: isize, z: isize) -> Option<&mut Chunk> {
        if
        x < 0 ||
            y < 0 ||
            z < 0 ||
            x >= (self.w) ||
            y >= (self.h) ||
            z >= (self.d)
        {
            return None;
        }
        let chunk_index = (y * (self.d) + z) * (self.w) + x;
        Some(&mut self.chunks[chunk_index as usize])
    }

    pub fn set(&mut self, x: isize, y: isize, z: isize, id: i32) {
        let mut cx = x / (CHUNK_W);
        let mut cy = y / (CHUNK_H);
        let mut cz = z / (CHUNK_D);
        if x < 0 {
            cx -= 1;
        }
        if y < 0 {
            cy -= 1;
        }
        if z < 0 {
            cz -= 1;
        }
        if
        cx < 0 ||
            cy < 0 ||
            cz < 0 ||
            cx >= (self.w) ||
            cy >= (self.h) ||
            cz >= (self.d)
        {
            return;
        }
        let chunk_index = ((cy * (self.d) + cz) * (self.w) + cx) as usize;
        let chunk = &mut self.chunks[chunk_index];
        let lx = x - cx * (CHUNK_W);
        let ly = y - cy * (CHUNK_H);
        let lz = z - cz * (CHUNK_D);
        //println!("id: {}, id: {}", id, id as u8);
        chunk.voxels[((ly * (CHUNK_D) + lz) * (CHUNK_W) + lx) as usize].id = id as u8;
        chunk.modified = true;

        if lx == 0 {
            if let Some(chunk) = self.get_mut_chunk(cx - 1, cy, cz) {
                chunk.modified = true;
            }
        }
        if ly == 0 {
            if let Some(chunk) = self.get_mut_chunk(cx, cy - 1, cz) {
                chunk.modified = true;
            }
        }
        if lz == 0 {
            if let Some(chunk) = self.get_mut_chunk(cx, cy, cz - 1) {
                chunk.modified = true;
            }
        }

        if lx == (CHUNK_W) - 1 {
            if let Some(chunk) = self.get_mut_chunk(cx + 1, cy, cz) {
                chunk.modified = true;
            }
        }
        if ly == (CHUNK_H) - 1 {
            if let Some(chunk) = self.get_mut_chunk(cx, cy + 1, cz) {
                chunk.modified = true;
            }
        }
        if lz == (CHUNK_D) - 1 {
            if let Some(chunk) = self.get_mut_chunk(cx, cy, cz + 1) {
                chunk.modified = true;
            }
        }
    }


    pub fn ray_cast(
        &self,
        a: Vec3,
        dir: Vec3,
        max_dist: f32,
        end: &mut Vec3,
        norm: &mut Vec3,
        iend: &mut Vec3,
    ) -> Option<&Voxel> {
        let mut px = a.x;
        let mut py = a.y;
        let mut pz = a.z;

        let dx = dir.x;
        let dy = dir.y;
        let dz = dir.z;

        let mut t = 0.0;
        let mut ix = px.floor() as isize;
        let mut iy = py.floor() as isize;
        let mut iz = pz.floor() as isize;

        let stepx = if dx > 0.0 { 1.0 } else { -1.0 };
        let stepy = if dy > 0.0 { 1.0 } else { -1.0 };
        let stepz = if dz > 0.0 { 1.0 } else { -1.0 };

        let infinity = f32::INFINITY;

        let tx_delta = if dx == 0.0 { infinity } else { 1.0 / dx.abs() };
        let ty_delta = if dy == 0.0 { infinity } else { 1.0 / dy.abs() };
        let tz_delta = if dz == 0.0 { infinity } else { 1.0 / dz.abs() };

        let xdist = if stepx > 0.0 { ix as f32 + 1.0 - px } else { px - ix as f32 };
        let ydist = if stepy > 0.0 { iy as f32 + 1.0 - py } else { py - iy as f32 };
        let zdist = if stepz > 0.0 { iz as f32 + 1.0 - pz } else { pz - iz as f32 };

        let mut tx_max = tx_delta * xdist;
        let mut ty_max = ty_delta * ydist;
        let mut tz_max = tz_delta * zdist;

        let mut stepped_index = -1;

        while t <= max_dist {
            if let Some(voxel) = self.get(ix, iy, iz) {
                if voxel.id != 0 {
                    end.x = px + t * dx;
                    end.y = py + t * dy;
                    end.z = pz + t * dz;

                    iend.x = ix as f32;
                    iend.y = iy as f32;
                    iend.z = iz as f32;

                    norm.x = 0.;
                    norm.y = 0.;
                    norm.z = 0.0;
                    match stepped_index {
                        0 => norm.x = -stepx,
                        1 => norm.y = -stepy,
                        2 => norm.z = -stepz,
                        _ => (),
                    }
                    return Some(voxel);
                }
            }

            if tx_max < ty_max {
                if tx_max < tz_max {
                    ix += stepx as isize;
                    t = tx_max;
                    tx_max += tx_delta;
                    stepped_index = 0;
                } else {
                    iz += stepz as isize;
                    t = tz_max;
                    tz_max += tz_delta;
                    stepped_index = 2;
                }
            } else {
                if ty_max < tz_max {
                    iy += stepy as isize;
                    t = ty_max;
                    ty_max += ty_delta;
                    stepped_index = 1;
                } else {
                    iz += stepz as isize;
                    t = tz_max;
                    tz_max += tz_delta;
                    stepped_index = 2;
                }
            }
        }

        iend.x = ix as f32;
        iend.y = iy as f32;
        iend.z = iz as f32;

        end.x = px + t * dx;
        end.y = py + t * dy;
        end.z = pz + t * dz;
        norm.x = 0.;
        norm.y = 0.;
        norm.z = 0.0;
        None
    }


    pub fn write(&self, dest: &mut [u8]) {
        let mut index = 0;
        for chunk in &self.chunks {
            for voxel in chunk.voxels {
                dest[index] = voxel.id;
                index += 1;
            }
        }
    }


    pub fn read(&mut self, source: &[u8]) {
        let mut index = 0;
        for chunk in &mut self.chunks {
            for voxel in chunk.voxels.iter_mut() {
                voxel.id = source[index];
                index += 1;
            }
            chunk.modified = true;
        }
    }


    // pub fn get_light(&self, x: isize, y: isize, z: isize, channel: usize) -> u8 {
    //     match self.calculate_indices(x, y, z) {
    //         Some((chunk_index, _, _, _, _, lx, ly, lz)) => {
    //             let chunk = &self.chunks[chunk_index];
    //             chunk.light_map.get(lx, ly, lz, channel)
    //         }
    //         None => 0,
    //     }
    // }


    pub fn get_voxel<'a>(&self, x: isize, y: isize, z: isize) -> Option<&Voxel> {
        match self.calculate_indices(x, y, z) {
            Some((chunk_index, voxel_index, _, _, _, _, _, _)) => {
                let chunk = self.chunks.get(chunk_index);
                if let Some(chunk) = chunk {
                    chunk.voxels.get(voxel_index)
                } else {
                    None
                }
            }
            None => None,
        }
    }


    pub fn get_mut_chunk_by_voxel(&mut self, x: isize, y: isize, z: isize) -> Option<&mut Chunk> {
        match self.calculate_indices(x, y, z) {
            Some((chunk_index, _, _, _, _, _, _, _)) => self.chunks.get_mut(chunk_index),
            None => None,
        }
    }


    fn calculate_indices(
        &self,
        x: isize,
        y: isize,
        z: isize
    ) -> Option<(usize, usize, isize, isize, isize, isize, isize, isize)> {
        let mut cx = x / (CHUNK_W);
        let mut cy = y / (CHUNK_H);
        let mut cz = z / (CHUNK_D);
        if x < 0 {
            cx -= 1;
        }
        if y < 0 {
            cy -= 1;
        }
        if z < 0 {
            cz -= 1;
        }
        if
        cx < 0 ||
            cy < 0 ||
            cz < 0 ||
            cx >= (self.w) ||
            cy >= (self.h) ||
            cz >= (self.d)
        {
            return None;
        }
        let chunk_index = ((cy * (self.d) + cz) * (self.w) + cx) as usize;
        let lx = x - cx * (CHUNK_W);
        let ly = y - cy * (CHUNK_H);
        let lz = z - cz * (CHUNK_D);
        let voxel_index = ((ly * (CHUNK_D) + lz) * (CHUNK_W) + lx) as usize;
        Some((chunk_index, voxel_index, cx, cy, cz, lx, ly, lz))
    }
}