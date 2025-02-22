use glam::Vec3;

use super::{ Chunk, Voxel, CHUNK_D, CHUNK_H, CHUNK_W };


#[derive(Clone)]
pub struct Chunks {
    pub chunks: Vec<Chunk>,
    pub volume: usize,
    pub w: usize,
    pub h: usize,
    pub d: usize,
}

impl Chunks {
    pub fn new(w: usize, h: usize, d: usize) -> Self {
        let volume = w * h * d;
        let mut chunks = Vec::with_capacity(volume);

        for y in 0..h {
            for z in 0..d {
                for x in 0..w {
                    let chunk = Chunk::new(x as isize, y as isize, z as isize);
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
    pub fn _get_mut_voxel(&mut self, x: isize, y: isize, z: isize) -> Option<&mut Voxel> {
        match self.calculate_indices(x, y, z) {
            Some((chunk_index, voxel_index, _, _, _, _, _, _)) => {
                let chunk = self.chunks.get_mut(chunk_index);
                if let Some(chunk) = chunk {
                    chunk.voxels.get_mut(voxel_index)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn get_light(&self, x: isize, y: isize, z: isize, channel: usize) -> u8 {
        match self.calculate_indices(x, y, z) {
            Some((chunk_index, _, _, _, _, lx, ly, lz)) => {
                let chunk = &self.chunks[chunk_index];
                chunk.lightmap.get(lx as usize, ly as usize, lz as usize, channel)
            }
            None => 0,
        }
    }
    
    pub fn _get_chunk_by_voxel(&self, x: isize, y: isize, z: isize) -> Option<&Chunk> {
        match self.calculate_indices(x, y, z) {
            Some((chunk_index, _, _, _, _, _, _, _)) => self.chunks.get(chunk_index),
            None => None,
        }
    }

    pub fn get_mut_chunk_by_voxel(&mut self, x: isize, y: isize, z: isize) -> Option<&mut Chunk> {
        match self.calculate_indices(x, y, z) {
            Some((chunk_index, _, _, _, _, _, _, _)) => self.chunks.get_mut(chunk_index),
            None => None,
        }
    }

    pub fn _get_chunk(&self, x: isize, y: isize, z: isize) -> Option<&Chunk> {
        if
            x < 0 ||
            y < 0 ||
            z < 0 ||
            x >= (self.w as isize) ||
            y >= (self.h as isize) ||
            z >= (self.d as isize)
        {
            return None;
        }
        let chunk_index = (y * (self.d as isize) + z) * (self.w as isize) + x;
        Some(&self.chunks[chunk_index as usize])
    }

    pub fn get_mut_chunk(&mut self, x: isize, y: isize, z: isize) -> Option<&mut Chunk> {
        if
            x < 0 ||
            y < 0 ||
            z < 0 ||
            x >= (self.w as isize) ||
            y >= (self.h as isize) ||
            z >= (self.d as isize)
        {
            return None;
        }
        let chunk_index = (y * (self.d as isize) + z) * (self.w as isize) + x;
        Some(&mut self.chunks[chunk_index as usize])
    }

    pub fn set(&mut self, x: isize, y: isize, z: isize, id: i32) {
        match self.calculate_indices(x, y, z) {
            Some((chunk_index, voxel_index, cx, cy, cz, lx, ly, lz)) => {
                let chunk = self.chunks.get_mut(chunk_index);
                if let Some(chunk) = chunk {
                    chunk.voxels[voxel_index].id = id as u8;
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

                    if lx == (CHUNK_W as isize) - 1 {
                        if let Some(chunk) = self.get_mut_chunk(cx + 1, cy, cz) {
                            chunk.modified = true;
                        }
                    }
                    if ly == (CHUNK_H as isize) - 1 {
                        if let Some(chunk) = self.get_mut_chunk(cx, cy + 1, cz) {
                            chunk.modified = true;
                        }
                    }
                    if lz == (CHUNK_D as isize) - 1 {
                        if let Some(chunk) = self.get_mut_chunk(cx, cy, cz + 1) {
                            chunk.modified = true;
                        }
                    }
                }
            }
            None => {
                return;
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
        iend: &mut Vec3
    ) -> Option<&Voxel> {
        let px = a.x;
        let py = a.y;
        let pz = a.z;

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

        let xdist = if stepx > 0.0 { (ix as f32) + 1.0 - px } else { px - (ix as f32) };
        let ydist = if stepy > 0.0 { (iy as f32) + 1.0 - py } else { py - (iy as f32) };
        let zdist = if stepz > 0.0 { (iz as f32) + 1.0 - pz } else { pz - (iz as f32) };

        let mut tx_max = tx_delta * xdist;
        let mut ty_max = ty_delta * ydist;
        let mut tz_max = tz_delta * zdist;

        let mut stepped_index = -1;

        while t <= max_dist {
            if let Some(voxel) = self.get_voxel(ix, iy, iz) {
                if voxel.id != 0 {
                    end.x = px + t * dx;
                    end.y = py + t * dy;
                    end.z = pz + t * dz;

                    iend.x = ix as f32;
                    iend.y = iy as f32;
                    iend.z = iz as f32;

                    norm.x = 0.0;
                    norm.y = 0.0;
                    norm.z = 0.0;
                    match stepped_index {
                        0 => {
                            norm.x = -stepx;
                        }
                        1 => {
                            norm.y = -stepy;
                        }
                        2 => {
                            norm.z = -stepz;
                        }
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
        norm.x = 0.0;
        norm.y = 0.0;
        norm.z = 0.0;
        None
    }
    pub fn write(&self, dest: &mut [u8]) {
        let mut index = 0;
        for chunk in &self.chunks {
            for voxel in &chunk.voxels {
                dest[index] = voxel.id;
                index += 1;
            }
        }
    }

    pub fn read(&mut self, source: &[u8]) {
        let mut index = 0;
        for chunk in &mut self.chunks {
            for voxel in &mut chunk.voxels {
                voxel.id = source[index];
                index += 1;
            }
            chunk.modified = true;
        }
    }
    fn calculate_indices(
        &self,
        x: isize,
        y: isize,
        z: isize
    ) -> Option<(usize, usize, isize, isize, isize, isize, isize, isize)> {
        let mut cx = x / (CHUNK_W as isize);
        let mut cy = y / (CHUNK_H as isize);
        let mut cz = z / (CHUNK_D as isize);
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
            cx >= (self.w as isize) ||
            cy >= (self.h as isize) ||
            cz >= (self.d as isize)
        {
            return None;
        }
        let chunk_index = ((cy * (self.d as isize) + cz) * (self.w as isize) + cx) as usize;
        let lx = x - cx * (CHUNK_W as isize);
        let ly = y - cy * (CHUNK_H as isize);
        let lz = z - cz * (CHUNK_D as isize);
        let voxel_index = ((ly * (CHUNK_D as isize) + lz) * (CHUNK_W as isize) + lx) as usize;
        Some((chunk_index, voxel_index, cx, cy, cz, lx, ly, lz))
    }
}
