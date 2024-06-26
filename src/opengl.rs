use std::{
    ffi::{c_void, CString},
    mem::size_of,
    ptr::null,
    str,
};

use gl::types::{GLboolean, GLenum, GLint};

#[derive(Clone, Copy)]
pub struct Shader(u32);

pub fn create_shader(shader_type: GLenum) -> Shader {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        Shader(shader)
    }
}

impl Shader {
    pub fn source(self, source: &str) {
        unsafe {
            let c_source = CString::new(source).unwrap();

            gl::ShaderSource(self.0, 1, &c_source.as_ptr(), null());
        }
    }

    pub fn compile(self) {
        unsafe { gl::CompileShader(self.0) }
    }

    pub fn get(&self, pname: GLenum) -> bool {
        unsafe {
            let mut success = gl::FALSE as GLint;
            gl::GetShaderiv(self.0, pname, &mut success);
            success == 1
        }
    }

    pub fn get_info_log(self) -> String {
        const CAPACITY: usize = 512;
        let mut info_log: Vec<u8> = Vec::with_capacity(CAPACITY);
        let mut info_log_size: i32 = 0;

        unsafe {
            gl::GetShaderInfoLog(
                self.0,
                CAPACITY as i32,
                &mut info_log_size,
                info_log.as_mut_ptr().cast(),
            );

            info_log.set_len(info_log_size.try_into().unwrap());
        }

        String::from_utf8(info_log).unwrap()
    }

    pub fn delete(self) {
        unsafe {
            gl::DeleteShader(self.0);
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Program(u32);

pub fn create_program() -> Program {
    unsafe { Program(gl::CreateProgram()) }
}

impl Program {
    pub fn attach_shader(self, shader: Shader) {
        unsafe {
            gl::AttachShader(self.0, shader.0);
        }
    }

    pub fn link(self) {
        unsafe {
            gl::LinkProgram(self.0);
        }
    }

    pub fn get(self, pname: GLenum) -> bool {
        unsafe {
            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(self.0, pname, &mut success);
            success == 1
        }
    }

    pub fn get_info_log(self) -> String {
        const CAPACITY: usize = 512;
        let mut info_log: Vec<u8> = Vec::with_capacity(CAPACITY);
        let mut info_log_size: i32 = 0;

        unsafe {
            gl::GetProgramInfoLog(
                self.0,
                CAPACITY as i32,
                &mut info_log_size,
                info_log.as_mut_ptr().cast(),
            );
            info_log.set_len(info_log_size.try_into().unwrap());
        }

        String::from_utf8(info_log).unwrap()
    }

    pub fn use_(self) {
        unsafe {
            gl::UseProgram(self.0);
        }
    }

    pub fn get_uniform_location(self, name: &str) -> Uniform {
        let name = CString::new(name).unwrap();
        unsafe { Uniform(gl::GetUniformLocation(self.0, name.as_ptr())) }
    }
}

pub struct Uniform(i32);

impl Uniform {
    pub fn set_4f(self, f1: f32, f2: f32, f3: f32, f4: f32) {
        unsafe {
            gl::Uniform4f(self.0, f1, f2, f3, f4);
        }
    }

    pub fn set_1f(self, f: f32) {
        unsafe {
            gl::Uniform1f(self.0, f);
        }
    }

    pub fn set_2f(self, f1: f32, f2: f32) {
        unsafe {
            gl::Uniform2f(self.0, f1, f2);
        }
    }

    pub fn set_3f(self, f1: f32, f2: f32, f3: f32) {
        unsafe {
            gl::Uniform3f(self.0, f1, f2, f3);
        }
    }

    pub fn set_1i(self, i: i32) {
        unsafe {
            gl::Uniform1i(self.0, i);
        }
    }

    pub fn set_matrix4fv(self, count: i32, transpose: bool, matrix: &[f32]) {
        unsafe {
            gl::UniformMatrix4fv(self.0, count, bool_to_glbool(transpose), matrix.as_ptr());
        }
    }
}

#[derive(Clone, Copy)]
pub struct Buffer(u32);

pub fn gen_buffers<const N: usize>() -> [Buffer; N] {
    let mut buffer_ids: [u32; N] = [0; N];
    unsafe {
        gl::GenBuffers(N as i32, buffer_ids.as_mut_ptr());
    }

    buffer_ids.map(|id| Buffer(id))
}

impl Buffer {
    pub fn bind(self, target: GLenum) {
        unsafe { gl::BindBuffer(target, self.0) }
    }

    pub fn unbind(target: GLenum) {
        unsafe { gl::BindBuffer(target, 0) }
    }
}

#[derive(Clone, Copy)]
pub struct VertexAttributeArray(u32);

impl VertexAttributeArray {
    pub fn bind(self) {
        unsafe { gl::BindVertexArray(self.0) }
    }
}

pub fn gen_vertex_arrays<const N: usize>() -> [VertexAttributeArray; N] {
    let mut vao_ids: [u32; N] = [0; N];
    unsafe {
        gl::GenVertexArrays(N as i32, vao_ids.as_mut_ptr());
    }
    vao_ids.map(|id| VertexAttributeArray(id))
}

pub fn buffer_data<T>(target: GLenum, data: &[T], usage: GLenum) {
    unsafe {
        gl::BufferData(
            target,
            (data.len() * size_of::<T>()) as isize,
            data.as_ptr().cast(),
            usage,
        );
    }
}

pub fn vertex_attrib_pointer(index: u32, size: i32, type_: GLenum, normalized: bool, stride: usize, pointer: usize) {
    let gl_normalized = if normalized { gl::TRUE } else { gl::FALSE };
    unsafe {
        gl::VertexAttribPointer(
            index,
            size,
            type_,
            gl_normalized,
            stride as i32,
            pointer as *const c_void,
        );
    }
}

pub fn enable_vertex_attrib_array(index: u32) {
    unsafe {
        gl::EnableVertexAttribArray(index);
    }
}

pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::ClearColor(r, g, b, a);
    }
}

pub fn clear(mask: GLenum) {
    unsafe {
        gl::Clear(mask);
    }
}

pub fn draw_arrays(mode: GLenum, first: i32, count: usize) {
    unsafe { gl::DrawArrays(mode, first, count as i32) }
}

pub fn draw_elements(mode: GLenum, count: usize, type_: GLenum, indices: usize) {
    unsafe { gl::DrawElements(mode, count as i32, type_, indices as *const c_void) }
}

pub fn polygon_mode(face: GLenum, mode: GLenum) {
    unsafe { gl::PolygonMode(face, mode) }
}

fn bool_to_glbool(b: bool) -> GLboolean {
    match b {
        true => gl::TRUE,
        false => gl::FALSE,
    }
}

// Texture
#[derive(Debug, Clone, Copy)]
pub struct Texture(u32);

pub fn gen_textures<const N: usize>() -> [Texture; N] {
    let mut texture_ids: [u32; N] = [0; N];
    unsafe {
        gl::GenTextures(N as i32, texture_ids.as_mut_ptr());
    }
    texture_ids.map(|id| Texture(id))
}

impl Texture {
    pub fn bind(self, target: GLenum) {
        unsafe {
            gl::BindTexture(target, self.0);
        }
    }

    pub fn unbind(target: GLenum) {
        unsafe { gl::BindTexture(target, 0) }
    }
}

pub fn tex_image_2d(
    target: GLenum,
    level: GLenum,
    internal_format: GLenum,
    width: i32,
    height: i32,
    border: GLenum,
    format: GLenum,
    type_: GLenum,
    data: &[u8],
) {
    unsafe {
        gl::TexImage2D(
            target,
            level as i32,
            internal_format as i32,
            width,
            height,
            border as i32,
            format,
            type_,
            data.as_ptr().cast(),
        );
    }
}

pub fn tex_parameter_i(target: GLenum, pname: GLenum, param: GLenum) {
    unsafe {
        gl::TexParameteri(target, pname, param as i32);
    }
}

pub fn active_texture(texture: GLenum) {
    unsafe {
        gl::ActiveTexture(texture);
    }
}

pub fn blend_func(s_factor: GLenum, d_factor: GLenum) {
    unsafe {
        gl::BlendFunc(s_factor, d_factor);
    }
}

pub fn enable(cap: GLenum) {
    unsafe {
        gl::Enable(cap);
    }
}
