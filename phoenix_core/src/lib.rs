pub mod core;

pub mod events {
    pub mod events;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut engine = core::PhoenixEngine::new(800, 800, "Phoenix Engine v0.1.0").expect("Failed to initialize engine!");
        // let mut ui = PhoenixUi::new(context, renderer from engine);
        engine.run(|| {
            
        });
    }
}
