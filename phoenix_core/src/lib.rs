pub mod core;

pub mod debugger {
    pub mod debugger;
}

pub mod graphics {
    pub mod renderer;
    pub mod shaders;
}

pub mod events {
    pub mod events;
}

pub mod assets {
    pub mod loader;
}

fn act() {
    println!("Act!");
}

fn foo() {
    core::bindings::register_action_for_event(core::bindings::KeyAction::KeyPress('w'), || {
        act();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut engine = core::PhoenixEngine::new(800, 800, "Phoenix Engine v0.1.0");
        // let mut ui = PhoenixUi::new(context, renderer from engine);
        
        // map events to functionallity.

        // engine.register_fn_for_event(event::KeyPress::W, script::player_move);
        
        engine.run(|| {
            // foo();
        });

        // engine.terminate();
    }
}
