#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod ansi;

use tauri::Manager;
use std::io::Write;
use std::ffi::CString;
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
    writer: Arc<Mutex<dyn Write + Send>>,
}

#[tauri::command]
fn on_window_open(interface: tauri::State<PtyInterface>) -> CmdResult<()> {
    let pair = interface.pair.lock().unwrap();
    let mut cmd = CommandBuilder::new("bash");
    cmd.env("TERM", "xterm-256color");
    pair.slave.spawn_command(cmd)?;
    Ok(())
}

#[tauri::command]
fn send_command(command: &str, interface: tauri::State<PtyInterface>) -> CmdResult<()> {
    let mut writer = interface.writer.lock().unwrap();
    writeln!(writer, "{}\n", command).unwrap();
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
    let mut reader = pair.master.try_clone_reader()?;
    let writer = pair.master.take_writer()?;

    tauri::Builder::default()
        .manage(PtyInterface {
            pair: Arc::new(Mutex::new(pair)),
            writer: Arc::new(Mutex::new(writer)),
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
                        let len = s.len();
                        
                        let csa: CString = CString::new(s).unwrap();
                        let cv: Vec<u8> = csa.into_bytes_with_nul();
                        let mut tmp: Vec<i8> = cv.into_iter().map(|c| c as i8).collect::<_>(); // line 7
                        let _cptr: *mut i8 = tmp.as_mut_ptr();
                        
                        let converted: *mut libc::c_char = unsafe { ansi::convert_ansi_to_html(_cptr, len as u32) };
                        let csb = unsafe { CString::from_raw(converted) };

                        let _ = app_handle.emit_all("output", Some(csb.to_str().unwrap()));
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![send_command, on_window_open])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
