use image::GenericImageView;

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
      gl::GenTextures(1, &mut texture_id);
      gl::BindTexture(texture_type, texture_id);

      // Set texture parameters
      gl::TexParameteri(texture_type, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
      gl::TexParameteri(texture_type, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
      gl::TexParameteri(texture_type, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
      gl::TexParameteri(texture_type, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

      // Upload flipped texture data to OpenGL
      gl::TexImage2D(
        texture_type,
          0,
          format as i32,
          width as i32,
          height as i32,
          0,
          format,
          pixel_type,
          flipped_data.as_ptr() as *const std::ffi::c_void,
      );
      
      gl::GenerateMipmap(texture_type); // Generate mipmaps
    }

    Texture {
      id: texture_id,
      r#type: texture_type,
    }
  }

  pub fn bind(&self, unit: gl::types::GLuint) {
    unsafe {
        gl::ActiveTexture(gl::TEXTURE0 + unit);
        gl::BindTexture(gl::TEXTURE_2D, self.id);
    }
  }
}

#[derive(Debug, Clone)]
pub struct Diffuse;

#[derive(Debug, Clone)]
pub struct Specular;

pub trait Sampler {
  fn new(
    file_path: &str,
    unit: gl::types::GLuint,
    format: gl::types::GLenum,
    pixel_image: gl::types::GLenum
  ) -> Self where Self: Sized;

  fn bind(&self);

  fn unbind(&self);
}

#[derive(Debug, Clone)]
pub struct Sampler2D<T> {
  texture: Texture,
  unit: gl::types::GLuint,
  _marker: std::marker::PhantomData<T>,
}

impl<T> Sampler for Sampler2D<T> {
  fn new(
    file_path: &str,
    unit: gl::types::GLuint,
    format: gl::types::GLenum,
    pixel_type: gl::types::GLenum,
  ) -> Sampler2D<T> {
    let texture = Texture::new(file_path, gl::TEXTURE_2D, format, pixel_type);
    Sampler2D {
      texture,
      unit,
      _marker: std::marker::PhantomData
    }
  }

  fn bind(&self) {
    self.texture.bind(self.unit);
  }

  fn unbind(&self) {
    unsafe {
      gl::ActiveTexture(gl::TEXTURE0);
      gl::BindTexture(gl::TEXTURE_2D, 0);
    }
  }
}

pub struct Sampler3D<T> {
  texture: Texture,
  unit: gl::types::GLuint,
  _marker: std::marker::PhantomData<T>
}

impl<T> Sampler for Sampler3D<T> {
  fn new(
    file_path: &str,
    unit: gl::types::GLuint,
    format: gl::types::GLenum,
    pixel_type: gl::types::GLenum,
  ) -> Sampler3D<T> {
    let texture = Texture::new(file_path, gl::TEXTURE_3D, format, pixel_type);
    Sampler3D {
      texture,
      unit,
      _marker: std::marker::PhantomData
    }
  }

  fn bind(&self) {
    self.texture.bind(self.unit);
  }

  fn unbind(&self) {
    unsafe {
      gl::ActiveTexture(gl::TEXTURE0);
      gl::BindTexture(gl::TEXTURE_3D, 0);
    }
  }
}
