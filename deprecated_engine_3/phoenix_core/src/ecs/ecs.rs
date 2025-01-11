use std::{collections::HashMap, hash::Hash};
use crate::mesh::mesh::*;

/// Component
pub trait Component: 'static {}

// Trait for dynamic component stores
pub trait AnyComponentStore: std::any::Any {
  fn as_any(&self) -> &dyn std::any::Any;
  fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: Component> AnyComponentStore for ComponentStore<T> {
  fn as_any(&self) -> &dyn std::any::Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
    self
  }
}

/// Storage for a specific component type
pub struct ComponentStore<T: Component> {
  components: HashMap<Entity, T>,
}

impl<T: Component> ComponentStore<T> {
  pub fn new() -> Self {
    ComponentStore {
      components: HashMap::new(),
    }
  }

  pub fn add(&mut self, entity: Entity, component: T) {
    self.components.insert(entity, component);
  }

  pub fn get(&self, entity: Entity) -> Option<&T> {
    self.components.get(&entity)
  }
}

// In a pure ECS system, components only hold data
pub struct Position(pub cgmath::Point3<f32>);
impl Component for Position {}

pub struct Rotation(pub cgmath::Quaternion<f32>);
impl Component for Rotation {}

pub struct Velocity(pub f32);
impl Component for Velocity {}

pub struct Model {
  mesh: Mesh,
  rig: Option<Rig>
}
impl Component for Model {}

/// Entity

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Entity(u64);
impl Entity {
  pub fn new(id: u64) -> Entity {
    Entity(id)
  }
}

/// System

pub trait System {
  fn update(&mut self, ecs: &mut ECSManager, delta_time: f32);
}

pub struct MovementSystem;

impl System for MovementSystem {
  fn update(&mut self, ecs: &mut ECSManager, delta_time: f32) {
    // Query entities with Position, Rotation and Velocity
    let positions = match ecs
      .components
      .get_mut(&std::any::type_name::<Position>().to_string())
      .and_then(|store| store.downcast_mut::<ComponentStore<Position>>())
    {
      Some(pos) => pos,
      None => {
        // Debugger
        panic!("Position components not registered!");
      }
    };

    let velocities = match ecs
      .components
      .get_mut(&std::any::type_name::<Velocity>().to_string())
      .and_then(|store| store.downcast_mut::<ComponentStore<Velocity>>())
    {
      Some(velo) => velo,
      None => {
        panic!("Velocity components not registered!");
      }
    };

    let rotations = match ecs
      .components
      .get_mut(&std::any::type_name::<Rotation>().to_string())
      .and_then(|store| store.downcast_mut::<ComponentStore<Rotation>>())
    {
      Some(rot) => rot,
      None => {
        panic!("Velocity components not registered!");
      }
    };

    for (entity, velocity, rotations) in velocities.components.iter() {
      if let Some(position) = positions.components.get_mut(entity) {
        position.0 += velocity.0 * delta_time; // Update position
      }
    }
  }
}

pub struct RenderSystem;

impl System for RenderSystem {
  
}

pub struct SystemManager {
  systems: Vec<Box<dyn System>>
}

impl SystemManager {
  pub fn new() -> SystemManager {
    SystemManager {
      systems: Vec::new()
    }
  }

  pub fn add_system<S: System + 'static>(&mut self, system: S) {
    self.systems.push(Box::new(system));
  }

  pub fn run(&mut self, ecs: &mut ECSManager, delta_time: f32) {
    for system in self.systems.iter_mut() {
      system.update(ecs, delta_time);
    }
  }
}

/// Entity Component System Manager
pub struct ECSManager {
  next_entity_id: u64,
  entities: Vec<Entity>,
  components: HashMap<String, Box<dyn AnyComponentStore>>
  systems: SystemManager
}

impl ECSManager {
  pub fn new() -> Self {
    ECSManager {
      next_entity_id: 0,
      entities: Vec::new(),
      components: HashMap::new(),
      systems: SystemManager::new()
    }
  }

  /// Create a new entity
  pub fn create_entity(&mut self) -> Entity {
    let entity = Entity::new(self.next_entity_id);
    self.next_entity_id += 1;
    self.entities.push(entity);
    entity
  }

  /// Register a component type
  pub fn register_component<T: Component>(&mut self) {
    let type_name = std::any::type_name::<T>().to_string();
    self.components.insert(type_name, Box::new(ComponentStore::<T>::new()));
  }

  /// Add a component to an entity
  pub fn add_component<T: Component>(&mut self, entity: Entity, component: T) {
    if let Some(store) = self.components.get_mut(&std::any::type_name::<T>().to_string()) {
      let store = store.downcast_mut::<ComponentStore<T>>().unwrap();
      store.add(entity, component);
    }
  }

  /// Get a component of an entity
  pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
    self.components
      .get(&std::any::type_name::<T>().to_string())?
      .downcast_ref::<ComponentStore<T>>()?
      .get(entity)
  }

  pub fn add_system<S: System + 'static>(&mut self, system: S) {
    self.systems.add_system(system);
  }

  pub fn run_systems(&mut self, delta_time: f32) {
    self.systems.run(self, delta_time);
  }
}
3