use std::{cmp, path::Path};

use glam::{vec2, vec3a, Mat4, Vec2};
use glfw::ffi::{KEY_A, KEY_D};

use self::{entity::Entity, level::Level, loader::Loader, sprite_renderer::SpriteRenderer};

mod entity;
mod level;
pub mod loader;
mod material;
mod sprite_renderer;
mod texture2d;

pub enum GameState {
    Active,
    Menu,
    Win,
}

pub struct Game {
    state: GameState,
    keys: [bool; 1024],
    width: u32,
    height: u32,
    sprite_renderer: SpriteRenderer,
    levels: Vec<Level>,
    current_level: usize,
    player: Entity,
}

pub struct Wee {
    hello: String,
}

fn some() -> Wee {
    let mut str = String::new();
    str.push_str("hel");

    Wee { hello: str }
}

macro_rules! load_textures {
    ($loader:ident, $(($name:expr, $file_name:expr, $alpha:expr)),+ $(,)?) => {{
        $(
            $loader.load_texture($name, Path::new(concat!("resources/textures/", $file_name)), $alpha);
        )+
    }};
}

const PLAYER_SIZE: Vec2 = vec2(100.0, 20.0);

impl Game {
    pub fn new(width: u32, height: u32, loader: &mut Loader) -> Game {
        // load shaders
        let material = loader.load_material("sprite", "src/shaders/sprite.vert", "src/shaders/sprite.frag");

        // configure shaders
        let projection = Mat4::orthographic_lh(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);
        material.use_();
        material.set_integer("image", 0);
        material.set_matrix4f("projection", projection);

        // renderer
        let sprite_renderer = SpriteRenderer::new(material.clone());

        // load textures
        load_textures!(
            loader,
            ("background", "background.jpg", false),
            ("face", "awesomeface.png", true),
            ("block", "block.png", false),
            ("block_solid", "block_solid.png", false),
            ("paddle", "paddle.png", true),
        );

        // load levels
        let levels = load_levels(loader, width, height, 1);

        // player
        let player_pos = vec2(width as f32 / 2.0 - PLAYER_SIZE.x / 2.0, height as f32 - PLAYER_SIZE.y);
        let player = Entity::new(
            player_pos,
            PLAYER_SIZE,
            loader.get_texture("paddle").clone(),
            vec3a(1.0, 1.0, 1.0),
        );

        Game {
            state: GameState::Active,
            keys: [false; 1024],
            width,
            height,
            levels,
            current_level: 0,
            sprite_renderer,
            player,
        }
    }

    pub fn update(&self, dt: f32) {}

    pub fn process_input(&mut self, dt: f32) {
        const PLAYER_VELOCITY: f32 = 500.0;
        match self.state {
            GameState::Active => {
                let velocity = PLAYER_VELOCITY * dt;

                if self.keys[KEY_A as usize] {
                    self.player.position.x = (self.player.position.x - velocity).max(0.0);
                } else if self.keys[KEY_D as usize] {
                    self.player.position.x = (self.player.position.x + velocity).min(self.width as f32 - PLAYER_SIZE.x);
                }
            }
            _ => todo!(),
        }
    }

    pub fn render(&self, loader: &mut Loader) {
        match self.state {
            GameState::Active => {
                self.sprite_renderer.draw(
                    loader.get_texture("background"),
                    vec2(0., 0.),
                    vec2(self.width as f32, self.height as f32),
                    0.0,
                    vec3a(1., 1., 1.),
                );
                self.levels[self.current_level].draw(&self.sprite_renderer);
                self.player.draw(&self.sprite_renderer);
            }
            _ => todo!(),
        }
    }

    pub fn set_bool(&mut self, key: usize, value: bool) {
        self.keys[key] = value;
    }
}

fn load_levels(loader: &Loader, width: u32, height: u32, number: usize) -> Vec<Level> {
    let mut levels = Vec::with_capacity(number);
    for i in 1..=number {
        let mut level = Level::new();
        level.load(
            Path::new(&format!("resources/levels/level_{}.lvl", i)),
            width,
            height / 2,
            loader,
        );
        levels.push(level);
    }
    levels
}
