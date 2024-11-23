use phoenix_core::*;

pub struct Player {
  // ...
}

impl Player {
  fn new() {
    // ...
  }
}

impl PlayerCharacter for Player {
  fn move() {

  }

  // ...
}

pub struct Enemy {
  // ...
}

impl Enemy {
  fn new() {
    // ...
  }
}

impl Entity for Enemy {
  fn move() {

  }

  // ...
}

fn func() {
  println!("W pressed!");
}

fn main() {
  let mut app = PhoenixApplication::new(
    800,
    800,
    "Test Game",
    "./assets/icons/icon.png"
  );

  let game = GameLayer::new();
  let scene1 = Scene::new();
  let player = Player::new("player.phnx");

  /*
  
  mesh: "C/sdas"
  script: "cxasc"
  textures {
    "sdasd",
    "a2asadas"
  }

  */

  let enemy = Enemy::new("path/to/directory/with/model/scripts/etc.");

  // Should I bind inputs to the player or to the layer? Im think layer but what do you think?
  enemy.add_system(phoenix_astar_path_finding(player)); // Where player is the target i think

  scene1.spawn_player(player);
  scene1.spawn_entity(enemy);
  game.scenes.push_scene(scene1);
  app.push_layer(game);

  app.run();
}
