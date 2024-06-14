use std::fmt::Display;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use tauri::{AppHandle, Manager};

/// Structure représentant un thread avec un identifiant, un statut, une fréquence et un handle.
pub struct TreadObject {
    id: u32,
    status: Arc<Mutex<bool>>,
    rate: u32,
    handle: Option<thread::JoinHandle<()>>,
}

impl TreadObject {
    /// Crée un nouveau `TreadObject` avec l'identifiant et la fréquence spécifiés.
    ///
    /// # Arguments
    ///
    /// * `id` - L'identifiant du thread.
    /// * `rate` - La fréquence à laquelle le thread incrémente le compteur (en secondes).
    ///
    /// # Returns
    ///
    /// Retourne une nouvelle instance de `TreadObject`.
    pub fn new(id: u32, rate: u32, app: AppHandle) -> Self {
        let status = Arc::new(Mutex::new(true));
        let status_clone = Arc::clone(&status);

        let handle = thread::spawn(move || {
            let mut counter = 0;
            loop {
                {
                    let status = status_clone.lock().unwrap();
                    if !*status {
                        println!("Thread {}: Stopped", id);
                        break;
                    }
                }
                counter += 1;
                println!("Thread {}: counter: {}", id, counter);
                match app.emit(&format!("thread-{}", id), counter) {
                    Ok(_) => println!("Thread {}: counter: {}", id, counter),
                    Err(e) => println!("Failed to emit event for thread {}: {}", id, e),
                }
                thread::sleep(Duration::from_secs(rate as u64));
            }
        });

        Self {
            id,
            status,
            rate,
            handle: Some(handle),
        }
    }

    /// Arrête le thread en mettant à jour le statut et en rejoignant le handle du thread.
    pub fn stop(&mut self) {
        {
            let mut status = self.status.lock().unwrap();
            *status = false;
            println!("Thread {}: status set to false", self.id);
        }

        if let Some(handle) = self.handle.take() {
            println!("Thread {}: joining handle", self.id);
            if handle.join().is_ok() {
                println!("Thread {}: handle joined", self.id);
            } else {
                println!("Thread {}: failed to join handle", self.id);
            }
        }
    }
}

impl Display for TreadObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Thread {}: rate: {}, handle: {}, status: {}",
            self.id,
            self.rate,
            self.handle.is_some(),
            *self.status.lock().unwrap()
        )
    }
}