use crate::gl_call;
use crate::debugger::debugger::Debugger;

#[derive(Clone)]
pub struct UniformBufferObject {
  id: gl::types::GLuint,
  size: gl::types::GLintptr,
}

impl UniformBufferObject {
  pub fn new(size: gl::types::GLintptr) -> UniformBufferObject {
    let mut id = 0;
    unsafe {
      gl_call!(gl::CreateBuffers(1, &mut id));
      gl_call!(gl::NamedBufferData(id, size, std::ptr::null(), gl::STATIC_DRAW));
    }

    UniformBufferObject { id, size }
  }

  pub fn set_data<T>(&self, offset: gl::types::GLintptr, data: &[T]) {
    assert!(offset >= 0, "Offset must be non-negative!");
    let data_size = (data.len() * std::mem::size_of::<T>()) as gl::types::GLintptr;
    assert!(
      offset + data_size <= self.size,
      "Data exceeds the buffer size!"
    );

    unsafe {
      gl_call!(gl::NamedBufferSubData(
        self.id,
        offset,
        data_size,
        data.as_ptr() as *const std::ffi::c_void,
      ));
    }
  }

  pub fn bind_to(&self, binding_point: gl::types::GLuint) {
    unsafe {
      gl_call!(gl::BindBufferBase(gl::UNIFORM_BUFFER, binding_point, self.id));
    }
  }

  pub fn unbind_from(binding_point: gl::types::GLuint) {
    unsafe {
      gl_call!(gl::BindBufferBase(gl::UNIFORM_BUFFER, binding_point, 0));
    }
  }

  pub fn delete(self) {
    unsafe {
      gl_call!(gl::DeleteBuffers(1, &self.id));
    }
  }
}

pub struct InstanceBufferObject {
  id: gl::types::GLuint,
  size: gl::types::GLintptr,
}

impl InstanceBufferObject {
  pub fn new(size: gl::types::GLintptr) -> InstanceBufferObject {
    let mut id = 0;
    unsafe {
      gl_call!(gl::CreateBuffers(1, &mut id));
      gl_call!(gl::NamedBufferData(id, size, std::ptr::null(), gl::DYNAMIC_DRAW));
    }

    InstanceBufferObject { id, size }
  }

  pub fn set_data<T>(&self, offset: gl::types::GLintptr, data: &[T]) {
    assert!(offset >= 0, "Offset must be non-negative!");
    let data_size = (data.len() * std::mem::size_of::<T>()) as gl::types::GLintptr;
    assert!(
      offset + data_size <= self.size,
      "Data exceeds the buffer size!"
    );

    unsafe {
      gl_call!(gl::NamedBufferSubData(
        self.id,
        offset,
        data_size,
        data.as_ptr() as *const std::ffi::c_void,
      ));
    }
  }

  pub fn bind(&self) {
    unsafe {
      gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, self.id));
    }
  }

  pub fn unbind() {
    unsafe {
      gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, 0));
    }
  }

  pub fn delete(self) {
    unsafe {
      gl_call!(gl::DeleteBuffers(1, &self.id));
    }
  }
}
