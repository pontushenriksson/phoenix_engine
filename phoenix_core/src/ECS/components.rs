use gl;
use image::GenericImageView;

pub struct Texture2D {
  id: gl::types::GLuint,
}

/// https://learnopengl.com/Getting-started/Textures
impl Texture2D {
  pub fn new(file_path: &str) -> Texture2D {
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
      let mut texture_id: gl::types::GLuint = 0;
      unsafe {
          gl::GenTextures(1, &mut texture_id);
          gl::BindTexture(gl::TEXTURE_2D, texture_id);

          // Set texture parameters
          gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
          gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
          gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
          gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

          // Upload flipped texture data to OpenGL
          gl::TexImage2D(
              gl::TEXTURE_2D,
              0,
              gl::RGBA as i32,
              width as i32,
              height as i32,
              0,
              gl::RGBA,
              gl::UNSIGNED_BYTE,
              flipped_data.as_ptr() as *const std::ffi::c_void,
          );
      }

      Texture2D { id: texture_id }
  }

  pub fn bind(&self) {
    unsafe {
      gl::BindTexture(gl::TEXTURE_2D, self.id);
    }
  }

  pub fn unbind(&self) {
    unsafe {
      gl::BindTexture(gl::TEXTURE_2D, 0);
    }
  }

  pub fn to_mipmap(&self) {
    unsafe {
      gl::BindTexture(gl::TEXTURE_2D, self.id);
      gl::GenerateMipmap(gl::TEXTURE_2D); // Generate mipmaps
    }
  }
}

/*

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
      // data: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>
    }
  }

  // Loads an image from file, uploads it as an OpenGL texture, and configures it.
  pub fn load_from_file<P: AsRef<Path>>(
    path: P,
    r#type: gl::types::GLenum,
    format: gl::types::GLenum,
  ) -> Result<Texture, ImageError> {
    let img: DynamicImage = open(path)?; // Load image
    let (width, height) = img.dimensions();
    let data: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = img.flipv().into_rgba8(); // Flip vertically, convert to RGBA

    let mut texture = Texture::new(r#type); // Create texture
    texture.width = width;
    texture.height = height;

    texture.bind();
    unsafe {
      gl::TexParameteri(r#type, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
      gl::TexParameteri(r#type, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
      gl::TexParameteri(r#type, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
      gl::TexParameteri(r#type, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

      // Set alignment for 4-byte RGBA textures

      gl::PixelStorei(gl::UNPACK_ALIGNMENT, 4);

      gl::TexImage2D(
        r#type,
        0,
        format as i32,          // Internal format
        width as i32,
        height as i32,
        0,
        format as u32,               // Format of the pixel data (assuming RGBA)
        gl::UNSIGNED_BYTE,
        data.as_ptr() as *const _, // Image data pointer
      );
    }
    
    texture.unbind(); // Unbind the texture

    Ok(texture)
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

*/

// Later:

/*

pub struct GameObject {
  id: u32,
  components: HashMap<TypeId, Box<dyn Any>>, // store components dynamically
}

*/
