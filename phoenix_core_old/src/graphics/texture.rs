use image::GenericImageView;

#[derive(Debug)]
pub enum TextureType {
  Diffuse,
  Specular,
}

#[derive(Debug)]
pub struct Texture {
  id: gl::types::GLuint,
  r#type: TextureType,
  // target: gl::types::GLenum,
}

impl Texture {
  pub fn new(file_path: &str, texture_type: TextureType) -> Texture {
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

    Texture { id: texture_id, r#type: texture_type }
  }

  pub fn new_from_gltf(data: &[u8], texture_type: TextureType) -> Texture {
    // Decode image data using the `image` crate
    let img = image::load_from_memory(data)
      .expect("Failed to decode texture data from glTF");
    let (width, height) = img.dimensions();
    let img_data = img.to_rgba8(); // Convert to RGBA format

    // Flip the image vertically for OpenGL
    let row_size = (width * 4) as usize;
    let mut flipped_data = vec![0u8; (width * height * 4) as usize];
    for y in 0..height {
      let src_offset = (y * width * 4) as usize;
      let dst_offset = ((height - 1 - y) * width * 4) as usize;
      flipped_data[dst_offset..dst_offset + row_size]
        .copy_from_slice(&img_data.as_raw()[src_offset..src_offset + row_size]);
    }

    // Create OpenGL texture
    let mut texture_id: gl::types::GLuint = 0;
    unsafe {
      gl::GenTextures(1, &mut texture_id);
      gl::BindTexture(gl::TEXTURE_2D, texture_id);

      // Set texture parameters
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

      // Upload texture data to OpenGL
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

      // Generate mipmaps
      gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    Texture {
      id: texture_id,
      r#type: texture_type,
    }
  }

  pub fn activate(&self, texture_unit: u32) {
    unsafe {
      gl::ActiveTexture(gl::TEXTURE0 + texture_unit);
      gl::BindTexture(gl::TEXTURE_2D, self.id);
    }
  }

  pub fn deactivate(&self) {
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

impl Drop for Texture {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteTextures(1, &self.id);
    }
  }
}
