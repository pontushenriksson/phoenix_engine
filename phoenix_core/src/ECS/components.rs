use gl;
use image::{DynamicImage, GenericImageView, ImageError, open};
use std::path::Path;

pub struct Texture {
  id: gl::types::GLuint,
  r#type: gl::types::GLenum,
  width: u32,
  height: u32,
}

impl Texture {
  // Creates a new texture with an OpenGL ID and type.
  pub fn new(r#type: gl::types::GLenum) -> Texture {
    let mut id: gl::types::GLuint = 0;
    unsafe {
      gl::GenTextures(1, &mut id);
    }
    Texture {
      id,
      r#type,
      width: 0,
      height: 0,
    }
  }

  // Loads an image from file, uploads it as an OpenGL texture, and configures it.
  pub fn load_from_file<P: AsRef<Path>>(
    path: P,
    r#type: gl::types::GLenum,
    format: gl::types::GLenum,
  ) -> Result<Texture, ImageError> {
    let img = open(path)?; // Load image
    let (width, height) = img.dimensions();
    let data = img.flipv().into_rgba8(); // Flip vertically, convert to RGBA

    let texture = Texture::new(r#type);
    texture.bind();
    unsafe {
      gl::TexParameteri(r#type, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
      gl::TexParameteri(r#type, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
      gl::TexParameteri(r#type, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
      gl::TexParameteri(r#type, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
      
      gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1); // Set alignment

      gl::TexImage2D(
        r#type,
        0,
        format as i32,           // Internal format
        width as i32,
        height as i32,
        0,
        gl::RGBA,                // Format of the pixel data
        gl::UNSIGNED_BYTE,
        data.as_ptr() as *const _, // Image data pointer
      );
    }

    Ok(Texture {
      id: texture.id,
      r#type,
      width,
      height,
    })
  } 

  // Generates mipmaps for the texture.
  pub fn into_mipmap(&self, filter: gl::types::GLenum) {
    self.bind();
    unsafe {
      gl::GenerateMipmap(self.r#type);
      gl::TexParameteri(self.r#type, gl::TEXTURE_MIN_FILTER, filter as i32);
      gl::TexParameteri(self.r#type, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    }
  }

  // Binds the texture for use.
  pub fn bind(&self) {
    unsafe {
      gl::BindTexture(self.r#type, self.id);
    }
  }

  pub fn unbind(&self) {
    unsafe {
      gl::BindTexture(self.r#type, 0);
    }
  }

  pub fn set_tex_parameter_f32_vec(/* GL_TEXTURE_2D, GL_TEXTURE_BORDER_COLOR, borderColor */) {

  }
}
