enum MeshType {
  Mesh,
  Line,
}

struct Mesh {
  vertices: Vec<f32>,
  indices: Option<Vec<u32>>,
  mesh_type: MeshType,
  // normals
}

struct GameObject {
  /*
  position: Vector3,
  rotation: Quaternion,
  mesh: Mesh,
  */
}

impl GameObject {
  pub fn update(&mut self) {
    // Example of a binding, althought not a bind yet
  }
}

struct Scene {
  objects: Vec<GameObject>,
  // camera: Camera, Can be orthographic or perspective
  // lighting data
}

impl Scene {
  pub fn new() -> Scene {
    Scene {
      objects: Vec::new(),
      // camera: Camera::orthographic() or ::perspective(),
      // lighting data
    }
  }

  pub fn update(&mut self) {
    // Update all objects in the scene
    for object in &mut self.objects {
      object.update();
    }
  }
}
