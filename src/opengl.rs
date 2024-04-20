use std::{
    ffi::{c_void, CString},
    mem::size_of,
    ptr::null,
    str,
};

use gl::types::{GLchar, GLenum, GLint};

#[derive(Clone, Copy)]
pub struct Shader(u32);

pub fn create_shader(shader_type: GLenum) -> Shader {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        Shader(shader)
    }
}

impl Shader {
    pub fn source(&self, source: &str) {
        unsafe {
            let c_source = CString::new(source).expect("Only string without 0 byte is accepted");

            gl::ShaderSource(self.0, 1, &c_source.as_ptr(), null());
        }
    }

    pub fn compile(&self) {
        unsafe { gl::CompileShader(self.0) }
    }

    pub fn get(&self, pname: GLenum) -> bool {
        unsafe {
            let mut success = gl::FALSE as GLint;
            gl::GetShaderiv(self.0, pname, &mut success);
            success == 1
        }
    }

    pub fn get_info_log(&self) -> String {
        unsafe {
            const CAPACITY: usize = 512;
            let mut info_log: Vec<u8> = Vec::with_capacity(CAPACITY);
            let info_log_size: usize = 0;

            gl::GetShaderInfoLog(
                self.0,
                CAPACITY as i32,
                info_log_size as *mut i32,
                info_log.as_mut_ptr() as *mut GLchar,
            );

            info_log.set_len(info_log_size);
            String::from_utf8(info_log).unwrap()
        }
    }

    pub fn delete(&self) {
        unsafe {
            gl::DeleteShader(self.0);
        }
    }
}

#[derive(Copy, Clone)]
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
        unsafe {
            const CAPACITY: usize = 512;
            let mut info_log: Vec<u8> = Vec::with_capacity(CAPACITY);
            let info_log_size: usize = 0;

            gl::GetProgramInfoLog(
                self.0,
                CAPACITY as i32,
                info_log_size as *mut i32,
                info_log.as_mut_ptr() as *mut GLchar,
            );

            info_log.set_len(info_log_size);
            String::from_utf8(info_log).unwrap()
        }
    }

    pub fn use_(self: Program) {
        unsafe {
            gl::UseProgram(self.0);
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

pub fn bind_buffer(target: GLenum, buffer: Buffer) {
    unsafe { gl::BindBuffer(target, buffer.0) }
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
            &data[0] as *const T as *const c_void,
            usage,
        );
    }
}

pub fn vertex_attrib_pointer(index: u32, size: i32, type_: GLenum, normalized: bool, stride: usize, offset: usize) {
    let gl_normalized = if normalized { gl::TRUE } else { gl::FALSE };
    unsafe {
        gl::VertexAttribPointer(
            index,
            size,
            type_,
            gl_normalized,
            stride as i32,
            offset as *const c_void,
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
