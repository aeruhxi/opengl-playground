use std::mem::size_of;

use glam::{Mat4, Vec2, Vec3, Vec3A};

use crate::opengl::{
    active_texture, buffer_data, draw_arrays, enable_vertex_attrib_array, gen_buffers, gen_vertex_arrays,
    vertex_attrib_pointer, VertexAttributeArray,
};

use super::{material::Material, texture2d::Texture2D};

pub struct SpriteRenderer {
    material: Material,
    quad_vao: VertexAttributeArray,
}

impl SpriteRenderer {
    pub fn new(material: Material) -> SpriteRenderer {
        let [quad_vao] = gen_vertex_arrays::<1>();
        let [vbo] = gen_buffers::<1>();

        #[rustfmt::skip]
        let vertices: [f32;12] = [
            0.0, 1.0,
            1.0, 0.0,
            0.0, 0.0,

            0.0, 1.0,
            1.0, 1.0,
            1.0, 0.0,
        ];

        quad_vao.bind();
        vbo.bind(gl::ARRAY_BUFFER);
        buffer_data(gl::ARRAY_BUFFER, &vertices, gl::STATIC_DRAW);
        vertex_attrib_pointer(0, 2, gl::FLOAT, false, 2 * size_of::<f32>(), 0);
        enable_vertex_attrib_array(0);

        SpriteRenderer { material, quad_vao }
    }

    pub fn draw(&self, texture: &Texture2D, position: Vec2, size: Vec2, rotate: f32, color: Vec3A) {
        self.material.use_();

        let translation = Mat4::from_translation(Vec3::from((position, 0.0)));
        let rotation = Mat4::from_translation(Vec3::from((0.5 * size.x, 0.5 * size.y, 0.0)))
            * Mat4::from_rotation_z(f32::to_radians(rotate))
            * Mat4::from_translation(Vec3::from((-0.5 * size.x, -0.5 * size.y, 0.0)));
        let scale = Mat4::from_scale(Vec3::from((size, 1.0)));

        let model = translation * rotation * scale;

        self.material.set_matrix4f("model", model);
        self.material.set_vector3f("spriteColor", color);

        active_texture(gl::TEXTURE0);
        texture.bind();

        self.quad_vao.bind();
        draw_arrays(gl::TRIANGLES, 0, 6);
    }
}
