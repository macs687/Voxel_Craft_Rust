use crate::assets::Assets;
use crate::graphics::mesh::Mesh;
use crate::settings::TRIANGLES;
use crate::voxels::chunk::{CHUNK_D, CHUNK_H, CHUNK_W};
use crate::voxels::Chunks;
use crate::window::{Camera, Window};
use gl::LINES;
use glam::{vec3, Mat4};

pub fn draw_world(window: &mut Window, assets: &Assets, camera: &Camera, chunks: &Chunks, meshes: &Vec<Mesh>, crosshair: &Mesh) {
    window.gl_clear();

    assets.shader.use_shader();
    assets.shader.uniform_matrix("preview", camera.get_projection(window.width() as f32, window.height() as f32) * camera.get_view());
    assets.texture.bind();

    let mut model = Mat4::IDENTITY;
    model *= Mat4::from_translation(vec3(0.5, 0.0, 0.0));

    for i in 0..chunks.volume {
        let chunk = &chunks.chunks[i];
        let mesh = &meshes[i];
        model =
            Mat4::IDENTITY *
                Mat4::from_translation(
                    vec3(
                        ((chunk.x * CHUNK_W) as f32) + 0.5,
                        ((chunk.y * CHUNK_H) as f32) + 0.5,
                        ((chunk.z * CHUNK_D) as f32) + 0.5
                    )
                );
        assets.shader.uniform_matrix("model", model);
        mesh.draw(TRIANGLES);

    }


    assets.crosshair_shader.use_shader();
    crosshair.draw(LINES);
}