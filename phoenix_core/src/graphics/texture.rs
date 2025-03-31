use image::GenericImageView;

use crate::gl_call;
use crate::debugger::debugger::Debugger;

#[derive(Debug, Clone)]
pub struct Texture {
  id: gl::types::GLuint,
  r#type: gl::types::GLenum,
}

impl Texture {
  pub fn new(
    file_path: &str,
    texture_type: gl::types::GLenum,
    format: gl::types::GLenum,
    pixel_type: gl::types::GLenum,
  ) -> Texture {
    let img = image::open(file_path).expect("Failed to load texture");
    let (width, height) = img.dimensions();
    let img_data = img.to_rgba8(); // Convert to RGBA format

    // Flip the image vertically
    let row_size = (width * 4) as usize; // 4 bytes per pixel (RGBA)
    let mut flipped_data = vec![0u8; (width * height * 4) as usize];
    
    for y in 0..height {
      let src_offset = (y * width * 4) as usize;
      let dst_offset = ((height - 1 - y) * width * 4) as usize; // Flipping vertically

      // Correctly copy the row from img_data to flipped_data
      flipped_data[dst_offset..dst_offset + row_size]
        .copy_from_slice(&img_data.as_raw()[src_offset..src_offset + row_size]);
    }

    // Generate and bind a texture in OpenGL
    let mut texture_id= 0;
    unsafe {
      gl_call!(gl::GenTextures(1, &mut texture_id));
      gl_call!(gl::BindTexture(texture_type, texture_id));

      // Set texture parameters
      gl_call!(gl::TexParameteri(texture_type, gl::TEXTURE_WRAP_S, gl::REPEAT as i32));
      gl_call!(gl::TexParameteri(texture_type, gl::TEXTURE_WRAP_T, gl::REPEAT as i32));
      gl_call!(gl::TexParameteri(texture_type, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32));
      gl_call!(gl::TexParameteri(texture_type, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32));

      // Upload flipped texture data to OpenGL
      gl_call!(gl::TexImage2D(
        texture_type,
          0,
          format as i32,
          width as i32,
          height as i32,
          0,
          format,
          pixel_type,
          flipped_data.as_ptr() as *const std::ffi::c_void,
      ));
      
      gl_call!(gl::GenerateMipmap(texture_type)); // Generate mipmaps
      gl_call!(gl::BindTexture(texture_type, 0));
    }

    Texture {
      id: texture_id,
      r#type: texture_type,
    }
  }

  pub fn bind(&self, unit: gl::types::GLuint) {
    unsafe {
      gl_call!(gl::ActiveTexture(gl::TEXTURE0 + unit));
      gl_call!(gl::BindTexture(self.r#type, self.id));
    }
  }

  pub fn unbind(&self) {
    unsafe {
      gl_call!(gl::BindTexture(self.r#type, 0));
    }
  }
}

#[derive(Debug, Clone)]
pub struct Diffuse;

#[derive(Debug, Clone)]
pub struct Specular;

#[derive(Debug, Clone)]
pub struct Topography;

pub trait Sampler: Send + Sync {
  fn new(
    file_path: &str,
    format: gl::types::GLenum,
    pixel_image: gl::types::GLenum,
  ) -> Box<Self>
  where
    Self: Sized;

  fn bind(&self, unit: gl::types::GLuint);
  fn unbind(&self);
  fn clone_box(&self) -> Box<dyn Sampler>;
}

// Implement Clone for Box<dyn Sampler>
impl Clone for Box<dyn Sampler> {
  fn clone(&self) -> Box<dyn Sampler> {
    self.clone_box()
  }
}

#[derive(Debug, Clone)]
pub struct Sampler2D<T>
  where T: Clone {
  texture: Texture,
  _marker: std::marker::PhantomData<T>,
}

// Explicitly add `where T: 'static + Send + Sync`
impl<T: 'static + Send + Sync> Sampler for Sampler2D<T>
  where T: Clone {
  fn new(
    file_path: &str,
    format: gl::types::GLenum,
    pixel_type: gl::types::GLenum,
  ) -> Box<Self> {
    let texture = Texture::new(file_path, gl::TEXTURE_2D, format, pixel_type);
    Box::new(Sampler2D {
      texture,
      _marker: std::marker::PhantomData,
    })
  }

  fn bind(&self, unit: gl::types::GLuint) {
    self.texture.bind(unit);
  }

  fn unbind(&self) {
    unsafe {
      gl_call!(gl::ActiveTexture(gl::TEXTURE0));
      gl_call!(gl::BindTexture(gl::TEXTURE_2D, 0));
    }
  }

  fn clone_box(&self) -> Box<dyn Sampler> {
    Box::new((*self).clone())
  }
}

#[derive(Clone)]
pub struct Sampler3D<T>
  where T: Clone {
  texture: Texture,
  _marker: std::marker::PhantomData<T>
}

impl<T: 'static + Send + Sync> Sampler for Sampler3D<T>
  where T: Clone {
  fn new(
    file_path: &str,
    format: gl::types::GLenum,
    pixel_type: gl::types::GLenum,
  ) -> Box<Sampler3D<T>> {
    let texture = Texture::new(file_path, gl::TEXTURE_3D, format, pixel_type);
    Box::new(
      Sampler3D {
        texture,
        _marker: std::marker::PhantomData
      }
    )
  }

  fn bind(&self, unit: gl::types::GLuint) {
    self.texture.bind(unit);
  }

  fn unbind(&self) {
    unsafe {
      gl_call!(gl::ActiveTexture(gl::TEXTURE0));
      gl_call!(gl::BindTexture(gl::TEXTURE_3D, 0));
    }
  }

  fn clone_box(&self) -> Box<dyn Sampler> {
    Box::new((*self).clone())
  }
}
