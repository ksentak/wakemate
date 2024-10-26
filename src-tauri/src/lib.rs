use keepawake::{Builder, KeepAwake};
use std::sync::{Arc, Mutex};
use tauri::State;

// Define the `SleepGuard` struct to manage sleep prevention state.
struct SleepGuard {
    awake_guard: Option<KeepAwake>, // Optional guard that, when `Some`, prevents sleep.
}

impl SleepGuard {
    // Constructor for `SleepGuard`, initializing it with no active guard (None).
    fn new() -> Self {
        Self { awake_guard: None }
    }

    // Activate sleep prevention if not already active.
    fn prevent_sleep(&mut self) -> Result<(), String> {
        if self.awake_guard.is_none() { // Check if there's no active guard.
            let guard = Builder::default() // Create a new sleep prevention guard with specific options.
                .display(true) // Prevent the display from sleeping.
                .idle(true) // Prevent the system from sleeping due to inactivity.
                .sleep(true) // Prevent the system from sleeping (OS-specific).
                .reason("User requested") // Reason for preventing sleep, for display on macOS/Linux.
                .app_name("wakeMate") // Application name (shown on Linux).
                .app_reverse_domain("com.example.wakemate") // Reverse domain name (used on Linux).
                .create() // Creates the `KeepAwake` instance based on options.
                .map_err(|e| e.to_string())?; // Convert any errors to `String` for Result handling.

            self.awake_guard = Some(guard); // Store the active guard, now preventing sleep.
            println!("Sleep prevention activated.");
        } else {
            println!("Sleep prevention is already active.");
        }
        Ok(()) // Return `Ok` if activation was successful.
    }

    // Deactivate sleep prevention by dropping the guard.
    fn allow_sleep(&mut self) {
        if self.awake_guard.is_some() { // Check if there's an active guard.
            self.awake_guard = None; // Set the guard to `None`, dropping it and allowing sleep.
            println!("Sleep prevention deactivated.");
        } else {
            println!("Sleep prevention is already inactive.");
        }
    }
}

#[tauri::command]
fn prevent_sleep(sleep_guard: State<Arc<Mutex<SleepGuard>>>) -> Result<(), String> {
    // Lock the `Mutex` to access `SleepGuard` (for safe concurrent access).
    let mut guard = sleep_guard.lock().map_err(|_| "Failed to acquire lock".to_string())?;
    guard.prevent_sleep()?; // Call `prevent_sleep` to activate sleep prevention.
    Ok(())
}

#[tauri::command]
fn allow_sleep(sleep_guard: State<Arc<Mutex<SleepGuard>>>) {
    let mut guard = sleep_guard.lock().expect("Failed to acquire lock"); // Lock and access `SleepGuard`.
    guard.allow_sleep(); // Call `allow_sleep` to deactivate sleep prevention.
}

// The main entry point for the Tauri app.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init()) 
        .manage(Arc::new(Mutex::new(SleepGuard::new()))) // Share `SleepGuard` state across commands.
        .invoke_handler(tauri::generate_handler![prevent_sleep, allow_sleep])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
