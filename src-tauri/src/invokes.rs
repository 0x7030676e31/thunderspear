use crate::AppState;

use tauri::State;

#[tauri::command]
pub async fn get_files(state: State<'_, AppState>) -> Result<String, ()> {
  let state = state.lock().await;
  log::info!("Fetching {} files...", state.root.len());

  Ok(serde_json::to_string(&state.root).unwrap())
}

#[tauri::command]
pub async fn upload_files(state: State<'_, AppState>, files: Vec<String>) -> Result<String, ()> {
  let mut state = state.lock().await;
  Ok(state.upload(files))
}

#[tauri::command]
pub async fn download_files(state: State<'_, AppState>, files: Vec<u32>, target: String) -> Result<(), ()> {
  let state = state.lock().await;  
  state.download(files, target);
  Ok(())
}

#[tauri::command]
pub async fn delete_files(state: State<'_, AppState>, files: Vec<u32>) -> Result<(), ()> {
  let mut state = state.lock().await;
  state.delete(files);
  Ok(())
}

#[tauri::command]
pub async fn rename_file(state: State<'_, AppState>, file: u32, name: String) -> Result<(), ()> {
  let mut state = state.lock().await;
  state.rename(file, name);
  Ok(())
}

#[tauri::command]
pub async fn query_files(state: State<'_, AppState>, query: String) -> Result<Vec<u32>, ()> {
  let state = state.lock().await;
  Ok(state.query(query))
}
