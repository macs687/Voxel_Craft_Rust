use std::io;
use crate::voxels::{Block, BlockRegistry};

pub struct BlocksController {
    pub blocks: BlockRegistry
}


impl BlocksController {
    pub fn init() -> Result<Self, io::Error> {
        let mut blocks = BlockRegistry::new();

        Ok(Self{
            blocks
        })
    }


    pub fn setup_blocks(&mut self) {
        let mut block = Block::new(0, 0);
        block.draw_group = 1;
        block.light_passing = true;
        self.blocks.blocks[block.id as usize] = Some(block.clone());

        // STONE
        block = Block::new(1, 2);
        self.blocks.blocks[block.id as usize] = Some(block.clone());

        // GRASS
        block = Block::new(2, 4);
        block.texture_faces[2] = 2;
        block.texture_faces[3] = 1;
        self.blocks.blocks[block.id as usize] = Some(block.clone());

        // LAMP
        block = Block::new(3, 3);
        block.emission[0] = 11;
        block.emission[1] = 11;
        block.emission[2] = 6;
        self.blocks.blocks[block.id as usize] = Some(block.clone());

        // GLASS
        block = Block::new(4, 5);
        block.draw_group = 2;
        block.light_passing = true;
        self.blocks.blocks[block.id as usize] = Some(block.clone());

        // GLASS
        block = Block::new(5, 6);
        self.blocks.blocks[block.id as usize] = Some(block.clone());
    }
}