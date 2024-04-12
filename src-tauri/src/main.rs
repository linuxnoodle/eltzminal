#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(extern_types)]

use toml::Table;
use tauri::Manager;
use std::io::Write;
use std::io::BufRead;
use std::fs::OpenOptions;
use std::sync::{Arc, Mutex};
use fuzzy_search::symspell::SymSpell;
use fuzzy_search::distance::levenshtein;
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

#[derive(Serialize, Clone)]
struct TerminalVariables {
    font: String,
    font_size: i64,
    background_color: String,
    foreground_color: String,
}

struct PtyInterface {
    pair: Arc<Mutex<portable_pty::PtyPair>>,
    writer: Arc<Mutex<dyn Write + Send>>,
    vars: Arc<Mutex<TerminalVariables>>,
    history: Arc<Mutex<SymSpell<fn(&str, &str) -> usize>>>,
    history_file: Arc<Mutex<std::fs::File>>,
}

#[tauri::command]
fn on_window_open(interface: tauri::State<PtyInterface>) -> CmdResult<TerminalVariables> {
    let pair = interface.pair.lock().unwrap();
    let mut cmd = CommandBuilder::new("bash");
    cmd.env("TERM", "xterm-256color");
    pair.slave.spawn_command(cmd)?;
    let vars = interface.vars.lock().unwrap();
    Ok(vars.clone())
}

#[tauri::command]
fn send_command(command: &str, interface: tauri::State<PtyInterface>) -> CmdResult<()> {
    let mut writer = interface.writer.lock().unwrap();
    writeln!(writer, "{}\n", command).unwrap();
    // append to history vec
    let mut history = interface.history.lock().unwrap();
    history.insert(command.to_string());
    // append to history file as well for persistence
    let mut history_file = interface.history_file.lock().unwrap();
    writeln!(history_file, "{}", command).unwrap();
    Ok(())
}

#[tauri::command]
fn fuzzy_find_history(query: String, interface: tauri::State<PtyInterface>) -> CmdResult<Vec<String>> {
    let history = interface.history.lock().unwrap();
    let results = history.fuzzy_search(&query);
    println!("{:?}", results);
    Ok(results)
}

fn main() -> CmdResult<()> {
    let mut font: String = "monospace".to_string();
    let mut font_size = 12;
    let mut background_color: String = "#000000".to_string();
    let mut foreground_color: String = "#ffffff".to_string();
    // append to history file

    if !std::path::Path::new(std::env!("HOME")).join(".config/eltzminal/terminal.toml").exists() {
        // create folders
        let _ = std::fs::create_dir_all(std::path::Path::new(std::env!("HOME")).join(".config/eltzminal"));
        let _ = std::fs::write(
            std::path::Path::new(std::env!("HOME")).join(".config/eltzminal/terminal.toml"),
r##"[terminal]
font = "monospace"
font_size = 12
background_color = "#000000"
foreground_color = "#ffffff"
"##,
        );
    } else {
        // read config
        let config_str = std::fs::read_to_string(std::path::Path::new(std::env!("HOME")).join(".config/eltzminal/terminal.toml"));
        let tbl: Table = toml::from_str(&config_str.unwrap()).unwrap();

        font = tbl["terminal"]["font"].as_str().unwrap().to_string();
        font_size = tbl["terminal"]["font_size"].as_integer().unwrap();
        background_color = tbl["terminal"]["background_color"].as_str().unwrap().to_string();
        foreground_color = tbl["terminal"]["foreground_color"].as_str().unwrap().to_string();
    }

    let pty_system = native_pty_system();
    let pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    })?;
    let mut reader = pair.master.try_clone_reader()?;
    let writer = pair.master.take_writer()?;

    // read history file into string
    let mut history: SymSpell<fn(&str, &str) -> usize> = SymSpell::new(levenshtein, 20);
    let history_file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(std::path::Path::new(std::env!("HOME")).join(".config/eltzminal/history")).unwrap();
    for line in std::io::BufReader::new(&history_file).lines() {
        // clone and print line
        let line = line.unwrap();
        println!("{}", line);
        history.insert(line);
    }

    tauri::Builder::default()
        .manage(PtyInterface {
            pair: Arc::new(Mutex::new(pair)),
            writer: Arc::new(Mutex::new(writer)),
            vars: Arc::new(Mutex::new(TerminalVariables {
                font,
                font_size,
                background_color,
                foreground_color,
            })),
            history: Arc::new(Mutex::new(history)),
            history_file: Arc::new(Mutex::new(history_file)),
            
        })
        .setup(|app| {
            let mut buffer = [0; 2048];
            // Clone handle to allow access from within thread
            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                // Continously reads from reader, emitting all output back to JS
                loop {
                    let n = reader.read(&mut buffer).unwrap();
                    if n > 0 {
                        let s = String::from_utf8_lossy(&buffer[..n]).to_string();
                        let output = ansi_to_html::convert(&s).unwrap();
                        let _ = app_handle.emit_all("output", output);
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![send_command, on_window_open, fuzzy_find_history])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
