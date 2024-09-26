// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use core::*;
use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    // hello_from_core();
    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.run()?;

    Ok(())
}

/*

UI Multithread example:

use slint::prelude::*;
use std::sync::mpsc;
use std::thread;

fn main() {
    // Initialize your UI
    let ui = slint::include_slint!
    ("your_ui.slint");
    let window = ui.window();

    // Channel for communication
    let (tx, rx) = mpsc::channel();

    // Start a separate thread for Slint
    let ui_thread = thread::spawn(move || {
        loop {
            // Process Slint events
            slint::platform::process_events();

            // Check for messages to update the UI
            if let Ok(value) = rx.try_recv() {
                ui.set_property(value); // Update property based on game state
            }

            // Request a redraw
            window.request_redraw();
        }
    });

    // Main game loop
    loop {
        // Simulate game logic updates
        let new_value = 42; // Replace this with your game logic
        tx.send(new_value).unwrap(); // Send data to UI thread

        // Handle other game logic and rendering here

        // Sleep or yield to avoid busy waiting
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    // Join the UI thread when done (in practice, you'd handle cleanup)
    ui_thread.join().unwrap();
}

*/
