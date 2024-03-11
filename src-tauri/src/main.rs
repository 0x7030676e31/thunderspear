#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(const_trait_impl, effects, let_chains)]

use std::sync::Arc;
use std::env;

use tokio::sync::Mutex;

mod invokes;
mod state;
mod reader;
mod actions;
mod strsim;

type AppState = Arc<Mutex<state::State>>;

fn main() {
  if env::var("RUST_LOG").is_err() {
    env::set_var("RUST_LOG", "info");
  }

  pretty_env_logger::init();
  log::info!("Starting Thunderstorm Desktop v{}", env!("CARGO_PKG_VERSION"));

  let state = state::State::new();
  let state = Arc::new(Mutex::new(state));

  tauri::Builder::default()
    .manage(state)
    .invoke_handler(tauri::generate_handler![
      invokes::get_files,
      invokes::upload_files,
      invokes::download_files,
      invokes::delete_files,
      invokes::rename_file,
      invokes::query_files,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
