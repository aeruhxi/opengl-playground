use glam::{Mat4, Vec2, Vec3A, Vec4};

use crate::opengl::{create_program, create_shader, Program};

// Not deriving Copy for allowing heap allocated
// data in the future
#[derive(Debug, Clone)]
pub struct Material {
    program: Program,
}

impl Material {
    pub fn new(vertex_source: &str, fragment_source: &str) -> Self {
        let vertex_shader = create_shader(gl::VERTEX_SHADER);
        vertex_shader.source(vertex_source);
        vertex_shader.compile();
        if !vertex_shader.get(gl::COMPILE_STATUS) {
            let info_log = vertex_shader.get_info_log();
            println!("Error during compilation of vertex shader:\n{}", info_log);
        }

        let fragment_shader = create_shader(gl::FRAGMENT_SHADER);
        fragment_shader.source(fragment_source);
        fragment_shader.compile();
        if !fragment_shader.get(gl::COMPILE_STATUS) {
            let info_log = fragment_shader.get_info_log();
            println!("Error during compilation of fragment shader:\n{}", info_log);
        }

        let program = create_program();
        program.attach_shader(vertex_shader);
        program.attach_shader(fragment_shader);
        program.link();

        if !program.get(gl::LINK_STATUS) {
            let info_log = program.get_info_log();
            println!("Error during linking:\n{}", info_log);
        }

        vertex_shader.delete();
        fragment_shader.delete();

        Material { program }
    }

    pub fn use_(&self) {
        self.program.use_();
    }

    pub fn set_float(&self, name: &str, value: f32) {
        self.program.get_uniform_location(name).set_1f(value);
    }

    pub fn set_integer(&self, name: &str, value: i32) {
        self.program.get_uniform_location(name).set_1i(value);
    }

    pub fn set_vector2f(&self, name: &str, value: Vec2) {
        self.program.get_uniform_location(name).set_2f(value.x, value.y);
    }

    pub fn set_vector3f(&self, name: &str, value: Vec3A) {
        self.program
            .get_uniform_location(name)
            .set_3f(value.x, value.y, value.z);
    }

    pub fn set_vector4f(&self, name: &str, value: Vec4) {
        self.program
            .get_uniform_location(name)
            .set_4f(value.x, value.y, value.z, value.w);
    }

    pub fn set_matrix4f(&self, name: &str, value: Mat4) {
        self.program
            .get_uniform_location(name)
            .set_matrix4fv(1, false, &value.to_cols_array());
    }
}
