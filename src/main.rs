extern crate gl;
extern crate glfw;
use std::mem::size_of;

mod opengl;

use glfw::Context;
use ogl::{
    bind_buffer, buffer_data, clear, clear_color, create_program, create_shader, draw_arrays,
    enable_vertex_attrib_array, gen_buffers, gen_vertex_arrays, vertex_attrib_pointer,
};
use opengl as ogl;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

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
    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0, 
        0.5, -0.5, 0.0, 
        0.0, 0.5, 0.0
    ];

    // vertex shader
    let vertex_shader = create_shader(gl::VERTEX_SHADER);
    vertex_shader.source(VERTEX_SHADER_SOURCE);
    vertex_shader.compile();

    let success = vertex_shader.get(gl::COMPILE_STATUS);
    if !success {
        let info_log = vertex_shader.get_info_log();
        println!("Error during compilation\n{}", info_log);
    }

    // fragment shader
    let fragment_shader = create_shader(gl::FRAGMENT_SHADER);
    fragment_shader.source(FRAGMENT_SHADER_SOURCE);
    fragment_shader.compile();

    let shader_program = create_program();
    shader_program.attach_shader(vertex_shader);
    shader_program.attach_shader(fragment_shader);
    shader_program.link();

    let success = shader_program.get(gl::LINK_STATUS);
    if !success {
        let info_log = shader_program.get_info_log();
        println!("Error during linking\n{}", info_log);
    }

    // clean up
    vertex_shader.delete();
    fragment_shader.delete();

    // buffers
    let [vbo] = gen_buffers::<1>();
    let [vao] = gen_vertex_arrays::<1>();

    vao.bind();
    bind_buffer(gl::ARRAY_BUFFER, vbo);

    buffer_data(gl::ARRAY_BUFFER, &vertices, gl::STATIC_DRAW);

    vertex_attrib_pointer(0, 3, gl::FLOAT, false, 3 * size_of::<f32>(), 0);
    enable_vertex_attrib_array(0);

    while !window.should_close() {
        clear_color(0.2, 0.3, 0.3, 1.0);
        clear(gl::COLOR_BUFFER_BIT);

        shader_program.use_();
        vao.bind();

        draw_arrays(gl::TRIANGLES, 0, 3);

        window.swap_buffers();
        glfw.poll_events();
    }
}

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
       FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;
