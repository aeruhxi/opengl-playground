extern crate gl;
extern crate glfw;
use std::mem::size_of;

mod opengl;
mod renderer;

use glfw::Context;
use opengl::{
    bind_buffer, buffer_data, clear, clear_color, draw_arrays, enable_vertex_attrib_array, gen_buffers,
    gen_vertex_arrays, vertex_attrib_pointer,
};
use renderer::loader::Loader;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    #[cfg(target_os = "macos")]
    {
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    }

    let (mut window, _events) = glfw
        .create_window(800, 600, "Hello, world!", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // Load OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    window.set_framebuffer_size_callback(|_window, width, height| {
        unsafe { gl::Viewport(0, 0, width, height) };
    });

    #[rustfmt::skip]
    let vertices: [f32; 18] = [
         0.5, -0.5, 0.0,        1.0, 0.0, 0.0,
        -0.5, -0.5, 0.0,        0.0, 1.0, 0.0,
         0.0,  0.5, 0.0,        0.0, 0.0, 1.0,
    ];

    // Program
    let mut loader = Loader::new();
    let material = loader.load_material("triangle", "src/shaders/vert.glsl", "src/shaders/frag.glsl");

    // buffers
    let [vbo] = gen_buffers::<1>();
    let [vao] = gen_vertex_arrays::<1>();

    vao.bind();
    bind_buffer(gl::ARRAY_BUFFER, vbo);
    buffer_data(gl::ARRAY_BUFFER, &vertices, gl::STATIC_DRAW);

    vertex_attrib_pointer(0, 3, gl::FLOAT, false, 6 * size_of::<f32>(), 0);
    enable_vertex_attrib_array(0);

    vertex_attrib_pointer(1, 3, gl::FLOAT, false, 6 * size_of::<f32>(), 3 * size_of::<f32>());
    enable_vertex_attrib_array(1);

    while !window.should_close() {
        clear_color(0.2, 0.3, 0.3, 1.0);
        clear(gl::COLOR_BUFFER_BIT);

        material.use_();
        vao.bind();
        draw_arrays(gl::TRIANGLES, 0, 3);

        window.swap_buffers();
        glfw.poll_events();
    }
}
