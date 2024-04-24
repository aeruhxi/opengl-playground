use std::{
    collections::{
        hash_map::{self, Entry},
        HashMap,
    },
    fs,
};

use super::material::{create_material, Material};

pub struct Loader {
    materials: HashMap<&'static str, Material>,
}

impl Loader {
    pub fn new() -> Loader {
        Loader {
            materials: HashMap::new(),
        }
    }

    pub fn load_material(
        &mut self,
        name: &'static str,
        vertex_shader_file_path: &str,
        fragment_shader_file_path: &str,
    ) -> &Material {
        let entry = self.materials.entry(name);

        match entry {
            Entry::Occupied(_) => panic!("Material already exists: {}", name),
            Entry::Vacant(entry) => {
                let vertex_source = read_file(vertex_shader_file_path);
                let fragment_source = read_file(fragment_shader_file_path);

                let material = create_material(&vertex_source, &fragment_source);

                entry.insert(material)
            }
        }
    }

    pub fn get_material(&self, name: &str) -> &Material {
        self.materials
            .get(name)
            .expect(&format!("Material not found: {}", name))
    }
}

fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect(&format!("Failed to read file: {}", file_path))
}
