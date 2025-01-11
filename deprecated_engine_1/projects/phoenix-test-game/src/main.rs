mod scripts {
    pub mod script;
    pub mod player;
}

use core::*;
use crate::scene::GameObject;
use scripts::*;

fn main() {
    let mut engine: PhoenixEngine = core::PhoenixEngine::new();

    let zelda: GameObject = GameObject::new(""/* File path */);

    // let object: GameObject = GameObject::new();


    let mut x: u32 = 0;

    engine.run(|| {
        // autosave::do_it();
        script::update(x);
        player::update();
        x += 1;
    });
}
