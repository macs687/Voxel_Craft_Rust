use gl::{DEPTH_BUFFER_BIT, LINES};
use glam::{vec3, Mat4};
use crate::assets::Assets;
use crate::graphics::linebatch::LineBatch;
use crate::graphics::mesh::Mesh;
use crate::settings::TRIANGLES;
use crate::voxels::{CHUNK_D, CHUNK_H, CHUNK_W};
use crate::voxels::chunks::Chunks;
use crate::window::{Camera, Window};
use std::borrow::Borrow;

pub fn draw_world(assets: &Assets, camera: &Camera, window: &Window, chunks: &Chunks, meshes: &Vec<Option<Mesh>>, crosshair: &Mesh, line_batch: &mut LineBatch ) {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
    }

    // Используем шейдер
    assets.shader.use_shader();

    assets.shader.uniform_matrix(
        "projview",
        camera.get_projection(window.width() as f32, window.height() as f32) * camera.get_view()
    );

    // Привязываем текстуру
    assets.texture.bind();

    let mut _model = Mat4::IDENTITY;
    for i in 0..chunks.volume {
        let chunk = &chunks.chunks[i];
        let mesh = meshes[i].borrow();
        _model =
            Mat4::IDENTITY *
                Mat4::from_translation(
                    vec3(
                        (chunk.x as f32) * (CHUNK_W as f32) + 0.5,
                        (chunk.y as f32) * (CHUNK_H as f32) + 0.5,
                        (chunk.z as f32) * (CHUNK_D as f32) + 0.5
                    )
                );
        assets.shader.uniform_matrix("model", _model);
        if let Some(mesh) = mesh {
            mesh.draw(TRIANGLES);
        }
    }

    assets.crosshair_shader.use_shader();
    crosshair.draw(LINES);

    assets.lines_shader.use_shader();
    assets.shader.uniform_matrix(
        "projview",
        camera.get_projection(window.width() as f32, window.height() as f32) * camera.get_view()
    );

    line_batch.line(0.0, 0.0, 0.0, 0.0, 10.0, 0.0, 1.0, 0.0, 0.0, 1.0);
    unsafe {
        gl::LineWidth(2.0);
    }
    line_batch.render();
}