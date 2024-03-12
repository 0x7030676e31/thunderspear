use std::collections::VecDeque;
use std::sync::OnceLock;
use std::{fs, env};

use strsim::normalized_damerau_levenshtein as distance;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;

#[derive(Serialize, Deserialize)]
pub struct File {
  id: u32,
  name: String,
  path: String,
  size: u64,
  created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct QueuedFile {
  id: u32,
  path: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct State {
  pub token: Option<String>,
  pub channel: Option<String>,
  pub next_id: u32,
  pub root: Vec<File>,
  #[serde(skip)]
  pub queue: VecDeque<QueuedFile>,
  #[serde(skip)]
  pub uploading: Option<u32>,
  #[serde(skip)]
  abort_controller: Option<oneshot::Sender<()>>,
}

fn path() -> &'static str {
  static PATH: OnceLock<String> = OnceLock::new();
  PATH.get_or_init(|| {
    match env::consts::OS {
      "linux" => format!("{}/.thunderspear", env::var("HOME").unwrap()),
      "windows" => format!("{}\\thunderspear.json", env::var("APPDATA").unwrap()),
      _ => panic!("Unsupported OS"),
    }
  })
}

impl State {
  pub fn new() -> Self {
    let path = path();
    if fs::metadata(path).is_ok() {
      let data = fs::read_to_string(path).unwrap();
      serde_json::from_str(&data).unwrap()
    } else {
      Self::default()
    }
  }

  pub fn write(&self) {
    let path = path();
    let data = serde_json::to_string(&self).unwrap();
    fs::write(path, data).unwrap();
  }

  pub fn next_id(&mut self) -> u32 {
    let id = self.next_id;
    self.next_id += 1;
    id
  }

  pub fn upload(&mut self, files: Vec<String>) -> String {
    let files = files.into_iter().filter_map(|path| {
      let meta = fs::metadata(&path);
      if meta.is_err() || !meta.unwrap().is_file() {
        return None;
      }
      
      let id = self.next_id();
      Some(QueuedFile { id, path })
    }).collect::<Vec<_>>();

    log::info!("{} files have been queued for upload...", files.len());
    let stringified = serde_json::to_string(&files).unwrap();
    if self.uploading.is_none() {
      self.upload_next();
    }

    stringified
  }

  fn upload_next(&mut self) {
    todo!();
  }

  pub fn download(&self, ids: Vec<u32>, target: String) {
    todo!();
  }

  pub fn delete(&mut self, ids: Vec<u32>) {
    log::info!("{} files have been queued for deletion...", ids.len());
    
    self.root.retain(|file| !ids.contains(&file.id));
    self.queue.retain(|file| !ids.contains(&file.id));

    if let Some(id) = self.uploading && ids.contains(&id) {
      self.abort_controller.take().unwrap().send(()).unwrap();
      self.uploading = None;
    }

    self.write();
  }

  pub fn rename(&mut self, id: u32, name: String) {
    log::info!("Renaming file with id {} to {}...", id, name);
    for file in &mut self.root {
      if file.id == id {
        file.name = name;
        break;
      }
    }

    self.write();
  }

  pub fn query(&self, query: String) -> Vec<u32> {
    let mut root = self.root.iter().filter_map(|file| {
      let dist = distance(&file.name, &query);
      if dist < 0.5 {
        Some((file.id, dist))
      } else {
        None
      }
    }).collect::<Vec<_>>();

    let mut queue = self.queue.iter().filter_map(|file| {
      let dist = distance(&file.path, &query);
      if dist < 0.5 {
        Some((file.id, dist))
      } else {
        None
      }
    }).collect::<Vec<_>>();

    root.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    queue.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    
    let root = root.into_iter().map(|(id, _)| id);
    let queue = queue.into_iter().map(|(id, _)| id);

    root.chain(queue).collect()
  }
}