use glam::{vec2, Vec2, Vec3A};

use super::{sprite_renderer::SpriteRenderer, texture2d::Texture2D};

#[derive(Debug)]
pub struct Entity {
    position: Vec2,
    size: Vec2,
    velocity: Vec2,
    color: Vec3A,
    rotation: f32,
    sprite: Texture2D,
    is_solid: bool,
    is_destroyed: bool,
}

impl Entity {
    pub fn new(position: Vec2, size: Vec2, sprite: Texture2D, color: Vec3A) -> Entity {
        Entity {
            position,
            size,
            velocity: vec2(0.0, 0.0),
            color,
            rotation: 0.0,
            sprite,
            is_solid: false,
            is_destroyed: false,
        }
    }

    pub fn draw(&self, renderer: &SpriteRenderer) {
        renderer.draw(&self.sprite, self.position, self.size, self.rotation, self.color);
    }

    pub fn is_destroyed(&self) -> bool {
        self.is_destroyed
    }

    pub fn is_solid(&self) -> bool {
        self.is_solid
    }
}
