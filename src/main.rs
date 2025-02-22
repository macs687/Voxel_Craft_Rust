use settings::*;
use math::*;

use window::{Window, Events, Camera};
use assets::{Assets, BlocksController};
use voxels::{Chunks, Chunk, CHUNK_VOL};
use graphics::{VoxelRenderer, LineBatch, Mesh};
use lighting::Lighting;
use files::{read_binary_file, write_binary_file};
use world::draw_world;


mod settings;
mod math;
mod window;
mod assets;
mod voxels;
mod graphics;
mod lighting;
mod files;
mod world;
mod loaders;


const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

const VERTICES: [f32; 8] = [
    // x   | y
    -0.01, -0.01, 0.01, 0.01,

    -0.01, 0.01, 0.01, -0.01,
];

#[allow(non_upper_case_globals)]
const attrs: [i32; 2] = [2, 0]; // null terminator

fn main() {
    let mut window = Window::new(WIDTH, HEIGHT, "Window 2.0").unwrap();
    let mut events = Events::new();

    events.initialize(&mut window);

    let mut assets = Assets::init().expect("fail load assets");
    println!("load assets: ok");

    println!("start block init");
    let mut blocks_controller = BlocksController::init().unwrap();

    blocks_controller.setup_blocks();
    println!("blocks init: ok");

    println!("start chunks init");
    let mut chunks = Chunks::new(4, 4, 4);
    let mut meshes = Vec::with_capacity(chunks.volume);
    for _ in 0..chunks.volume {
        meshes.push(None);
    }
    println!("chunks init: ok");

    println!("start init renderer");
    let mut renderer = VoxelRenderer::new(1024 * 1024 * 8);
    let mut line_batch = LineBatch::new(4096);
    println!("renderer init: ok");

    window.gl_setting();

    let crosshair = Mesh::new(VERTICES.as_ptr(), 4, attrs.as_ptr());
    println!("crosshair init: ok");

    let mut camera = Camera::new(Vec3::new(5.0, 5.0, 20.0), (70.0_f32).to_radians());
    println!("camera init: ok");

    println!("start settings init");
    let mut last_time = window.glfw.get_time();
    let mut _delta: f32 = 0.0;

    let mut cam_x = 0.0;
    let mut cam_y = 0.0;

    let speed = 15.0;

    let mut choosen_block: i32 = 1;
    println!("settings init: ok");

    println!("start lighting init");
    let mut lighting = Lighting::new();

    lighting.on_world_loaded(&blocks_controller.blocks, &mut chunks);
    println!("lighting init: ok");

    println!("start world loaded");
    let mut buffer = vec![0u8; chunks.volume * CHUNK_VOL];
    let _result = read_binary_file("res/worlds/world.bin", &mut buffer);
    chunks.read(&buffer);

    lighting.clear(&mut chunks);
    lighting.on_world_loaded(&blocks_controller.blocks, &mut chunks);
    println!("finish world loaded");


    println!("start main loop");
    while !window.should_close() {
        let current_time = window.glfw.get_time();
        _delta = (current_time - last_time) as f32;
        last_time = current_time;

        if events.jpressed(ESCAPE) {
            window.close();
        }

        if events.jpressed(TAB) {
            window.window.set_cursor_mode(events.toggle_cursor());
        }

        for i in 0..5 {
            if events.jpressed(K_0 + i) {
                choosen_block = i;
            }
        }

        if events.pressed(W) {
            camera.position += camera.front * _delta * speed;
        }

        if events.pressed(S) {
            camera.position -= camera.front * _delta * speed;
        }

        if events.pressed(D) {
            camera.position -= camera.right * _delta * speed;
        }

        if events.pressed(A) {
            camera.position += camera.right * _delta * speed;
        }

        if events.cursor_locked {
            cam_y -= (-events.delta_y / (window.height() as f32)) * 2.0;
            cam_x += (-events.delta_x / (window.height() as f32)) * 2.0;

            if cam_y < -(89.0_f32).to_radians() {
                cam_y = -(89.0_f32).to_radians();
            }
            if cam_y > (89.0_f32).to_radians() {
                cam_y = (89.0_f32).to_radians();
            }

            camera.rotation = Quat::IDENTITY;
            camera.rotate(cam_y, cam_x, 0.0);
        }

        let mut end = glam::Vec3::default();
        let mut norm = glam::Vec3::default();
        let mut iend = glam::Vec3::default();

        if let Some(_vox) = chunks.ray_cast(
            camera.position,
            camera.front,
            10.0,
            &mut end,
            &mut norm,
            &mut iend)
        {
            line_batch.boxx(
                iend.x + 0.5,
                iend.y + 0.5,
                iend.z + 0.5,
                1.01,
                1.01,
                1.01,
                0.0,
                0.0,
                0.0,
                1.0
            );

            if events.jclicked(LCM) && events.cursor_locked {
                let x = iend.x as isize;
                let y = iend.y as isize;
                let z = iend.z as isize;

                chunks.set(x, y, z, 0);

                lighting.on_block_set(x, y, z, 0, &blocks_controller.blocks, &mut chunks);
            }

            if events.jclicked(PCM) && events.cursor_locked {
                let x = (iend.x + norm.x) as isize;
                let y = (iend.y + norm.y) as isize;
                let z = (iend.z + norm.z) as isize;
                chunks.set(x, y, z, choosen_block);

                lighting.on_block_set(x, y, z, choosen_block as u8, &blocks_controller.blocks, &mut chunks);
            }
        }

        let mut closes: Vec<Option<Chunk>> = vec![None; 27];

        for i in 0..chunks.volume {
            if let Some(chunk) = chunks.chunks.get_mut(i) {
                if !chunk.modified {
                    continue;
                }
                chunk.modified = false;
            }
            let chunk = &chunks.chunks[i];

            if let Some(mesh) = meshes[i].take() {
                // Освобождаем ресурсы меша
                drop(mesh);
            }

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

            let mesh = renderer.render(chunk, &closes, &blocks_controller.blocks);
            meshes[i] = Some(mesh);
        }


        draw_world(&assets, &camera, &window, &chunks, &meshes, &crosshair, &mut line_batch);

        window.swap_buffers();
        events.pull_events(&mut window);
    }
    println!("finish main loop");

    println!("saving world");
    let mut buffer = vec![0u8; chunks.volume * CHUNK_VOL];
    chunks.write(&mut buffer);
    let _result = write_binary_file("res/worlds/world.bin", &buffer);
    println!("world saved in {} bytes on res/worlds/world.bin", chunks.volume * CHUNK_VOL, );

    window.terminate();
}