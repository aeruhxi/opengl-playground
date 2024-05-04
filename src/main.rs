extern crate gl;
extern crate glfw;

mod game;
mod opengl;
mod util;

use std::{cell::RefCell, rc::Rc};

use game::{loader::Loader, Game};
use glfw::{Action, Context, Key, Window};
use opengl::{clear, clear_color};

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

    let mut loader = Loader::new();
    let game: Rc<RefCell<Game>> = Rc::new(RefCell::new(Game::new(800, 600, &mut loader)));

    {
        let game = Rc::clone(&game);
        window.set_key_callback(move |window, key, _scan_code, action, _mods| handle_key(&game, window, key, action));
    }

    let mut delta_time: f32;
    let mut last_frame: f32 = 0.0;

    while !window.should_close() {
        let current_frame = glfw.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;
        glfw.poll_events();

        game.borrow_mut().process_input(delta_time);

        game.borrow_mut().update(delta_time);

        clear_color(0.0, 0.0, 0.0, 1.0);
        clear(gl::COLOR_BUFFER_BIT);
        game.borrow().render(&mut loader);

        window.swap_buffers();
    }
}

fn handle_key(game: &RefCell<Game>, window: &mut Window, key: Key, action: Action) {
    if key == Key::Escape && action == Action::Press {
        return window.set_should_close(true);
    }

    let key = key as usize;
    if key <= 1024 {
        if action == Action::Press {
            game.borrow_mut().set_bool(key, true);
        } else if action == Action::Release {
            game.borrow_mut().set_bool(key, false);
        }
    }
}
