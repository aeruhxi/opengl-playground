use std::path::Path;

use glam::{vec2, vec3a, Vec3A};

use crate::util::read_file;

use super::{entity::Entity, loader::Loader, sprite_renderer::SpriteRenderer};

#[derive(Debug)]
pub struct Level {
    bricks: Vec<Entity>,
}

impl Level {
    pub fn new() -> Level {
        Level { bricks: Vec::new() }
    }

    pub fn load(&mut self, file_path: &Path, level_width: u32, level_height: u32, loader: &Loader) {
        self.bricks.clear();

        let mut tile_data: Vec<Vec<u32>> = Vec::with_capacity(5);
        for line in read_file(file_path).lines() {
            let row: Vec<u32> = line
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| c.to_digit(10).or_else(|| panic!("Invalid tile data: {}", c)).unwrap())
                .collect();
            tile_data.push(row);
        }

        if tile_data.is_empty() {
            panic!("Invalid level file");
        }

        // initialize bricks
        let row_no = tile_data.len() as u32;
        let col_no = tile_data[0].len() as u32;
        let unit_height = level_height / row_no;
        let unit_width = level_width / col_no;

        for (y, row) in tile_data.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                if tile == 0 {
                    continue;
                };

                let color = get_color_from_tile_no(tile);
                let texture = loader.get_texture(get_texture_from_tile_no(tile));
                let pos = vec2(x as f32 * unit_width as f32, y as f32 * unit_height as f32);
                let size = vec2(unit_width as f32, unit_height as f32);
                let tile_entity = Entity::new(pos, size, texture.clone(), color);
                self.bricks.push(tile_entity);
            }
        }
    }

    pub fn draw(&self, renderer: &SpriteRenderer) {
        for tile in &self.bricks {
            if tile.is_destroyed() {
                continue;
            };
            tile.draw(renderer);
        }
    }

    pub fn is_completed(&self) -> bool {
        self.bricks
            .iter()
            .all(|tile| if tile.is_solid() { true } else { tile.is_destroyed() })
    }
}

fn get_color_from_tile_no(tile_no: u32) -> Vec3A {
    match tile_no {
        1 => vec3a(0.8, 0.8, 0.8),
        2 => vec3a(0.2, 0.6, 1.0),
        3 => vec3a(0.0, 0.7, 0.0),
        4 => vec3a(0.8, 0.8, 0.4),
        5 => vec3a(1.0, 0.5, 0.0),
        _ => vec3a(1., 1., 1.),
    }
}

fn get_texture_from_tile_no(title_no: u32) -> &'static str {
    match title_no {
        1 => "block_solid",
        _ => "block",
    }
}
