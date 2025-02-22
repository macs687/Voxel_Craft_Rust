use std::io;
use crate::graphics::{load_shader, Shader, Texture};
use crate::loaders::png_loading::load_texture;

pub struct Assets {
    pub shader: Shader,
    pub crosshair_shader: Shader,
    pub lines_shader: Shader,
    pub texture: Texture
}


impl Assets {
    pub fn init() -> Result<Self, io::Error> {
        let shader = load_shader("res/main.glslv","res/main.glslf").expect("load main shader: Error");
        println!("load main shader: ok");

        let crosshair_shader = load_shader("res/crosshair.glslv", "res/crosshair.glslf").expect("load crosshair shader: Error");
        println!("load crosshair shader: ok");

        let lines_shader = load_shader("res/lines.glslv","res/lines.glslf").expect("load lines shader: Error");
        println!("load lines shader: ok");

        let texture = load_texture("res/block.png").expect("load texture: Error");
        println!("load texture: ok");


        Ok(Self{
            shader,
            crosshair_shader,
            lines_shader,
            texture
        })
    }

}