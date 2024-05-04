use std::path::Path;

use glam::{vec2, vec3a, Mat4};

use self::{loader::Loader, sprite_renderer::SpriteRenderer};

pub mod loader;
pub mod material;
pub mod sprite_renderer;
pub mod texture2d;

pub enum GameState {
    ACTIVE,
    MENU,
    WIN,
}

pub struct Game {
    state: GameState,
    bool: [bool; 1024],
    width: u32,
    height: u32,
    sprite_renderer: SpriteRenderer,
}

pub struct Wee {
    hello: String,
}

fn some() -> Wee {
    let mut str = String::new();
    str.push_str("hel");

    Wee { hello: str }
}

impl Game {
    pub fn new<'a>(width: u32, height: u32, loader: &'a mut Loader) -> Game {
        let material = loader.load_material("sprite", "src/shaders/sprite.vert", "src/shaders/sprite.frag");

        let projection = Mat4::orthographic_lh(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);

        material.use_();
        // material.set_integer("image", 0);
        material.set_matrix4f("projection", projection);

        let sprite_renderer = SpriteRenderer::new(material.clone());

        // load texture
        loader.load_texture("face", Path::new("src/resources/awesomeface.png"), true);

        Game {
            state: GameState::ACTIVE,
            bool: [false; 1024],
            width,
            height,
            sprite_renderer,
        }
    }

    pub fn update(&self, dt: f32) {}

    pub fn process_input(&self, dt: f32) {}

    pub fn render(&self, loader: &mut Loader) {
        self.sprite_renderer.draw(
            loader.get_texture("face"),
            vec2(200., 200.),
            vec2(100., 100.),
            45.,
            vec3a(0., 1., 0.),
        )
    }

    pub fn set_bool(&mut self, key: usize, value: bool) {
        self.bool[key] = value;
    }
}
