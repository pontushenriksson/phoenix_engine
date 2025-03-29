use phoenix_core::*;

fn main() {
  let mut app = PhoenixApplication::new(
    800,
    800,
    "Test Game",
    "./assets/icons/icon.png"
  );

  let mgl_handle = app.add::<GameLayer>(/* Some input */);

  mgl_handle.register_input(Phoenix::Event::Press(W));
  
  let ui_handle = app.add::<UiLayer>(/* Some input */);
  
  let scene_handle = mgl_handle.scenes.add::<Scene>(/* Some input */);
  
  let obj = scene_handle.add::<StaticGameObject>(/* Some input like the model path and textures perhaps*/);
  
  obj.register_system(scene_handle.input(Phoenix::Event::Press(X)), func());

  let camera_handle = scene.add::<Camera3D>(/* Some input*/).with_pos(/* Read from file or given */);

  camera_handle.register_system(Phoenix::Event::Press(X), move_cam());
  
  scene_handle.load::<StaticGameObject>(obj).with_pos(/* Read form file or given idk*/);

  scene_handle.on_scene_change(/* Something */).load::<StaticGameObject>(obj).with_pos(/* Read from file or ...*/);

  app.run();
}
