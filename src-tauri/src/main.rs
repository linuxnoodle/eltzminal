#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use std::sync::{Arc, Mutex};
use serde::{ser::Serializer, Serialize};
use portable_pty::{CommandBuilder, PtySize, native_pty_system};

// -- anyhow::Error but with serde serialization
// Only really necessary because portable_pty throws anyhow::Error,
// but tauri requires a serializable error type to not crap it's pants
#[derive(Debug, thiserror::Error)]
pub enum CmdError {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl Serialize for CmdError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CmdError::Anyhow(e) => e.to_string().serialize(serializer),
        }
    }
}

pub type CmdResult<T, E = CmdError> = anyhow::Result<T, E>;

struct PtyInterface {
    pair: Arc<Mutex<portable_pty::PtyPair>>,
}

#[tauri::command]
fn send_command(command: &str, interface: tauri::State<PtyInterface>) -> CmdResult<()> {
    // Write directly to 
    let pair = interface.pair.lock().unwrap();
    writeln!(pair.master.take_writer()?, "{}\n", command).unwrap();
    Ok(())
}

fn main() -> CmdResult<()> {
    let pty_system = native_pty_system();
    let pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    })?;

    // Run shell
    let command = CommandBuilder::new("bash");
    let _ = pair.slave.spawn_command(command)?;

    // Establish reader
    let mut reader = pair.master.try_clone_reader()?;

    tauri::Builder::default()
        .manage(PtyInterface {
            pair: Arc::new(Mutex::new(pair)),
        })
        .setup(|app| {
            let mut buffer = [0; 1024];
            // Clone handle to allow access from within thread
            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                // Continously reads from reader, emitting all output back to JS
                loop {
                    let n = reader.read(&mut buffer).unwrap();
                    if n > 0 {
                        let s = String::from_utf8_lossy(&buffer[..n]).to_string();
                        app_handle.emit_all("output", Some(s)).unwrap();
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![send_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
