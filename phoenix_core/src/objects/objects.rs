use futures::Stream;

use crate::graphics::mesh::{StaticMesh, DynamicMesh, StreamMesh};
use crate::objects::transform::Transform;

// #[derive(Clone)]
pub struct StaticObject {
  pub mesh: StaticMesh,
  pub transform: Transform,
}

impl StaticObject {
  pub fn new(
    mesh: StaticMesh,
    transform: Transform,
  ) -> StaticObject {
    StaticObject {
      mesh,
      transform
    }
  }
}

pub struct DynamicObject {
  pub mesh: DynamicMesh,
  pub transform: Transform,
}

impl DynamicObject {
  pub fn new(
    mesh: DynamicMesh,
    transform: Transform,
  ) -> DynamicObject {
    DynamicObject {
      mesh,
      transform
    }
  }
}

pub struct StreamObject {
  pub mesh: StreamMesh,
  pub transform: Transform,
}

impl StreamObject {
  pub fn new(
    mesh: StreamMesh,
    transform: Transform,
  ) -> StreamObject {
    StreamObject {
      mesh,
      transform
    }
  }
}
