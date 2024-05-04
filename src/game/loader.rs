use std::{
    collections::{hash_map::Entry, HashMap},
    path::Path,
};

use stb_image::image::{self, LoadResult};

use crate::util::read_file;

use super::{material::Material, texture2d::Texture2D};

pub struct Loader {
    materials: HashMap<&'static str, Material>,
    textures: HashMap<&'static str, Texture2D>,
}

impl Loader {
    pub fn new() -> Loader {
        Loader {
            materials: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn load_material(
        &mut self,
        name: &'static str,
        vertex_shader_file_path: &str,
        fragment_shader_file_path: &str,
    ) -> &Material {
        match self.materials.entry(name) {
            Entry::Occupied(_) => panic!("Material already exists: {}", name),
            Entry::Vacant(entry) => {
                let vertex_source = read_file(Path::new(vertex_shader_file_path));
                let fragment_source = read_file(Path::new(fragment_shader_file_path));

                let material = Material::new(&vertex_source, &fragment_source);

                entry.insert(material)
            }
        }
    }

    pub fn get_material(&self, name: &str) -> &Material {
        self.materials
            .get(name)
            .unwrap_or_else(|| panic!("Material not found: {}", name))
    }

    pub fn load_texture(&mut self, name: &'static str, image_file_path: &Path, alpha: bool) -> &Texture2D {
        let format = if alpha { gl::RGBA } else { gl::RGB };

        let texture_2d = Texture2D::new(format, format, gl::REPEAT, gl::REPEAT, gl::LINEAR, gl::LINEAR);

        if let LoadResult::ImageU8(image) = image::load(image_file_path) {
            texture_2d.generate(image.width as i32, image.height as i32, image.data);

            let entry = self.textures.entry(name);
            match entry {
                Entry::Occupied(_) => panic!("Texture already exists: {}", name),
                Entry::Vacant(entry) => entry.insert(texture_2d),
            }
        } else {
            panic!("Error reading image file: {}", image_file_path.display())
        }
    }

    pub fn get_texture(&self, name: &str) -> &Texture2D {
        self.textures
            .get(name)
            .unwrap_or_else(|| panic!("Texture not found: {}", name))
    }
}
