use std::collections::HashMap;
use std::io;
use crate::graphics::{load_shader, Shader, Texture};
use crate::loaders::load_texture;


pub struct Assets {
    pub shader: Shader,
    pub crosshair_shader: Shader,
    pub lines_shader: Shader,
    pub texture: Texture
}


impl Assets {
    pub fn init() -> Result<Self, io::Error> {
        let shader = load_shader("res/main.glslv","res/main.glslf");

        match shader {
            Ok(shader) => {
                println!("load main shader: ok");

                let crosshair_shader = load_shader("res/crosshair.glslv","res/crosshair.glslf");

                match crosshair_shader {
                    Ok(crosshair_shader) => {
                        println!("load crosshair shader: ok");

                        let lines_shader = load_shader("res/lines.glslv","res/lines.glslf");

                        match lines_shader {
                            Ok(lines_shader) => {
                                println!("load lines shader: ok");

                                let texture = load_texture("res/block.png");

                                match texture {
                                    Ok(texture) => {
                                        println!("load texture: ok");

                                        println!(" assets load completed ");
                                        Ok(Self {
                                            shader,
                                            crosshair_shader,
                                            lines_shader,
                                            texture
                                        })
                                    }Err(..) => { Err(io::Error::new(io::ErrorKind::Other, "failed to load texture")) }
                                }
                            }Err(..) => { Err(io::Error::new(io::ErrorKind::Other, "failed to load lines shader")) }
                        }
                    }Err(..) => { Err(io::Error::new(io::ErrorKind::Other, "failed to load crosshair shader")) }
                }
            }Err(..) => { Err(io::Error::new(io::ErrorKind::Other, "failed to load main shader")) }
        }
    }
}