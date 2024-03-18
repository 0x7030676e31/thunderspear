use crate::actions::{preupload, upload, send_message};
use crate::reader::{CLUSTER_SIZE, Reader};
use crate::AppState;

use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::OnceLock;
use std::{fs, env, cmp};

use strsim::normalized_damerau_levenshtein as distance;
use serde::{Deserialize, Serialize};
use tokio::sync::{oneshot, mpsc};
use tauri::{AppHandle, Manager};
use tokio::time;

#[derive(Serialize, Deserialize)]
pub struct File {
  id: u32,
  name: String,
  path: String,
  size: u64,
  clusters: Vec<String>,
  created_at: u64,
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
  pub abort_controller: Option<oneshot::Sender<()>>,
  #[serde(skip)]
  pub this: Option<AppState>,
  #[serde(skip)]
  pub app_handle: Option<AppHandle>,
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
    self.queue.extend(files);

    if self.uploading.is_none() {
      self.upload_next();
    }

    stringified
  }

  fn upload_next(&mut self) {
    let file = self.queue.pop_front().unwrap();
    self.uploading = Some(file.id);
  
    let (tx, abort) = oneshot::channel();
    self.abort_controller = Some(tx);

    let handle = self.app_handle.as_ref().unwrap().clone();
    let channel = self.channel.as_ref().unwrap();
    let token = self.token.as_ref().unwrap();

    let (tx, mut rx) = mpsc::channel::<usize>(16);
    let reader = Reader::new(&file.path, tx);

    let size = reader.size as f64;
    handle.emit_all("uploading", (&file.id, size)).unwrap();
      
    let id = file.id.to_string();
    tokio::spawn(async move {
      let mut uploaded = 0;
      while let Some(read) = rx.recv().await {
        uploaded += read;
        handle.emit_all(&id, (uploaded as f64) / size * 100.0).unwrap();
      }
    });

    let (tx, mut rx) = mpsc::channel::<(String, usize)>(1);
    let handle = tokio::spawn(async move {
      let mut ids = vec![String::new(); reader.clusters];
      while let Some((id, idx)) = rx.recv().await {
        ids[idx] = id;
      }

      ids
    });

    let mut idx = 0;
    while let Some(cluster) = reader.next_cluster(idx) {
      let size = cmp::min(reader.size - idx * CLUSTER_SIZE, CLUSTER_SIZE);

      let auth = token.clone();
      let channel = channel.clone();
      let tx = tx.clone();

      tokio::spawn(async move {
        let details = loop {
          match preupload(&auth, &channel, idx, size).await {
            Ok(details) => break details,
            Err(retry_after) => time::sleep(time::Duration::from_secs_f32(retry_after)).await,
          };
        };

        log::debug!("Uploading cluster {}... ({})", idx, details.len());
        let now = time::Instant::now();
        upload(&auth, &details, cluster).await;
        
        log::debug!("Cluster {} uploaded, took {:.2}s", idx, now.elapsed().as_secs_f64());
        let now = time::Instant::now();

        let resp = loop {
          match send_message(&auth, &channel, &details, idx).await {
            Ok(resp) => break resp,
            Err(retry_after) => time::sleep(time::Duration::from_secs_f32(retry_after)).await,
          };
        };

        log::debug!("Cluster {} sent, took {:.2}s", idx, now.elapsed().as_secs_f64());
        if let Err(err) = tx.send((resp, idx)).await {
          log::error!("Failed to send message: {}", err);
        }
      });

      idx += 1;
    }

    let this = self.this.as_ref().unwrap().clone();
    tokio::spawn(async move {
      let ids = tokio::select! {
        _ = abort => {
          log::info!("Upload aborted...");
          return;
        }
        ids = handle => ids,
      };

      let ids = match ids {
        Ok(ids) => ids,
        Err(err) => {
          log::error!("Failed to upload: {}", err);
          return;
        }
      };

      log::info!("File {} has been uploaded successfully", file.path);
      let mut state = this.write().await;
      let name = file.path.split('/').last().unwrap().to_string();
      state.root.push(File {
        id: file.id,
        name,
        path: file.path,
        size: reader.size as u64,
        clusters: ids,
        created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
      });

      state.write();
      if state.queue.len() > 0 {
        state.upload_next();
        return;
      }

      state.abort_controller = None;
      state.uploading = None;
    });
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