mod scripts {
    pub mod script;
    pub mod player;
}

use core::*;
use scripts::*;

fn main() {
    let mut engine: PhoenixEngine = core::PhoenixEngine::new();

    // Load stuff, create entities, prepare everything

    let mut x: u32 = 0;

    engine.run(|| {
        script::update(x);
        player::update();
        x += 1;
    });
}
