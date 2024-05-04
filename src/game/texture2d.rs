use crate::opengl::{gen_textures, tex_image_2d, tex_parameter_i, Texture};
use gl::types::{GLboolean, GLenum, GLint};

#[derive(Clone, Debug)]
pub struct Texture2D {
    texture: Texture,
    internal_format: GLenum,
    image_format: GLenum,
    wrap_s: GLenum,
    wrap_t: GLenum,
    filter_min: GLenum,
    filter_mag: GLenum,
}

impl Texture2D {
    pub fn new(
        internal_format: GLenum,
        image_format: GLenum,
        wrap_s: GLenum,
        wrap_t: GLenum,
        filter_min: GLenum,
        filter_mag: GLenum,
    ) -> Self {
        let [texture] = gen_textures::<1>();
        Self {
            texture,
            internal_format,
            image_format,
            wrap_s,
            wrap_t,
            filter_min,
            filter_mag,
        }
    }

    pub fn generate(&self, width: i32, height: i32, data: Vec<u8>) {
        // create texture
        self.texture.bind(gl::TEXTURE_2D);
        tex_image_2d(
            gl::TEXTURE_2D,
            0,
            self.internal_format,
            width,
            height,
            0,
            self.image_format,
            gl::UNSIGNED_BYTE,
            &data,
        );

        // set texture wrap and filter modes
        tex_parameter_i(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, self.wrap_s);
        tex_parameter_i(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, self.wrap_t);
        tex_parameter_i(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, self.filter_min);
        tex_parameter_i(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, self.filter_mag);

        // unbind textures
        Texture::unbind(gl::TEXTURE_2D);
    }

    pub fn bind(&self) {
        self.texture.bind(gl::TEXTURE_2D);
    }
}
