use phoenix_core::{
    PhoenixApplication,
    graphics::*,
    objects::*,
};

use cgmath::One;

#[tokio::main]
async fn main() {
    let mut app = PhoenixApplication::new(
        1280,
        720,
        "Minecraft 2",
        "Z:/dev/phoenix/phoenix_engine/projects/minecraft_2/assets/icons/icon.png",
    ).unwrap();

    let vertices: [gl::types::GLfloat; 60] = [
    // Coordinates            Normals                        Colors                            Texture Coordinates (if higher than 1.0, texture repeats)
        -0.5, 0.0,  0.5,       -0.577, -0.577,  0.577,         0.83, 0.70, 0.44, 1.0,            0.0, 0.0,
        -0.5, 0.0, -0.5,       -0.577, -0.577, -0.577,         0.83, 0.70, 0.44, 1.0,            1.0, 0.0,
        0.5, 0.0, -0.5,        0.577, -0.577, -0.577,         0.83, 0.70, 0.44, 1.0,            0.0, 0.0,
        0.5, 0.0,  0.5,        0.577, -0.577,  0.577,         0.83, 0.70, 0.44, 1.0,            1.0, 0.0,
        0.0, 0.75, 0.0,        0.000,  0.894,  0.000,         0.92, 0.86, 0.76, 1.0,            0.5, 1.0,
    ];

    let indices: [gl::types::GLuint; 18] = [
      0, 1, 2,
      0, 2, 3,
      0, 1, 4,
      1, 2, 4,
      2, 3, 4,
      3, 0, 4,
    ];

    let descriptor = VertexDescriptor {
        attributes: vec![
            Attribute::Vec3,
            Attribute::Vec3,
            Attribute::Vec4,
            Attribute::Vec2,
        ],
        stride: 3 + 3 + 4 + 2,
    };
  
    let mesh = Mesh::new(
        BufferType::Static,
        vertices.to_vec(),
        Some(indices.to_vec()),
        descriptor,
    );

    let shader = ShaderProgram::new(
        "Z:/dev/phoenix/phoenix_engine/projects/minecraft_2/assets/materials/planks/shaders/planks.vert",
        "Z:/dev/phoenix/phoenix_engine/projects/minecraft_2/assets/materials/planks/shaders/planks.frag",
    );

    let diffuse = Sampler2D::<Diffuse>::new(
        "Z:/dev/phoenix/phoenix_engine/projects/minecraft_2/assets/materials/planks/textures/diffuse.png",
        gl::RGBA,
        gl::UNSIGNED_BYTE
    );
    
    let specular = Sampler2D::<Specular>::new(
        "Z:/dev/phoenix/phoenix_engine/projects/minecraft_2/assets/materials/planks/textures/specular.png",
        gl::RGBA,
        gl::UNSIGNED_BYTE
    );

    let u_data: [f32; 3] = [
        0.42,
        1.2,
        0.3,
    ];

    let ubo = UniformBufferObject::new((u_data.len() * std::mem::size_of::<f32>()) as isize);
    // ubo.set_data(0, &u_data);

    let mut material = Material::new(shader, app.info.texture_unit_count() as usize, Some(ubo));
    material.add_sampler(diffuse);
    material.add_sampler(specular);

    // material.set_ubo_data();

    let game_object = GameObject::new(mesh, material).with_transform(Transform::identity());

    let light = PointLight::new(
        Transform::identity(),
        cgmath::vec3(1.0 ,1.0 , 1.0),
        0.95,
        (0.42, 1.2, 0.3)
    );

    let ground_shader = ShaderProgram::new(
        "Z:/dev/phoenix/phoenix_engine/projects/minecraft_2/shaders/ground.vert",
        "Z:/dev/phoenix/phoenix_engine/projects/minecraft_2/shaders/ground.frag",
    );

    let ground_height_map = Sampler2D::<Topography>::new(
        "Z:/dev/phoenix/phoenix_engine/projects/minecraft_2/assets/textures/perlin noise.png",
        gl::RGBA,
        gl::UNSIGNED_BYTE
    );

    let ground_texture = Sampler2D::<Diffuse>::new(
        "Z:/dev/phoenix/phoenix_engine/projects/minecraft_2/assets/textures/bricks texture.jpg",
        gl::RGBA,
        gl::UNSIGNED_BYTE
    );

    let u_height_scale: [f32; 1] = [0.05];

    let ground_ubo = UniformBufferObject::new((u_height_scale.len() * std::mem::size_of::<f32>()) as isize);
    
    let mut ground_material = Material::new(ground_shader, app.info.texture_unit_count() as usize, Some(ground_ubo));
    ground_material.add_sampler(ground_height_map);
    ground_material.add_sampler(ground_texture);
    ground_material.shader.create_uniform("uHeightScale");

    let ground = Ground::new(64, 64, ground_material).with_transform(
        Transform {
            translation: cgmath::vec3(0.0, 0.2, 0.0),
            rotation: cgmath::Quaternion::one(),
            scale: cgmath::vec3(1.0, 1.0, 1.0),
        }
    );

    let camera = Camera::new(
        // 1280,
        // 720,
        1920,
        1080,
        (1280.0 / 720.0) as f32,
        cgmath::point3(0.0, 0.0, 3.0),
        45.0,
        0.1,
        100.0,
        0.4,
        100.0,
    );

    app.add_game_object(game_object);
    app.add_pointlight(light);
    app.add_ground(ground);
    app.add_camera(camera);

    app.run();
}
