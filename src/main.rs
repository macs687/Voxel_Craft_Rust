use glfw::ffi::{KEY_0, KEY_F1};
use math::*;
use settings::*;

use crate::assets::{Assets, BlocksController};
use crate::files::write_binary_file;
use crate::graphics::mesh::Mesh;
use crate::voxels::chunk::CHUNK_VOL;
use crate::world_render::draw_world;
use graphics::VoxelRenderer;
use voxels::{Chunk, Chunks};
use window::{Camera, Events, Window};

mod settings;
mod window;
mod loaders;
mod graphics;
mod voxels;
mod files;
mod assets;
mod math;
mod world_render;

const VERTICES: [f32; 8] = [
    -0.01f32, -0.01f32,
    0.01f32, 0.01f32,

    -0.01f32, 0.01f32,
    0.01f32, -0.01f32
];

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const TITLE: &str = "Voxel_Craft";

#[allow(non_upper_case_globals)]
const attrs: [i32; 3] = [3, 2, 0];


fn main() {
    let mut window = Window::init(WIDTH, HEIGHT, TITLE).unwrap();
    let mut events = Events::init();

    events.setting(&mut window);

    println!("start loading assets");
    let assets = Assets::init().expect("fail load assets");
    println!("loading assets: ok");

    println!("start blocks init");
    let mut blocks_controller = BlocksController::init().expect("Failed init blocks controller");


    blocks_controller.setup_blocks();
    println!("blocks init: ok");

    println!("start chunks init");
    let mut chunks = Chunks::new(5, 2, 5);
    println!("chunks init: ok");

    println!("start meshes init");
    let mut meshes = Vec::with_capacity(chunks.volume);
    println!("meshes init: ok");

    println!("start renderer init");
    let mut renderer = VoxelRenderer::new(1024*1024*8);


    for i in 0..chunks.volume {
        let mesh = renderer.render(&chunks.chunks[i], &vec![]);
        meshes.push(mesh);
    }
    println!("renderer init: ok");

    window.clear_color(0.1, 0.2, 0.4, 0.8);

    window.setting_gl();

    println!("start crosshair init");
    let crosshair = Mesh::new(VERTICES.as_ptr(), 4, attrs.as_ptr());
    println!("crosshair init: ok");

    println!("start camera init");
    let mut camera = Camera::init(Vec3::new(10.0, 5.0, 10.0), 70.0_f32.to_radians());
    println!("camera init: ok");

    let mut last_time = window.glfw.get_time();
    let mut _delta:f64 = 0.0;

    let speed:f32 = 5.0f32;

    let mut choosen_block = 1;

    let mut cam_x = 0.0;
    let mut cam_y = 0.0;

    // let mut buffer = vec![0u8; chunks.volume * CHUNK_VOL];
    // let _result = read_binary_file("res/worlds/world.bin", &mut buffer);
    // chunks.read(&buffer);

    // println!("start lighting init");
    // let mut lighting = Lighting::init();
    //
    // lighting.on_world_loaded(&blocks_controller.blocks, &mut chunks);
    // println!("end lighting init");

    println!("start main loop");
    while !window.should_close() {
        let current_time = window.glfw.get_time();
        _delta = current_time - last_time;
        last_time = current_time;

        if events.j_pressed(ESCAPE) {
            window.close();
        }

        if events.pressed(Q){
            camera.position.z += _delta as f32 * speed;
        }

        if events.pressed(E){
            camera.position.z -= _delta as f32 * speed;
        }

        if events.pressed(A){
            camera.position -= camera.right * _delta as f32 * speed;
        }

        if events.pressed(D){
            camera.position += camera.right * _delta as f32 * speed;
        }

        if events.pressed(S){
            camera.position -= camera.up * _delta as f32 * speed;
        }

        if events.pressed(W){
            camera.position += camera.up * _delta as f32 * speed;
        }

        if events.j_pressed(TAB){
            window.window.set_cursor_mode(events.toggle_cursor());
        }

        for i in 0..7 {
            if events.j_pressed(KEY_0 + i) {
                choosen_block = i;
            }
        }

        if events.j_pressed(KEY_F1) {
            let mut buffer = vec![0u8; chunks.volume * CHUNK_VOL];
            chunks.write(&mut buffer);
            let _result = write_binary_file("res/worlds/world.bin", &buffer);
            println!("world saved in {} bytes", chunks.volume * CHUNK_VOL);
        }

        if events.cursor_locked {
            cam_y += -events.delta_y / (window.height() as f32) * 2.0;
            cam_x += -events.delta_x / (window.height() as f32) * 2.0;

            //    cam_y < -90.0_f32.to_radians() {   // ????
            //      cam_y = -90.0_f32.to_radians();
            // }
            if cam_y > 89.0_f32.to_radians() {
                cam_y = 89.0_f32.to_radians();
            }

            camera.rotation = Quat::IDENTITY;
            camera.rotate(cam_y, cam_x, 0.0);
        }

        {
            let mut end = Vec3::ZERO;
            let mut norm = Vec3::ZERO;
            let mut iend = Vec3::ZERO;

            if
            let Some(_vox) = chunks.ray_cast(
                camera.position,
                camera.front,
                10.0,
                &mut end,
                &mut norm,
                &mut iend
            )
            {
                if events.j_clicked(LCM) {
                    chunks.set(iend.x as isize, iend.y as isize, iend.z as isize, 0);
                }

                if events.j_clicked(PCM) {
                    chunks.set(
                        (iend.x + norm.x) as isize,
                        (iend.y + norm.y) as isize,
                        (iend.z + norm.z) as isize,
                        choosen_block
                    );
                }
            }
        }

        /// рендер


        let mut closes: Vec<Option<Chunk>> = vec![None; 27];

        for i in 0..chunks.volume {
            if let Some(chunk) = chunks.chunks.get_mut(i) {
                if !chunk.modified {
                    continue;
                }
                chunk.modified = false;
            }
            let chunk = &chunks.chunks[i];

            // if let Some(mesh) = meshes[i].take() {
            //     // Освобождаем ресурсы меша
            //     drop(mesh);
            // }

            // Инициализируем массив closes снова
            for elem in &mut closes {
                *elem = None;
            }

            for j in 0..chunks.volume {
                let other = &chunks.chunks[j];
                let ox = other.x - chunk.x;
                let oy = other.y - chunk.y;
                let oz = other.z - chunk.z;

                if ox.abs() > 1 || oy.abs() > 1 || oz.abs() > 1 {
                    continue;
                }

                let index = ((oy + 1) * 3 + (oz + 1)) * 3 + (ox + 1);
                closes[index as usize] = Some(other.clone());
            }

            let mesh = renderer.render(chunk, &closes);
            meshes[i] = mesh;
        }

        draw_world(&mut window, &assets, &camera, &chunks, &meshes, &crosshair);


        window.swap_buffers();
        window.poll_events();
        events.pull_events(&mut window);
    }
    println!("end main loop");

    window.terminate();
}