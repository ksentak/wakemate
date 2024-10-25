use std::sync::{Arc, Mutex}; // Used for atomic flag and thread-safe handling
use std::thread; // For spawning a background thread
use std::time::Duration; // To define sleep durations for the thread
use std::sync::atomic::{AtomicBool, Ordering}; // Atomic flag for thread state

/// Struct to manage sleep prevention status and the background thread
#[derive(Default)]
struct SleepPreventer {
    active: Arc<AtomicBool>, // Atomic flag indicating if sleep prevention is active
    thread_handle: Arc<Mutex<Option<thread::JoinHandle<()>>>>, // Handle for managing the background thread
}

impl SleepPreventer {
    /// Creates a new instance of SleepPreventer
    fn new() -> Self {
        SleepPreventer {
            active: Arc::new(AtomicBool::new(false)), // Initially set as inactive
            thread_handle: Arc::new(Mutex::new(None)), // No active thread on creation
        }
    }

    /// Starts the sleep prevention mechanism
    fn prevent_sleep(&self) {
        // Check if sleep prevention is already active
        if self.active.load(Ordering::SeqCst) {
            // Uncomment for debugging
            println!("Sleep prevention already active.");
            return;
        }

        // Set the active flag to true, indicating sleep prevention should start
        self.active.store(true, Ordering::SeqCst);
        let active_clone = Arc::clone(&self.active); // Clone the flag for the thread

        // Spawn a new thread to maintain activity and prevent sleep
        let handle = thread::spawn(move || {
            #[cfg(target_os = "windows")]
            {
                // Import Windows API for preventing sleep
                use winapi::um::winbase::{SetThreadExecutionState, ES_CONTINUOUS, ES_SYSTEM_REQUIRED};
                
                // Set the execution state once at the start to prevent sleep
                unsafe { SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED); }
                
                // Loop while active to maintain the execution state
                while active_clone.load(Ordering::SeqCst) {
                    thread::sleep(Duration::from_secs(1)); // Sleep briefly to prevent high CPU usage
                }
                
                // Reset the execution state when deactivating sleep prevention
                unsafe { SetThreadExecutionState(ES_CONTINUOUS); }
            }

            #[cfg(not(target_os = "windows"))]
            {
                // For macOS/Linux, simulate activity by periodic operations to prevent sleep
                while active_clone.load(Ordering::SeqCst) {
                    // Uncomment for debugging
                    println!("Preventing sleep...");
                    
                    // Sleep for 30 seconds, a typical interval to simulate activity
                    thread::sleep(Duration::from_secs(30)); 
                }
            }
        });

        // Store the handle to the thread in a Mutex, allowing it to be stopped later
        *self.thread_handle.lock().unwrap() = Some(handle);
    }

    /// Stops the sleep prevention mechanism, allowing the system to sleep again
    fn allow_sleep(&self) {
        // Set the active flag to false, signaling the thread to stop
        self.active.store(false, Ordering::SeqCst);

        // Check if there is an active thread and join it (stop it gracefully)
        if let Some(handle) = self.thread_handle.lock().unwrap().take() {
            // Spawn a new thread to handle joining to avoid blocking the main thread
            thread::spawn(move || {
                if let Err(e) = handle.join() {
                    // Log any error if joining the thread fails
                    eprintln!("Error joining sleep prevention thread: {:?}", e);
                }
            });
        }

        // Uncomment for debugging
        println!("Sleep prevention deactivated.");
    }
}

#[tauri::command]
fn prevent_sleep_command(sleep_preventer: tauri::State<'_, SleepPreventer>) {
    // Start sleep prevention by calling prevent_sleep on the SleepPreventer instance
    sleep_preventer.prevent_sleep();
}

#[tauri::command]
fn allow_sleep_command(sleep_preventer: tauri::State<'_, SleepPreventer>) {
    // Stop sleep prevention by calling allow_sleep on the SleepPreventer instance
    sleep_preventer.allow_sleep();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize the Tauri app, providing the SleepPreventer as managed state
    tauri::Builder::default()
        .manage(SleepPreventer::new()) // Pass SleepPreventer instance as a shared state
        .invoke_handler(tauri::generate_handler![prevent_sleep_command, allow_sleep_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
